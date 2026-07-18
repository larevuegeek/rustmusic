//! Sortie DoP (DSD over PCM) bit-perfect via ALSA `hw:` — Linux uniquement.
//!
//! # Pourquoi ALSA direct et pas CPAL/PipeWire
//! Le DoP n'a pas de demi-mesure : soit chaque octet arrive **intact** au DAC
//! (qui reconnaît les marqueurs `0x05/0xFA` et rejoue du DSD natif), soit un
//! seul bit altéré détruit les marqueurs → bruit blanc très fort. PipeWire/Pulse
//! rééchantillonnent et appliquent le volume → destructeurs. On ouvre donc le
//! périphérique ALSA `hw:` en direct (accès exclusif, bypass du serveur son),
//! équivalent Linux du WASAPI exclusive de Windows.
//!
//! # Format
//! `S32_LE`, `carrier_rate` (= dsd_rate / 16), 2 canaux, **sans resampling**.
//! L'encodeur (`dop_encoder`) produit des `i32` `[0][hi][lo][0]` ; le marqueur
//! (`0x05/0xFA`, alterné à chaque trame, compteur continu) est posé ici sur
//! CHAQUE trame — musique comme silence — sinon le DAC perd le lock DSD.
//!
//! # v1 : per-track
//! Un stream par piste (pas encore de moteur gapless persistant). Le DAC DSD
//! se re-verrouille entre les morceaux (~1-2 s). Le moteur persistant viendra
//! dans un second temps (parité avec `dop_engine.rs` de Windows).

#![cfg(target_os = "linux")]

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use alsa::pcm::{Access, Format, HwParams, PCM};
use alsa::{Direction, ValueOr};

use crate::core::audio_decoder::dsd::dop_encoder::{
    dop_silence_payload, stamp_marker, DopEncoder,
};
use crate::core::audio_decoder::dsd::dsd_container::DsdContainerReader;

/// Un périphérique de sortie ALSA hardware candidat au DoP.
#[derive(Debug, Clone)]
pub struct AlsaDopDevice {
    /// Identifiant ALSA ouvrable, ex. `hw:2,0`.
    pub hw_id: String,
    /// Nom lisible (ex. "Fosi Audio K7"), pour le matching avec le nom UI.
    pub display: String,
}

/// Énumère les cartes ALSA matérielles (device 0 de chaque carte).
///
/// On cible `hw:{index},0` (non ambigu, contrairement à `hw:CARD=<id>`). La
/// plupart des DAC exposent leur sortie principale sur le device 0.
pub fn list_dop_devices() -> Vec<AlsaDopDevice> {
    let mut out = Vec::new();
    for card in alsa::card::Iter::new().flatten() {
        let idx = card.get_index();
        let display = card.get_name().unwrap_or_else(|_| format!("Carte {idx}"));
        out.push(AlsaDopDevice {
            hw_id: format!("hw:{idx},0"),
            display,
        });
    }
    out
}

/// Mappe le nom d'affichage sélectionné dans l'UI (ex. "Fosi Audio K7 (…)")
/// vers l'identifiant ALSA `hw:` correspondant, par sous-chaîne insensible à
/// la casse dans les deux sens. `None` si aucune carte hardware ne correspond.
pub fn resolve_hw_id(selected_display: &str) -> Option<String> {
    let sel = selected_display.to_lowercase();
    for dev in list_dop_devices() {
        let name = dev.display.to_lowercase();
        if name.is_empty() {
            continue;
        }
        if sel.contains(&name) || name.contains(&sel) {
            return Some(dev.hw_id);
        }
    }
    None
}

/// Ouvre le PCM `hw:` en réessayant sur EBUSY (errno 16). Après une réservation
/// D-Bus, PipeWire ferme le device de façon **asynchrone** : le `hw:` peut rester
/// occupé ~centaines de ms. On réessaie jusqu'à ~2,5 s avant d'abandonner.
fn open_hw_pcm(hw_id: &str) -> Result<PCM, String> {
    let mut last = String::new();
    for _ in 0..25 {
        match PCM::new(hw_id, Direction::Playback, false) {
            Ok(pcm) => return Ok(pcm),
            Err(e) => {
                let busy = e.errno() == 16 || e.errno() == -16; // EBUSY
                last = e.to_string();
                if busy {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    continue;
                }
                return Err(last);
            }
        }
    }
    Err(format!("device toujours occupé après attente : {last}"))
}

