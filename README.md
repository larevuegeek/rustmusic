# RustMusic

> Lecteur de musique HD multi-plateforme. Lecture native **DSD** (DSF/DFF), résampling haute qualité, intégration **DLNA**, **paroles synchronisées**, intégration **système (SMTC / MPRIS / Now Playing)**.

**Site officiel** : [rustmusic.dev](https://rustmusic.dev)
**Téléchargements** : [rustmusic.dev/downloads](https://rustmusic.dev/downloads)

---

## Fonctionnalités

### Audio
- Formats lus : **FLAC, WAV, AIFF, MP3, OGG, OPUS, AAC, M4A**
- Lecture **DSD native** (DSF + DFF) en pur Rust, filtre Blackman-Harris 2048 taps qualité foobar2000
- Resampler haute précision (FFT via [rubato](https://github.com/HEnquist/rubato))
- Profils de qualité audio : **Auto / Qualité maximale / Équilibré / Compatibilité / Mode dégradé** (adapté aux VMs / CPU contraints)
- Décodage parallèle DSD multicanal (SACD 5.0 / 5.1 sur multi-core)
- Downmix multicanal correct vers stéréo (ITU-R BS.775)
- Override mode de rendu Linux (Auto / GPU / Software) pour stabilité VM

### Bibliothèque
- Scan automatique de dossiers (rayon multi-thread)
- Métadonnées extraites maison : ID3v2.3 / v2.4, DSF / DFF DITI, sidecars `.lrc`
- Covers : extraction auto + récupération **Deezer** + ajout manuel
- Filtres : sans pochette, sans tag, etc.
- Tri persistant par contexte (localStorage)
- Navigation alphabétique A-Z style ascenseur
- Notation 0-5 étoiles (lecture/écriture POPM)

### Player & UI
- File d'attente avec drag & drop
- Mode shuffle / repeat
- Paroles synchronisées (LRCLIB + sidecars `.lrc`)
- Profils utilisateur multiples (chaque profil a sa bibliothèque, ses playlists, sa file)
- Thèmes Clair / Sombre / Auto
- Style de contrôles de fenêtre paramétrable (macOS / Windows / Linux / Auto)
- Layout responsive sur 3 breakpoints

### Réseau & système
- **Serveur DLNA / UPnP** intégré (partage ta bibliothèque sur le réseau local)
- **Notifications OS** au changement de morceau
- **System Media Transport Controls** (SMTC Windows, MPRIS Linux, Now Playing macOS)
  - Touches média clavier (play/pause/next/prev)
  - Mini-player dans le volume flyout Windows
  - Widget Now Playing macOS / KDE Plasma / GNOME
- Auto-updater intégré

### Internationalisation
- 4 langues : **Français**, **English**, **Deutsch**, **Español**

---

## Téléchargement

Pour les utilisateurs finaux, **télécharge les binaires signés** depuis [rustmusic.dev/downloads](https://rustmusic.dev/downloads) :

- Windows : `.exe` (installer NSIS) ou `.msi`
- macOS : `.dmg` (Intel + Apple Silicon)
- Linux : `.deb` (Debian, Ubuntu, Mint) ou `.rpm` (Fedora, openSUSE) ou `.AppImage`

---

## Compiler depuis les sources

### Prérequis

- **Rust** ≥ 1.75 ([rustup.rs](https://rustup.rs))
- **Node.js** ≥ 20 ([nvm](https://github.com/nvm-sh/nvm) recommandé)

#### Linux (Debian / Ubuntu)

```bash
sudo apt install -y \
  build-essential curl wget file git pkg-config libssl-dev \
  libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev \
  libwebkit2gtk-4.1-dev libsoup-3.0-dev libjavascriptcoregtk-4.1-dev patchelf \
  libasound2-dev libpulse-dev libdbus-1-dev
```

#### macOS

```bash
xcode-select --install
```

#### Windows

Installer [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (workload "Desktop development with C++") et [WebView2](https://developer.microsoft.com/microsoft-edge/webview2/).

### Build

```bash
git clone https://github.com/<votre-user>/rustmusic.git
cd rustmusic
npm install
npm run tauri build
```

Les paquets seront dans `src-tauri/target/release/bundle/`.

### Dev

```bash
npm run tauri dev
```

---

## Architecture

```
rust-music/
├── src/                       # Frontend SvelteKit + TypeScript
│   ├── lib/                   # Composants, stores, services, types, i18n
│   ├── routes/                # Pages SvelteKit
│   └── app.css                # Tailwind 4
├── src-tauri/                 # Backend Rust + Tauri 2
│   ├── src/
│   │   ├── core/              # Logique audio (player, decoder, resampler, DLNA)
│   │   ├── commands/          # Commandes Tauri exposées au frontend
│   │   ├── repository/        # Couche SQLite (sqlx)
│   │   ├── mapper/            # Mapping entités ↔ DTO
│   │   └── lib.rs             # Point d'entrée
│   ├── Cargo.toml
│   └── tauri.conf.json
└── CHANGELOG.md               # Historique des versions
```

### Stack

- **Frontend** : SvelteKit + Svelte 5 (runes) + TypeScript + Tailwind 4 + Vite
- **Backend** : Rust + Tauri 2 + tokio + axum (DLNA + serveur cover SMTC)
- **Audio** : CPAL + Symphonia + rubato + DSD2PCM maison
- **BDD** : SQLite via sqlx avec migrations versionnées
- **Intégration OS** : souvlaki (SMTC/MPRIS/Now Playing), tauri-plugin-notification

---

## Contribuer

Les contributions sont les bienvenues ! Lis [CONTRIBUTING.md](CONTRIBUTING.md) pour les détails sur le style de code, les conventions de commit, et le process de PR.

Pour signaler un bug ou proposer une fonctionnalité, ouvre une [issue GitHub](https://github.com/<votre-user>/rustmusic/issues).

---

## Licence

RustMusic est distribué sous **GNU General Public License v3.0** — voir [LICENSE](LICENSE).

En résumé :
- ✅ Tu peux utiliser RustMusic gratuitement, à des fins personnelles ou commerciales
- ✅ Tu peux étudier le code, le modifier, le redistribuer
- ⚠️ Si tu redistribues une version modifiée, elle **doit aussi être sous GPL-3.0** et tu **dois publier ton code source**
- ⚠️ Si tu redistribues un fork, merci de le **renommer** (pas "RustMusic" ou variante) et d'utiliser **un logo distinct** pour éviter la confusion avec le projet officiel

---

## Crédits

Développé avec passion par [LaRevueGeeK](https://rustmusic.dev). Icônes : [Lucide](https://lucide.dev), [Phosphor](https://phosphoricons.com), [Mynaui](https://mynaui.com).

Merci aux projets open source utilisés : Tauri, SvelteKit, Symphonia, CPAL, rubato, souvlaki, sqlx, axum, et tant d'autres.
