//! Backend WASAPI exclusive (Windows uniquement).
//!
//! Délègue le pipeline bas-niveau à `audio_output_wasapi::run_wasapi_playback`
//! qui tourne dans un thread dédié. Cette wrapper struct gère le cycle de
//! vie (thread spawn + JoinHandle), expose le trait `AudioOutput` et signale
//! `is_stopped` au Drop pour shutdown propre.

#![cfg(target_os = "windows")]

use std::sync::atomic::Ordering;
use std::thread::JoinHandle;

use ringbuf::traits::Consumer;

use super::traits::{AudioOutput, AudioOutputError};
use super::types::{AudioBackend, PlaybackAtomics, SymphoniaSharedState};
use crate::core::audio_player::audio_output_wasapi::{
    self, NegotiatedFormat, WasapiSymphoniaState, default_output_device_name, run_wasapi_playback,
    try_negotiate_exclusive_format,
};

/// Backend WASAPI exclusive mode pour la voie Symphonia. Gère les deux modes
/// de lecture (LiveDecode + FullBuffer) comme CPAL, via `WasapiSymphoniaState`.
pub struct WasapiExclusiveOutput {
    /// Handle du thread render. `Option` parce qu'on le take() au Drop.
    handle: Option<JoinHandle<Result<(), audio_output_wasapi::WasapiPlayerError>>>,
    /// Format effectivement négocié (souvent = source rate exactement).
    format: NegotiatedFormat,
    /// Nom du device (récupéré à l'init pour le frontend).
    device_name: String,
    /// Atomics partagés. Le Drop set `is_stopped=true` pour signaler au thread.
    atomics: PlaybackAtomics,
}

/// Erreur enrichie qui rend le `Consumer` au caller si la négociation échoue,
/// pour permettre un fallback CPAL propre. Le commit (spawn du thread) prend
/// possession définitive du consumer.
pub struct WasapiBuildError<C> {
    pub consumer: C,
    pub error: AudioOutputError,
}

impl WasapiExclusiveOutput {
    /// Tente de construire le backend WASAPI. La méthode garantit qu'en cas
    /// d'échec **avant** le spawn du thread, le `consumer` est rendu au caller
    /// dans `WasapiBuildError::consumer` pour permettre un fallback CPAL.
    ///
    /// Si la négociation réussit ET que le thread est spawn, le consumer est
    /// définitivement transféré au backend.
    pub fn try_new<C>(
        source_sample_rate: u32,
        source_channels: u16,
        preferred_device_name: Option<String>,
        atomics: PlaybackAtomics,
        shared: SymphoniaSharedState,
        consumer: C,
    ) -> Result<Self, WasapiBuildError<C>>
    where
        C: Consumer<Item = f32> + Send + 'static,
    {
        // Étape 1 : négociation du format sur le device préféré (fallback
        // sur le default Windows si non trouvé).
        let format = match try_negotiate_exclusive_format(
            source_sample_rate,
            source_channels,
            preferred_device_name.clone(),
        ) {
            Ok(f) => f,
            Err(e) => {
                return Err(WasapiBuildError {
                    consumer,
                    error: AudioOutputError::NegotiationFailed(e.to_string()),
                });
            }
        };

        // Étape 2 : nom du device pour l'UI — préfère le nom sélectionné,
        // sinon celui du défaut Windows.
        let device_name = preferred_device_name
            .clone()
            .or_else(|| default_output_device_name().ok())
            .unwrap_or_else(|| "Device WASAPI inconnu".to_string());

        // Étape 3 : spawn du render thread. À partir de là le consumer est
        // définitivement transféré.
        let is_paused = atomics.is_paused.clone();
        let is_stopped = atomics.is_stopped.clone();
        let volume = atomics.volume.clone();
        let current_position_frames = atomics.current_position_frames.clone();
        let seek_flush = shared.seek_flush.clone();
        let format_for_thread = format;
        let preferred_for_thread = preferred_device_name.clone();
        // État partagé pour le mode FullBuffer (le décodeur y bascule après
        // pré-remplissage — WASAPI doit le gérer comme CPAL).
        let sym_state = WasapiSymphoniaState {
            current_source: shared.current_source.clone(),
            full_buffer_data: shared.full_buffer_data.clone(),
            full_buffer_cursor: shared.full_buffer_cursor.clone(),
            is_full_buffer_ready: shared.is_full_buffer_ready.clone(),
            pending_seek_frames: shared.pending_seek_frames.clone(),
            output_channels: shared.output_channels,
        };
        let handle = std::thread::Builder::new()
            .name("rustmusic-wasapi-render".into())
            .spawn(move || {
                run_wasapi_playback(
                    format_for_thread,
                    consumer,
                    is_paused,
                    is_stopped,
                    volume,
                    current_position_frames,
                    seek_flush,
                    preferred_for_thread,
                    sym_state,
                )
            })
            // Si le spawn échoue (très rare — typiquement OOM), on n'a plus le
            // consumer (moved into the closure), donc on perd la possibilité
            // de fallback. C'est un cas extrême acceptable.
            .map_err(|e| WasapiBuildError {
                // SAFETY: on ne peut pas reconstituer le consumer ici, mais
                // ce panic ne devrait jamais arriver en pratique.
                consumer: panic!("wasapi thread spawn failed (OOM?): {e}"),
                #[allow(unreachable_code)]
                error: AudioOutputError::BuildFailed(format!("thread spawn: {e}")),
            })?;

        Ok(Self {
            handle: Some(handle),
            format,
            device_name,
            atomics,
        })
    }
}

impl AudioOutput for WasapiExclusiveOutput {
    fn start(&mut self) -> Result<(), AudioOutputError> {
        // Pas d'action : le thread render est déjà en cours dès `try_new`.
        // `start()` existe pour aligner avec CPAL où `stream.play()` doit être
        // appelé explicitement.
        Ok(())
    }

    fn output_sample_rate(&self) -> u32 {
        self.format.sample_rate
    }

    fn output_channels(&self) -> u16 {
        self.format.channels
    }

    fn device_name(&self) -> &str {
        &self.device_name
    }

    fn backend(&self) -> AudioBackend {
        AudioBackend::WasapiExclusive
    }
}

impl Drop for WasapiExclusiveOutput {
    fn drop(&mut self) {
        // Signal au render thread de s'arrêter.
        self.atomics.is_stopped.store(true, Ordering::SeqCst);
        // Wait sa terminaison pour cleanup propre (libère AudioClient + COM).
        if let Some(handle) = self.handle.take() {
            if let Err(e) = handle.join() {
                log::warn!("WASAPI render thread join panic: {e:?}");
            }
        }
    }
}
