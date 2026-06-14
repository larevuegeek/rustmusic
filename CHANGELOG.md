# Changelog

## [0.1.7] - 2026-05-21

### Audio — Profil de qualité de décodage
- Système de profils audio Auto / Qualité maximale / Équilibré / Compatibilité / Mode dégradé
- Auto détecte les VMs et le nombre de cœurs CPU pour choisir le bon preset (VM → Mode dégradé, < 4 cœurs → Équilibré, sinon Qualité maximale)
- Mode dégradé : filtre DSD 256 taps + sortie 44,1 kHz + pré-décodage complet du fichier en RAM avant lecture (zéro underrun garanti)
- Filtre DSD configurable par profil : 2048 / 1024 / 512 / 256 taps
- Resampler chunk size et sub-chunks adaptés au profil
- Settings : 5 cartes de sélection + bloc d'information affichant le profil résolu et la machine détectée

### Audio — Décodage parallèle DSD multicanal
- Parallélisation du DSD2PCM via rayon : 1 thread par canal pour les fichiers 3+ canaux (SACD multicanal 5.0 / 5.1)
- Stéréo et mono restent en séquentiel (overhead rayon non justifié)
- Speedup quasi-linéaire sur les multicanaux : un SACD DSD64 5.0 en Qualité maximale est désormais lisible sans grésillement sur un CPU normal

### Audio — Downmix multicanal vers stéréo (ITU-R BS.775)
- Remplace l'ancien "copie ch0 + ch1 seulement" qui supprimait la voix centrale et les surrounds
- 3.0 → stéréo : L + 0,707·C × 0,707
- 4.0 (quad) → stéréo : L + 0,707·LS × 0,707
- 5.0 → stéréo : (L + 0,707·C + 0,707·LS) × 0,5 (−6 dB pour éviter le clipping)
- 5.1 → stéréo : ajoute 0,5·LFE dans le mix
- Tu entends maintenant correctement les SACD multicanaux en stéréo

### Audio — Pré-chargement et seek
- Pré-chargement du fichier audio en RAM quand le morceau est sur un partage réseau (GVFS/SMB/NFS/SSHFS) ou en profil Bas / Mode dégradé — élimine les coupures liées à l'I/O
- Seek instantané en Mode dégradé pour DSD : drain du ring buffer dans un `Vec<f32>`, CPAL lit en mode FullBuffer (curseur indexé) au lieu du FIFO
- Fix race condition Phase 6 / CPAL sur le seek en Mode dégradé Symphonia (atomique dédié `pending_seek_frames`)

