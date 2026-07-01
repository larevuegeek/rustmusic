use sqlx::SqlitePool;

pub struct LibraryGenreRepository;

impl LibraryGenreRepository {
    /// Récupère tous les genres d'une bibliothèque avec stats (albums, tracks)
    pub async fn find_all_genres(
        pool: &SqlitePool,
        library_id: i64,
    ) -> Result<Vec<(String, i64, i64)>, sqlx::Error> {
        sqlx::query_as::<_, (String, i64, i64)>(
            r#"
            SELECT
                lc.genre AS name,
                COUNT(DISTINCT la.id) AS total_albums,
                COUNT(DISTINCT lt.id) AS total_tracks
            FROM library_cache lc
            INNER JOIN library_tracks lt ON lt.cache_id = lc.id
            LEFT JOIN library_albums la ON la.id = lt.library_album_id
            WHERE lt.library_id = ?
              AND lc.genre IS NOT NULL AND lc.genre != ''
            GROUP BY lc.genre
            ORDER BY total_tracks DESC
            "#,
        )
        .bind(library_id)
        .fetch_all(pool)
        .await
    }

    /// Récupère les 4 premières covers d'un genre
    pub async fn find_genre_covers(
        pool: &SqlitePool,
        genre: &str,
        library_id: i64,
    ) -> Vec<String> {
        sqlx::query_scalar::<_, String>(
            r#"
            SELECT DISTINCT la.cover_url
            FROM library_albums la
            WHERE la.genre = ? AND la.library_id = ? AND la.cover_url IS NOT NULL AND la.cover_url != ''
            LIMIT 4
            "#,
        )
        .bind(genre)
        .bind(library_id)
        .fetch_all(pool)
        .await
        .unwrap_or_default()
    }
}
