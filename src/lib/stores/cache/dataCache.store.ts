/**
 * ═══════════════════════════════════════════════════════════
 * dataCache — Cache mémoire avec Stale-While-Revalidate
 * ═══════════════════════════════════════════════════════════
 *
 * POURQUOI CE STORE ?
 * Quand tu navigues albums → détail album → retour albums,
 * sans cache le navigateur refait un appel SQL à chaque fois.
 * Avec le cache, on stocke les résultats en mémoire (Map JS)
 * et on les réutilise instantanément.
 *
 * COMMENT ÇA MARCHE ?
 *
 * 1. `get(clé)` — retourne les données du cache + leur âge
 *    Si les données existent et ont < MAX_AGE, on les sert directement.
 *    Si elles sont "stale" (vieilles), on les sert QUAND MÊME
 *    mais on signale qu'un refresh serait utile.
 *
 * 2. `set(clé, données)` — stocke les données avec un timestamp
 *
 * 3. `invalidate(clé)` — supprime une entrée (après un import par ex.)
 *
 * 4. `invalidateAll()` — vide tout le cache
 *
 * La CLÉ est une string libre, par convention on utilise :
 *   - "tracks:42"      → tracks de la library 42
 *   - "albums:42"      → albums de la library 42
 *   - "artists:42"     → artistes de la library 42
 *   - "album:abc-123"  → détail de l'album abc-123
 *   - "artist:def-456" → détail de l'artiste def-456
 */

// ─── Types ───

/** Une entrée dans le cache : la donnée + quand elle a été stockée */
type CacheEntry<T> = {
  data: T;
  timestamp: number; // Date.now() au moment du set()
};

/** Ce que retourne get() : la donnée + un booléen "est-ce encore frais ?" */
type CacheResult<T> = {
  data: T;
  fresh: boolean; // true = pas besoin de re-fetch, false = stale, refresh recommandé
};

// ─── Configuration ───

/**
 * MAX_AGE en millisecondes.
 * Les données de moins de 30 secondes sont considérées "fresh"
 * → on ne re-fetch pas du tout.
 *
 * Au-delà, on les affiche mais on signale qu'un refresh serait bien.
 *
 * 30s est un bon compromis :
 * - Navigation rapide (aller-retour) = instantané, pas de fetch
 * - Si l'utilisateur reste 1 min sur une page et revient,
 *   on affiche le cache puis on met à jour silencieusement
 */
const MAX_AGE = 30_000; // 30 secondes

// ─── Le cache lui-même ───

/**
 * Map<string, CacheEntry<any>>
 *
 * Pourquoi une Map et pas un objet {} ?
 * - Map est optimisé pour des insertions/suppressions fréquentes
 * - Map.delete() est O(1), contrairement à `delete obj[key]`
 * - Map garde l'ordre d'insertion (utile pour un futur LRU)
 *
 * Le `any` ici est inévitable car le cache stocke des types différents
 * (tracks[], albums[], ArtistDetailView...). La sécurité de type
 * est assurée par le générique <T> dans get() et set().
 */
const cache = new Map<string, CacheEntry<any>>();

// ─── API publique ───

export const dataCache = {

  /**
   * Récupère une entrée du cache.
   *
   * @param key — la clé (ex: "albums:42")
   * @returns null si rien en cache, sinon { data, fresh }
   *
   * Exemple d'utilisation dans un composant :
   * ```ts
   * const cached = dataCache.get<AlbumListView[]>(`albums:${libraryId}`);
   * if (cached) {
   *   albums = cached.data;          // affichage immédiat
   *   if (!cached.fresh) refresh();  // refresh en background si stale
   * } else {
   *   await fullLoad();              // premier chargement
   * }
   * ```
   */
  get<T>(key: string): CacheResult<T> | null {
    const entry = cache.get(key);
    if (!entry) return null;

    // Calcule l'âge de l'entrée
    const age = Date.now() - entry.timestamp;

    return {
      data: entry.data as T,
      fresh: age < MAX_AGE, // < 30s = frais, pas besoin de refresh
    };
  },

  /**
   * Stocke une entrée dans le cache.
   *
   * @param key — la clé
   * @param data — les données à cacher
   *
   * Le timestamp est automatiquement mis à Date.now().
   * Si la clé existe déjà, elle est écrasée (mise à jour).
   */
  set<T>(key: string, data: T): void {
    cache.set(key, {
      data,
      timestamp: Date.now(),
    });
  },

  /**
   * Invalide (supprime) une entrée spécifique.
   *
   * Quand l'appeler ?
   * - Après un import de fichiers → invalidate("tracks:42")
   * - Après un rescan → invalidateByPrefix("tracks:"), etc.
   * - Après une suppression
   *
   * L'entrée sera re-fetchée au prochain accès.
   */
  invalidate(key: string): void {
    cache.delete(key);
  },

  /**
   * Invalide toutes les entrées dont la clé commence par `prefix`.
   *
   * Utile après un import ou rescan qui affecte plusieurs types :
   * ```ts
   * dataCache.invalidateByPrefix("tracks:");
   * dataCache.invalidateByPrefix("albums:");
   * dataCache.invalidateByPrefix("artists:");
   * ```
   *
   * Ou plus radical : invalidateAll()
   */
  invalidateByPrefix(prefix: string): void {
    // On itère sur les clés de la Map
    // et on supprime celles qui matchent le prefix
    for (const key of cache.keys()) {
      if (key.startsWith(prefix)) {
        cache.delete(key);
      }
    }
  },

  /**
   * Vide tout le cache.
   *
   * Quand l'appeler ?
   * - Changement de profil
   * - Changement de bibliothèque
   * - Reset de l'app
   */
  invalidateAll(): void {
    cache.clear();
  },

  /** Retourne le nombre d'entrées en cache (debug) */
  size(): number {
    return cache.size;
  },
};