/// Configure des `HwParams` DoP standard sur un PCM (S32_LE / carrier / 2ch,
/// resampling désactivé). Factorisé entre le probe et le render.
fn configure_dop_params(pcm: &PCM, carrier_rate: u32, channels: u16) -> Result<(), String> {
    let hwp = HwParams::any(pcm).map_err(|e| format!("hw_params any: {e}"))?;
    hwp.set_access(Access::RWInterleaved)
        .map_err(|e| format!("set_access: {e}"))?;
    hwp.set_format(Format::S32LE)
        .map_err(|e| format!("set_format S32_LE: {e}"))?;
    hwp.set_channels(channels as u32)
        .map_err(|e| format!("set_channels: {e}"))?;
    // CRUCIAL : pas de rééchantillonnage, sinon le DoP est détruit.
    hwp.set_rate_resample(false)
        .map_err(|e| format!("set_rate_resample(false): {e}"))?;
    hwp.set_rate(carrier_rate, ValueOr::Nearest)
        .map_err(|e| format!("set_rate {carrier_rate}: {e}"))?;
    // Latence raisonnable : ~80 ms de buffer, périodes ~20 ms.
    let _ = hwp.set_buffer_time_near(80_000, ValueOr::Nearest);
    let _ = hwp.set_period_time_near(20_000, ValueOr::Nearest);
    pcm.hw_params(&hwp).map_err(|e| format!("apply hw_params: {e}"))?;

    // Vérifier que le rate obtenu est EXACTEMENT le carrier (sinon resampling
    // implicite ou format non supporté → DoP invalide).
    let got = pcm
        .hw_params_current()
        .and_then(|h| h.get_rate())
        .map_err(|e| format!("get_rate: {e}"))?;
    if got != carrier_rate {
        return Err(format!("rate obtenu {got} ≠ carrier {carrier_rate}"));
    }
    Ok(())
}

/// Le DAC accepte-t-il le format DoP au carrier donné, en accès exclusif ?
///
/// Ouvre le `hw:` (échoue si occupé par PipeWire/Pulse — dans ce cas pas de
/// bit-perfect possible → `false`), tente la négociation, referme.
pub fn dop_format_supported(hw_id: &str, carrier_rate: u32, channels: u16) -> bool {
    match open_hw_pcm(hw_id) {
        Ok(pcm) => match configure_dop_params(&pcm, carrier_rate, channels) {
            Ok(()) => true,
            Err(e) => {
                log::debug!("🎚️  DoP ALSA format refusé sur {hw_id} @ {carrier_rate} Hz : {e}");
                false
            }
        },
        Err(e) => {
            log::debug!("🎚️  Ouverture exclusive {hw_id} impossible : {e}");
            false
        }
    }
}

/// Pilotage du render DoP par le thread de lecture.
pub struct AlsaDopControl {
    /// Pause globale : on écrit du silence DSD (DAC reste locké).
    pub is_paused: Arc<AtomicBool>,
    /// Arrêt demandé : le render sort dès que possible.
    pub is_stopped: Arc<AtomicBool>,
    /// Position courante en secondes (bits f64). Mise à jour par le render.
    pub current_position: Arc<AtomicU64>,
    /// Seek demandé en secondes (bits f64) ; `u64::MAX` = aucun.
    pub seek_position: Arc<AtomicU64>,
    /// Durée totale en secondes (bits f64), pour clamper le seek.
    pub total_duration: Arc<AtomicU64>,
}

/// Nombre de trames de silence écrites par cycle en pause (~21 ms @ 176.4k).
const SILENCE_FRAMES: usize = 4096;

