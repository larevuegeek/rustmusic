// Parse un fichier LRC en lignes timecodées.
// Format supporté : [mm:ss.cs]Texte de la ligne
// Plusieurs timestamps sur la même ligne (paroles répétées) → une entrée par timestamp.
//
// Tags de métadonnées supportés (ignorés mais pas en erreur) :
// [ti:Titre], [ar:Artiste], [al:Album], [length:mm:ss], [offset:+/-N]
// [offset:N] est appliqué globalement (en ms).

export type LrcLine = {
    /** Timestamp en millisecondes depuis le début */
    timeMs: number;
    /** Texte de la ligne (sans timestamp) */
    text: string;
};

const TIMESTAMP_RE = /\[(\d+):(\d+)(?:[.:](\d+))?\]/g;
const META_RE = /^\[(ti|ar|al|au|length|by|re|ve):/i;
const OFFSET_RE = /^\[offset:\s*([+-]?\d+)\s*\]/i;

/**
 * Parse un texte LRC complet en lignes triées par timestamp ascendant.
 * Les lignes vides ou purement métadonnées sont ignorées.
 */
export function parseLrc(content: string): LrcLine[] {
    if (!content) return [];

    let offsetMs = 0;
    const lines: LrcLine[] = [];

    for (const rawLine of content.split(/\r?\n/)) {
        const line = rawLine.trim();
        if (!line) continue;

        // Métadonnée [offset:N] → on capture
        const offsetMatch = line.match(OFFSET_RE);
        if (offsetMatch) {
            offsetMs = parseInt(offsetMatch[1], 10) || 0;
            continue;
        }

        // Autres métadonnées [ti:…] etc. → ignorées
        if (META_RE.test(line)) continue;

        // Extraire tous les timestamps de la ligne
        const matches = [...line.matchAll(TIMESTAMP_RE)];
        if (matches.length === 0) continue;

        // Texte = ce qui suit le dernier timestamp
        const lastMatch = matches[matches.length - 1];
        const lastIdx = (lastMatch.index ?? 0) + lastMatch[0].length;
        const text = line.slice(lastIdx).trim();

        for (const m of matches) {
            const minutes = parseInt(m[1], 10);
            const seconds = parseInt(m[2], 10);
            const fraction = m[3] ?? '0';
            // [mm:ss.cs] (centisecondes) ou [mm:ss.SSS] (millisecondes)
            const fractionMs = fraction.length === 3
                ? parseInt(fraction, 10)
                : parseInt(fraction.padEnd(2, '0').slice(0, 2), 10) * 10;

            const timeMs = minutes * 60_000 + seconds * 1000 + fractionMs;
            lines.push({ timeMs, text });
        }
    }

    // Appliquer l'offset global s'il existe
    if (offsetMs !== 0) {
        for (const l of lines) l.timeMs = Math.max(0, l.timeMs + offsetMs);
    }

    // Trier par timestamp (au cas où le LRC est mal ordonné)
    lines.sort((a, b) => a.timeMs - b.timeMs);

    return lines;
}

/**
 * Retourne l'index de la ligne active pour un timestamp donné.
 * -1 si aucune ligne n'a encore commencé.
 */
export function findActiveLineIndex(lines: LrcLine[], currentMs: number): number {
    if (lines.length === 0) return -1;

    // Recherche binaire : on cherche la dernière ligne dont timeMs <= currentMs
    let lo = 0;
    let hi = lines.length - 1;
    let result = -1;

    while (lo <= hi) {
        const mid = (lo + hi) >>> 1;
        if (lines[mid].timeMs <= currentMs) {
            result = mid;
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }

    return result;
}
