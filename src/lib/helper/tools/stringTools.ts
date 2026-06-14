

function truncateMiddle(string: String, maxLength = 40): String {

    if(string.length <= maxLength) {
        return string;
    }

    //Longeur max (- les ...)
    const keep = Math.floor((maxLength - 3) / 2);

    const start = string.slice(0, keep);
    const end = string.slice(-keep);


    return `${start}...${end}`;
}  

function getFolderPath(path: string): string {
  if (!path) return path;

  // Normalise: \ -> / et supprime les / finaux
  const p = path.replace(/\\/g, "/").replace(/\/+$/, "");
  if (!p) return "/";

  const idx = p.lastIndexOf("/");
  if (idx === -1) return "/";

  // Racine unix
  if (idx === 0) return "/";

  return p.slice(0, idx);
}

/**
 * Filename (basename) of a path, without the extension.
 *   /a/b/A5.Remedy.dff  →  "A5.Remedy"
 *   C:\\Music\\song.flac →  "song"
 *
 * Returns an empty string if the path is empty.
 */
function fileStem(path: string | null | undefined): string {
  if (!path) return "";
  const normalized = path.replace(/\\/g, "/");
  const last = normalized.split("/").pop() ?? "";
  const dot = last.lastIndexOf(".");
  return dot > 0 ? last.slice(0, dot) : last;
}

/**
 * Best-effort display title for a track.
 *   - tag title if present (e.g. "Crazy In Love")
 *   - fallback to the filename without extension when the tag is missing
 *     (typical of DSF/DFF files that don't embed an ID3 chunk)
 *   - finally falls back to the provided `unknownLabel` (i18n)
 */
function displayTitle(
  title: string | null | undefined,
  path: string | null | undefined,
  unknownLabel: string,
): string {
  const t = title?.trim();
  if (t) return t;
  const stem = fileStem(path);
  if (stem) return stem;
  return unknownLabel;
}

export { truncateMiddle, getFolderPath, fileStem, displayTitle };