/// Ouvre le stream ALSA et joue la piste DSD en DoP jusqu'à EOF ou stop.
///
/// Boucle single-thread : décode un super-bloc DSD → encode DoP → tamponne les
/// marqueurs → `writei` **bloquant** (fournit le cadencement temps réel). En
/// pause on écrit du silence DoP pour garder le DAC verrouillé.
///
/// Retour : `Ok(true)` = fin naturelle (EOF), `Ok(false)` = stop utilisateur.
pub fn run_alsa_dop_playback(
    hw_id: &str,
    carrier_rate: u32,
    channels: u16,
    mut decoder: Box<dyn DsdContainerReader + Send>,
    lsb_first: bool,
    ctl: AlsaDopControl,
) -> Result<bool, String> {
    let ch = channels as usize;

    let pcm = open_hw_pcm(hw_id).map_err(|e| format!("ouverture {hw_id}: {e}"))?;
    configure_dop_params(&pcm, carrier_rate, channels)?;
    pcm.prepare().map_err(|e| format!("prepare: {e}"))?;

    log::info!("🎚️  Stream ALSA DoP ouvert : {hw_id} @ {carrier_rate} Hz / {channels} ch (S32_LE, bit-perfect)");

    let mut encoder = DopEncoder::new(channels as u8, lsb_first);
    // Marqueur DoP : alterne à CHAQUE trame, compteur continu (musique + silence).
    let mut marker_b = false;
    // Frames de musique jouées (position). Pas incrémenté en pause.
    let mut frames_played: u64 = 0;

    // Tampon de silence DoP pré-généré (payload idle 0x69), marqueurs posés au vol.
    let silence_payload = dop_silence_payload();

    // Écrit `buf` (interleavé, déjà marqué) en gérant les XRUN.
    let write_all = |pcm: &PCM, buf: &[i32]| -> Result<(), String> {
        let io = pcm.io_i32().map_err(|e| format!("io_i32: {e}"))?;
        let mut off = 0usize;
        while off < buf.len() {
            match io.writei(&buf[off..]) {
                Ok(frames) => off += frames * ch,
                Err(e) => {
                    // Underrun / suspend → tenter de récupérer puis réécrire.
                    if pcm.recover(e.errno(), true).is_err() {
                        return Err(format!("writei: {e}"));
                    }
                }
            }
        }
        Ok(())
    };

    'track: loop {
        if ctl.is_stopped.load(Ordering::Relaxed) {
            return Ok(false);
        }

        // ─── Seek ───
        let seek_bits = ctl.seek_position.load(Ordering::Relaxed);
        if seek_bits != u64::MAX {
            ctl.seek_position.store(u64::MAX, Ordering::Relaxed);
            let total = f64::from_bits(ctl.total_duration.load(Ordering::Relaxed)).max(0.0);
            let secs = f64::from_bits(seek_bits).clamp(0.0, total);
            if let Ok(actual) = decoder.seek_to_seconds(secs) {
                encoder.reset();
                frames_played = (actual * carrier_rate as f64) as u64;
                ctl.current_position.store(actual.to_bits(), Ordering::Relaxed);
                let _ = pcm.prepare(); // repart proprement après le drain implicite
            }
            continue;
        }

        // ─── Pause : silence DoP pour garder le lock DSD ───
        if ctl.is_paused.load(Ordering::Relaxed) {
            let mut buf = Vec::with_capacity(SILENCE_FRAMES * ch);
            for _ in 0..SILENCE_FRAMES {
                let s = stamp_marker(silence_payload, marker_b);
                for _ in 0..ch {
                    buf.push(s);
                }
                marker_b = !marker_b;
            }
            write_all(&pcm, &buf)?;
            continue;
        }

        // ─── Décode + encode + joue ───
        let blocks = match decoder.read_next_blocks() {
            Ok(Some(b)) => b,
            Ok(None) => break 'track, // EOF
            Err(e) => return Err(format!("décodage DSD: {e:?}")),
        };
        let payload = encoder.encode_blocks(&blocks); // interleavé, sans marqueur
        if payload.is_empty() {
            continue;
        }
        let frames = payload.len() / ch;

        // Poser le marqueur DoP trame par trame (même marqueur pour tous les
        // canaux d'une trame, alternance à chaque trame).
        let mut buf = Vec::with_capacity(payload.len());
        for f in 0..frames {
            let base = f * ch;
            for c in 0..ch {
                buf.push(stamp_marker(payload[base + c], marker_b));
            }
            marker_b = !marker_b;
        }

        write_all(&pcm, &buf)?;

        frames_played += frames as u64;
        let pos = frames_played as f64 / carrier_rate as f64;
        ctl.current_position.store(pos.to_bits(), Ordering::Relaxed);
    }

    // Fin naturelle : vider le buffer ALSA (laisse jouer la fin).
    let _ = pcm.drain();
    Ok(true)
}
