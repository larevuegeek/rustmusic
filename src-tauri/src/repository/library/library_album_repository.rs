// repository/album/album_repository.rs

use uuid::Uuid;

use crate::entity::library::library_album::LibraryAlbum;
use crate::entity::library::library_album::LibraryAlbumCreate;
use crate::mapper::library::album::album_detail_view::AlbumDetailView;
use crate::mapper::library::album::album_list_view::AlbumListView;

pub struct LibraryAlbumRepository;

impl LibraryAlbumRepository {

    pub async fn insert_library_album<'e, E>(
        exec: E,
        album_data: LibraryAlbumCreate,
    ) -> Result<LibraryAlbum, sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {

        let id: String = Uuid::new_v4().to_string();

        let album: LibraryAlbum = sqlx::query_as::<_, LibraryAlbum>(
            r#"
            INSERT INTO library_albums (
                id,
                library_id,
                artist_id,
                title,
                title_normalized,
                year,
                genre,
                cover_url,
                album_type,
                created_at,
                updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)

            ON CONFLICT(library_id, artist_id, title_normalized)
            DO UPDATE SET
                id = library_albums.id

            RETURNING *
            "#
        )
        .bind(&id)
        .bind(album_data.library_id)
        .bind(&album_data.artist_id)
        .bind(&album_data.title)
        .bind(&album_data.title_normalized)
        .bind(album_data.year)
        .bind(&album_data.genre)
        .bind(&album_data.cover_url)
        .bind(album_data.album_type.as_deref().unwrap_or("album"))
        .fetch_one(exec)
        .await?;

        Ok(album)
    }


    pub async fn update_library_album_cover<'e, E>(
            exec: E,
            new_path_str: &str,
            old_path_str: &str,
        ) -> Result<(), sqlx::Error>
        where
            E: sqlx::Executor<'e, Database = sqlx::Sqlite>
        {

        sqlx::query("UPDATE library_albums SET cover_url = ? WHERE cover_url = ?")
            .bind(new_path_str)
            .bind(old_path_str)
            .execute(exec)
            .await?;

            Ok(())
    }

    pub async fn update_cover_url_by_id<'e, E>(
        exec: E,
        album_id: &str,
        cover_url: &str,
    ) -> Result<(), sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {
        sqlx::query("UPDATE library_albums SET cover_url = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(cover_url)
            .bind(album_id)
            .execute(exec)
            .await?;
        Ok(())
    }

    pub async fn find_albums_without_cover<'e, E>(
        exec: E,
        library_id: i64,
    ) -> Result<Vec<LibraryAlbum>, sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {
        sqlx::query_as::<_, LibraryAlbum>(
            "SELECT * FROM library_albums WHERE library_id = ? AND (cover_url IS NULL OR cover_url = '') ORDER BY title"
        )
        .bind(library_id)
        .fetch_all(exec)
        .await
    }

    pub async fn find_by_normalized_title<'e, E>(
        exec: E,
        title_normalized: &str,
        library_id: i64,
        artist_id: Option<&str>,
        year: Option<i32>,
    ) -> Result<Option<LibraryAlbum>, sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {
        sqlx::query_as::<_, LibraryAlbum>(
            r#"
            SELECT * FROM library_albums
            WHERE title_normalized = ? AND library_id = ?
            AND artist_id IS ?
            AND year IS ?
            LIMIT 1
            "#
        )
        .bind(title_normalized)
        .bind(library_id)
        .bind(artist_id)
        .bind(year)
        .fetch_optional(exec)
        .await
    }

    pub async fn find_all_by_normalized_title<'e, E>(
        exec: E,
        title_normalized: &str,
        library_id: i64,
    ) -> Result<Vec<LibraryAlbum>, sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {
        sqlx::query_as::<_, LibraryAlbum>(
            "SELECT * FROM library_albums WHERE title_normalized = ? AND library_id = ?"
        )
        .bind(title_normalized)
        .bind(library_id)
        .fetch_all(exec)
        .await
    }

    pub async fn find_by_artist<'e, E>(
        exec: E,
        artist_id: &str,
        library_id: i64,
    ) -> Result<Vec<LibraryAlbum>, sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {
        sqlx::query_as::<_, LibraryAlbum>(
            "SELECT * FROM library_albums WHERE artist_id = ? AND library_id = ? ORDER BY year DESC, title"
        )
        .bind(artist_id)
        .bind(library_id)
        .fetch_all(exec)
        .await
    }

    pub async fn find_all_albums_by_library_id<'e, E>(
        exec: E,
        library_id: i64,
        missing_cover: Option<bool>,
    ) -> Result<Vec<AlbumListView>, sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {

        let where_query = if missing_cover.unwrap_or(false) {
            " AND (la.cover_url IS NULL OR la.cover_url = '')"
        } else {
            ""
        };

        let query_sql = format!(r#"
            SELECT
                -- =========================
                -- Identité Album
                -- =========================
                la.id,
                la.library_id,
                la.title,
                la.title_normalized,
                la.album_type,
                la.musicbrainz_id,
                la.artist_id,
                a.name                         AS artist,
                la.year,
                la.genre,
                la.cover_url,
                COUNT(lt.id)                   AS total_tracks,
                COALESCE(SUM(lt.duration), 0.0)  AS total_duration,
                la.notes,

                -- =========================
                -- Timestamps
                -- =========================
                la.created_at,
                la.updated_at

            FROM library_albums la
            LEFT JOIN artists a ON a.id = la.artist_id
            LEFT JOIN library_tracks lt ON lt.library_album_id = la.id
            WHERE la.library_id = ? {}
            GROUP BY la.id
            ORDER BY la.title_normalized ASC
            "#, where_query);


        let albums: Vec<AlbumListView> = sqlx::query_as::<_, AlbumListView>(&query_sql)
        .bind(library_id)
        .fetch_all(exec)
        .await?;

        Ok(albums)
    }

    pub async fn find_albums_by_artist_id<'e, E>(
        exec: E,
        library_id: i64,
        artist_id: &str,
    ) -> Result<Vec<AlbumListView>, sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {
        sqlx::query_as::<_, AlbumListView>(
            r#"
            SELECT
                la.id,
                la.library_id,
                la.title,
                la.title_normalized,
                la.album_type,
                la.musicbrainz_id,
                la.artist_id,
                a.name                         AS artist,
                la.year,
                la.genre,
                la.cover_url,
                COUNT(lt.id)                   AS total_tracks,
                COALESCE(SUM(lt.duration), 0.0)  AS total_duration,
                la.notes,
                la.created_at,
                la.updated_at
            FROM library_albums la
            LEFT JOIN artists a ON a.id = la.artist_id
            LEFT JOIN library_tracks lt ON lt.library_album_id = la.id
            WHERE la.library_id = ?1 AND la.artist_id = ?2
            GROUP BY la.id
            ORDER BY la.year DESC, la.title_normalized ASC
            "#
        )
        .bind(library_id)
        .bind(artist_id)
        .fetch_all(exec)
        .await
    }

    pub async fn find_album_by_id<'e, E>(
        exec: E,
        library_album_id: String,
    ) -> Result<AlbumDetailView, sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {

        let album: AlbumDetailView = sqlx::query_as::<_, AlbumDetailView>(
            r#"
            SELECT
                la.id,
                la.library_id,
                la.title,
                la.title_normalized,
                la.album_type,
                la.musicbrainz_id,

                la.artist_id,
                a.name AS artist,

                la.year,
                la.genre,
                la.notes,

                la.cover_url,

                -- fallback éventuel depuis cache
                MAX(lc.thumbnail_path) AS thumbnail_path,
                -- =========================
                COUNT(lt.id) AS total_tracks,
                COALESCE(SUM(lt.duration), 0) AS total_duration,
                la.created_at,
                la.updated_at

            FROM library_albums la
            LEFT JOIN artists a ON a.id = la.artist_id
            LEFT JOIN library_tracks lt ON lt.library_album_id = la.id
            LEFT JOIN library_cache lc ON lc.id = lt.cache_id

            WHERE la.id = ?

            GROUP BY la.id
            "#
        )
        .bind(library_album_id)
        .fetch_one(exec)
        .await?;

        Ok(album)
    }

    pub async fn search<'e, E>(
        exec: E,
        search_term: &str,
        limit: i64,
    ) -> Result<Vec<crate::entity::search::search_result::SearchResult>, sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {
        sqlx::query_as::<_, crate::entity::search::search_result::SearchResult>(
            r#"
            SELECT
                la.id AS id,
                'album' AS result_type,
                la.title AS title,
                a.name AS subtitle,
                la.cover_url AS thumbnail_path,
                NULL AS path,
                la.library_id AS library_id
            FROM library_albums la
            LEFT JOIN artists a ON la.artist_id = a.id
            WHERE LOWER(la.title) LIKE ?1
            LIMIT ?2
            "#
        )
        .bind(search_term)
        .bind(limit)
        .fetch_all(exec)
        .await
    }








}