### Audio — Robustesse
- CPAL `BufferSize::Fixed` : la valeur est désormais clampée dans la plage supportée par le périphérique (fix stream silencieux sur certaines cartes Intel HD)
- Log des erreurs CPAL rate-limité (1ère, puis 1 sur 100 jusqu'à 1000, puis 1 sur 1000) — sur VM faible on peut en recevoir des centaines par seconde
- Defense-in-depth scanner DSD : `catch_unwind` supplémentaire à l'entrée DSD pour transformer toute panic des parsers DSF/DFF en erreur propre — impossible de tuer le batch d'import
- Aggregation des fichiers ignorés à l'import + event Tauri enrichi (`skipped: usize`)

### Système de notifications OS
- Notification native (Windows action center / macOS / Linux libnotify) au changement de morceau avec titre + artiste
- Skip de la première émission au démarrage (restore de queue ≠ action utilisateur)
- Toggle dans Réglages (icône cloche)

### System Media Transport Controls (SMTC / MPRIS / Now Playing)
- Le morceau en cours apparaît dans le mini-player Windows (volume flyout), l'écran de verrouillage, et le widget Now Playing macOS / MPRIS Linux (KDE Plasma, GNOME)
- Les touches média du clavier (play/pause/next/prev) fonctionnent désormais
- Serveur HTTP local dédié pour servir les covers (cross-OS, contourne la limitation Windows unpackaged qui rejette les `file://`)
- Toggle dans Réglages pour désactiver l'intégration (utile sur Linux sans D-Bus, Windows N sans Media Pack)
- Implémenté via le crate `souvlaki`

### Render mode (Linux)
- Override manuel Automatique / Accélération GPU / Rendu logiciel
- Auto détecte si l'app tourne en VM et bascule en software rendering pour la stabilité
- Settings dédiés avec bannière "Redémarrage requis"
- Variables d'environnement WebKitGTK / GDK appliquées avant le démarrage de Tauri

### UI Player
- Popover hover stylé sur le badge "source → sortie" (PipelineInfoPopover) — affiche Source / Décodage / Resampling / Sortie / Profil avec badges Bit-perfect / DSP / DSD→PCM
- Indication multicanal correcte dans le popover : "3.0" / "4.0" / "5.0" / "5.1" / "6.1" / "7.1"
- Layout responsive sur 3 breakpoints : mobile (stacked), intermédiaire 500-768px (cover + actions en haut, transport en bas), desktop ≥768px (3 colonnes)
- Toutes les actions transport (shuffle, ±10s, play, next, repeat) restent visibles à toutes les tailles
- Fallback titre depuis le nom de fichier pour les DSF/DFF sans tag DITI/ID3
- Indicateur de pré-décodage dans la StatusBar (Mo décodés / Mo total) au lieu d'un loader dans le Player

### Internationalisation
- Mise à parité totale des 4 langues : FR, EN, DE, ES (toutes à 282/282 clés strictement identiques)
- Sections rattrapées : bibliothèque (genres, artist_label, total_duration), réglages (audio_quality_*, render_mode_*, scan_on_startup, single_click_play, album_covers), pipeline, statistiques, recherche, common

### Stabilité & Bugs
- Détection automatique du format réel des images via les magic bytes : fix des covers PNG/WebP renommées en `.jpg` (warning `Format error decoding Jpeg: Illegal start bytes` éliminé)
- Iconify offline : pré-bundling de 7 collections (lucide, heroicons, mynaui, ph, radix-icons, tabler, uit) — fix des icônes invisibles sur Debian 12 + WebKitGTK 2.40, fonctionne désormais sans connexion
- Auto-config Linux : détection VM + variables d'environnement WebKit / GDK pour éviter les freezes au démarrage sur certaines VMs (KVM, KDE Wayland, AMD Mesa)
- Fix svelte-check : 10 erreurs préexistantes corrigées (EditPlaylistPopin, ProfilSelectorPopin, page détail morceau avec ajout de `library_artist_id` au modèle TS + SQL JOIN library_artists)

## [0.1.6] - 2026-05-05

### Audio metadata extractor — refonte complète
- Nouveau module `audio_metadata` (extractor + file_format + tag_format)
- Parser ID3v2.3 / v2.4 entièrement maison : 35+ frames mappés (TIT2, TPE1, TALB, TCON, TRCK, TPOS, TBPM, TKEY, POPM, APIC, USLT, COMM, etc.)
- Support des encodages ISO-8859-1 / UTF-16 BE+LE avec BOM / UTF-16BE no BOM (v2.4) / UTF-8
- Helpers unsynchronisation, synchsafe int, split_at_null encoding-aware
- Folder cover fallback consolidé (cover.jpg / folder.jpg / front.{jpg,jpeg,png,webp,avif})
- Champ `total_tracks` ajouté, `compilation` typé bool, parsing TCMP correct

### Lecture DSD native (DSF + DFF)
- Décodage en pur Rust, sans Symphonia ni FFmpeg
- Parser DSF (Sony) et DFF (Philips DSDIFF) avec leur format de bytes spécifique (LSB-first vs MSB-first)
- Convertisseur DSD → PCM via l'algorithme Gesemann avec LUT 256 × 256 + filtre Blackman-Harris 2048 taps (foobar2000-grade)
- Initialisation de l'historique du filtre à `0x69` pour éliminer le pop au démarrage
- Décimation fixe 32× (DSD64 → 88,2 kHz, DSD128 → 176,4 kHz, etc.)
- Pipeline complète : Decoder → DSD2PCM → Resampler rubato → ring buffer → CPAL
- Seek block-aligned validé sur DSF et DFF

### Frontend DSD
- Badge "DSD64" doré dans le player, les listes, les vues compactes
- Affichage du signal en MHz (au lieu de kHz) pour les DSD
- Helper `audioFormatTools.ts` : `isDsdFormat()`, `dsdLabel()` ("DSD64/128/256/512/1024")
- Le badge porte tout, le format/bits/kHz redondants sont masqués

### Serveur DLNA / UPnP intégré
- Découverte automatique sur le réseau local via SSDP multicast 239.255.255.250:1900
- Streaming HTTP avec support des Range requests pour le seek dans les apps clientes
- ContentDirectory : navigation par Artistes / Albums / Dossiers
- Multi-bibliothèques natif (chaque lib expose ses propres contenus)
- Covers DLNA via `<upnp:albumArtURI>` dans le DIDL-Lite
- Section "Réseau" dans les réglages : toggle, nom du serveur, port, statut, copier l'URL
- Indicateur de statut dans la barre du player
- Auto-start au lancement si activé en réglages

### Paroles synchronisées (Apple Music style)
- Table SQLite `lyrics` avec source enum (Sidecar, LRCLIB, Manual, None)
- Client LRCLIB pour récupérer automatiquement les paroles synchronisées
- Lecteur de sidecar `.lrc` à côté du fichier audio (heuristique has_synced_timestamps)
- Parser LRC frontend avec binary search activeIndex O(log n) + métadonnée offset
- Auto-scroll basé uniquement sur la ligne active (instant si delta > 3 lignes, smooth sinon)
- Fond cover floutée + voile noir + dégradé en arrière-plan du panel
- Service avec dedup in-flight pour éviter les requêtes en double

### Instance unique
- Plugin `tauri-plugin-single-instance` : un seul process à la fois
- Au second lancement, la fenêtre existante est restaurée, focus, demande d'attention sur la barre des tâches

### Stabilité & Qualité
- Fix critique : `LibraryFile.modified_at` (INTEGER en BDD vs `Option<String>` en entity) faisait échouer le décodage en `SELECT *` → aucun morceau n'apparaissait dans la vue Dossiers DLNA
- Logger termlogger + cleanup massif de la verbosité (info → debug pour les events per-action SSDP, HTTP, SOAP, audio player, DSD, resampler)
- Helper formatBitrate qui affiche en Mb/s quand bitrate ≥ 1000 kbps
- Composant réutilisable `NowPlayingCard` avec variantes default / blur (mutualisé entre QueuePanel et LyricsPanel)

## [0.1.5] - 2026-05-02

### Paroles synchronisées (foundation)
- Table SQLite `lyrics` + migration + repository + entité Rust
- Commandes Tauri `get_lyrics(path)` et `refresh_lyrics(path)`

### Section Apparence (réglages)
- Nouveau panneau "Apparence" avec sélecteur de thème Auto / Clair / Sombre
- Mini previews du thème dans chaque carte (split image)
- Application live à chaque changement, listener `prefers-color-scheme` pour le mode Auto

### Contrôles fenêtre — 4 styles + 2 positions
- Auto (détection OS) / macOS / Windows / Linux
- Style macOS : 3 traffic lights colorés
- Style Windows : 3 boutons rectangulaires
- Style Linux : 3 boutons ronds GNOME Adwaita
- Position à droite ou à gauche de la barre de titre

### Instance unique
- Plugin `tauri-plugin-single-instance` v2 (foundation, finalisé en 0.1.6)

## [0.1.4] - 2026-04-21

### Pochettes d'albums
- Recuperation automatique via l'API Deezer (menu contextuel album)
- Recherche manuelle via un popin premium avec grille de resultats
- Choix d'un fichier local comme pochette (file picker)
- Sous-menu "Changer de pochette" dans le menu contextuel album avec 3 options
- Batch "Pochettes d'albums" dans les settings pour traiter tous les albums sans pochette
- Filtre "Sans pochette" sur la page albums (toggle dans la FilterBar)

### Notation des morceaux
- Extraction automatique du tag POPM/Rating au scan (support des formats 0-5, 0-100, 0-255)
- Composant StarRating cliquable (5 etoiles vertes, hover preview, glow)
- Integration dans la vue liste compacte, la vue grille et la page detail du morceau
- Tri par notation dans la page morceaux (NULLs places en fin de liste)
- Sauvegarde en base avec update optimiste (UI reactive sans refresh)

### Navigation alphabetique
- Composant AlphabetNav reutilisable (A-Z + #) style ascenseur
- Scroll-to au clic sur une lettre (style Spotify/Apple Music)
- Lettres grisees pour celles sans entree
- Integre sur les pages albums, artistes et morceaux
- Bouton toggle dans la barre de vue pour afficher/masquer

### Filtres et tri
- Refonte premium de la FilterBar : select dropdown pour le tri + bouton direction ASC/DESC integre
- Filtre "Sans pochette" etendu aux morceaux
- Direction DESC par defaut pour notation, duree, date (plus intuitif)
- Memorisation du tri dans localStorage par contexte (morceaux / albums / artistes / genres)
- Layout uniformise avec hauteurs coherentes et dropdown flottant au clic

### Menu contextuel
- Sous-menu "Ajouter a une playlist" sur les menus contextuels (ajout groupe depuis les albums et selections multiples)
- Padding ajuste sur les sous-menus pour une hierarchie visuelle plus claire

### Settings
- Option "Simple clic = lecture" (single_click_play) sur les listes de morceaux

### Stabilite
- Double protection contre les crashs de scan : catch_unwind par fichier + par batch rayon complet
- Si un batch crash en parallele, fallback sequentiel automatique fichier par fichier
- Log des fichiers du batch problematique pour identifier le coupable

### Corrections
- Le tri etait reinitialise a chaque changement de bibliotheque sur la page morceaux

## [0.1.3] - 2026-04-07

### Selection multiple
- Mode selection sur les listes de morceaux (checkbox, barre d'actions flottante)
- Actions groupees : lire, ajouter a la file, ajouter a une playlist
- Boutons "Selectionner" / "Tout selectionner" dans la barre de filtres
- Support sur les 3 vues : cards, compacte et discographie

### Albums
- Bouton play au survol des covers dans la grille d'albums
- Bouton "..." (menu contextuel) au survol des covers
- Ajout de "Ajouter a une playlist" dans le menu contextuel album (ajout groupé de tous les tracks)
- Bouton "..." avec menu contextuel sur la page detail album
- Tri des morceaux par N° / Titre / Duree sur la page album
- Tri des "Autres albums" par Annee / Titre sur la page album
- Zoom lightbox sur la cover album (clic pour agrandir)
- Border-radius des covers reduit (rounded-lg)

### Artistes
- Tri de la discographie par Album / Annee / Titre
- Tri des albums par Annee / Titre
- Nom d'album + annee affiches dans la discographie
- Zoom lightbox sur la photo artiste
- Comptage correct des tracks pour les artistes featured (via library_track_artists)

### Queue
- Drag & drop fonctionnel via pointer events (reorganisation des morceaux)
- Effets visuels premium : ligne de drop emerald avec glow, opacite, scale
- Bouton play/pause dans le header de la queue
- Correction du style scrollbar

### Player
- Correction du bouton pause/play qui remettait le morceau a zero (resume au lieu de replay)

### Grilles responsives
- Ajout de breakpoints pour grands ecrans : 6 colonnes a 1536px+, 7 a 1800px+, 8 a 2200px+

### Stabilite
- Protection catch_unwind contre les crashs Symphonia sur fichiers corrompus/exotiques
- Ajout des extensions wav, aiff, opus, aac au scan de bibliotheque
- Changement de library redirige vers la meme section avec le nouveau library_id

### Configuration
- Categorie macOS corrigee : Music au lieu de DeveloperTool
- Support minimum macOS 10.15 (Catalina) pour Intel
- Licence "Free for personal use" dans la page A propos

## [0.1.2] - 2026-04-05

### Profils
- Ajout d'un bouton "Modifier" visible sous chaque profil dans la popin de selection
- Confirmation de suppression de profil : il faut taper "supprimer" pour confirmer
- La creation d'un profil ne quitte plus la popin et ne le selectionne plus automatiquement

### Page artiste — Performance
- Chargement progressif avec skeletons : le hero s'affiche instantanement, les sections se remplissent au fur et a mesure
- Nouvelles commandes backend ciblees : `get_albums_by_artist`, `get_similar_artists` (remplacent le chargement de TOUS les albums/artistes)
- Optimisation des requetes SQL : suppression des sous-requetes `OR IN (SELECT ...)` couteuses
- Ajout de 6 index SQLite manquants (`library_tracks.artist_id`, `library_albums.artist_id`, `library_albums.genre`, etc.)

### Playlists
- Ajout du bouton "Tout lire" sur la page playlist

### Recherche
- Correction du lien artiste dans les resultats de recherche (le `library_id` etait null)

### Feedback
- Remplacement du formulaire de contact (non fonctionnel) par une page avec lien mailto `contact@rustmusic.dev`
- Suppression des mentions a RiffFlow
- Mise a jour des traductions (FR, EN, ES, DE)

## [0.1.1] - 2026-03-30

### Linux
- Correction de l'affichage des covers sur Linux (contournement du bug WebKitGTK 2.50 avec le protocole `asset://`)
- Correction de l'encodage `%2F` dans les URLs asset sur Linux
- Ajout du support de compilation `.deb` et `.rpm`
- Correction du warning GTK `gtk_widget_get_scale_factor`

### Covers & Thumbnails
- Nouveau composant `CoverImg` avec chargement async et cache LRU (max 300 entrées, ~40MB)
- Commande Rust `read_cover_as_base64` pour servir les covers via IPC (fallback base64)
- Fonction `assetSrc()` pour corriger l'encodage des chemins Linux
- Support des tailles de miniatures (`full`, `1x`, `2x`) avec generation en background
- Reorganisation des covers : `covers/albums/` et `covers/artists/` avec sous-dossiers `full/1x/2x`
- Migration automatique des anciennes covers vers la nouvelle structure
- Redimensionnement rapide via `fast_image_resize` (SIMD) au lieu du crate `image`
- Generation a la volee des miniatures manquantes (`resolve_thumbnail`) avec fallback sur `full`
- Pool de threads dedie (50% des cores) pour la generation de miniatures en arriere-plan
- Double mode d'affichage : `asset` protocol (direct, rapide) ou `base64` IPC (fallback)
- Filtrage des images artistes par defaut de Deezer (detection du pattern URL sans hash)

### Images artistes
- Live update des images artistes : apparition en temps reel pendant le fetch Deezer
- Store reactif `artistImageReadyStore` avec event `artist-image-ready`
- Recuperation des images artistes en mode force (re-telecharge meme si le chemin existe en base)
- Sauvegarde des images artistes dans `covers/artists/full/` avec miniatures en background

### Import & Progression
- Nouveau composant d'import premium : cercle de progression avec glow, shimmer, ETA
- Estimation du temps restant en live pendant l'import
- StatusBar redesignee : gradient, shimmer, pourcentage, bouton d'annulation au hover
- Refresh automatique des donnees apres import (`libraryContentStore.refresh()`)
- Correction du freeze de 30s a l'annulation du dialogue d'import
- Loader d'import ajoute aux pages Genres et Explorateur de fichiers
- Migration des covers avec progression dans la statusbar

### Parametres
- Ajout du bouton "Ouvrir le dossier de donnees" (ouvre l'explorateur sur le dossier AppData)
- Notifications desactivees par defaut (option retiree des parametres, sera reintroduite plus tard)
- Traductions ajoutees (FR, EN, ES, DE) pour les nouvelles entrees

### UI
- Correction de la troncature des titres longs (noms de fichiers) dans toutes les vues (album, playlists, queue, player, stats, recherche)
- Ajout du `title` (tooltip au hover) pour afficher le nom complet des titres tronques

### Optimisation
- Tailles de covers adaptees par contexte : `1x` pour les listes, `2x` pour les grilles, `full` pour les pages detail
- Transaction SQL pour les migrations de covers (rollback en cas d'erreur)
- Generation des miniatures en 2 passes : deplacement + DB (rapide) puis resize en parallele (rayon)

## [0.1.0] - Initial Release

- Lecteur audio FLAC/WAV/MP3/OGG/OPUS
- Gestion de bibliotheques avec scan automatique
- Interface Svelte 5 + Tailwind CSS 4
- Mode clair / sombre
- File d'attente avec persistance SQLite
- Recherche globale
- Playlists et favoris
- Profils utilisateur
- Auto-update via Tauri updater
