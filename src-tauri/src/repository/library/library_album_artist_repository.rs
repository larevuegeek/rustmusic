use uuid::Uuid;

use crate::entity::library::library_album_artist::{
    LibraryAlbumArtist,
    LibraryAlbumArtistCreate,
};

pub struct LibraryAlbumArtistRepository;

impl LibraryAlbumArtistRepository {

    // ----------------------------------------------------
    // INSERT
    // ----------------------------------------------------
    pub async fn insert_library_album_artist<'e, E>(
        exec: E,
        data: LibraryAlbumArtistCreate,
    ) -> Result<LibraryAlbumArtist, sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {

        let id: String = Uuid::new_v4().to_string();

        let result: LibraryAlbumArtist = sqlx::query_as::<_, LibraryAlbumArtist>(
            r#"
            INSERT INTO library_album_artists (
                id,
                library_id,
                library_album_id,
                artist_id
            )
            VALUES (?1, ?2, ?3, ?4)

            ON CONFLICT(library_album_id, artist_id)
            DO UPDATE SET
                library_id = excluded.library_id

            RETURNING *
            "#
        )
        .bind(id)
        .bind(data.library_id)
        .bind(data.library_album_id)
        .bind(data.artist_id)
        .fetch_one(exec)
        .await?;

        Ok(result)
    }

    pub async fn find_album_artist_by_ids<'e, E>(
        exec: E,
        library_id: &i64,
        library_album_id: &String,
        artist_id: &String,
    ) -> Result<Option<LibraryAlbumArtist>, sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {

        let result: Option<LibraryAlbumArtist> = sqlx::query_as::<_, LibraryAlbumArtist>(
            r#"
            SELECT *
            FROM library_album_artists
            WHERE library_id = ?1
            AND library_album_id = ?2
            AND artist_id = ?3
            LIMIT 1
            "#
        )
        .bind(library_id)
        .bind(library_album_id)
        .bind(artist_id)
        .fetch_optional(exec)
        .await?;

        Ok(result)
    }

    // ----------------------------------------------------
    // FIND BY ALBUM
    // ----------------------------------------------------
    pub async fn find_by_album<'e, E>(
        exec: E,
        library_id: i64,
        album_id: &str,
    ) -> Result<Vec<LibraryAlbumArtist>, sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {
        let results = sqlx::query_as::<_, LibraryAlbumArtist>(
            r#"
            SELECT *
            FROM library_album_artists
            WHERE library_id = ?1
              AND library_album_id = ?2
            ORDER BY role ASC
            "#
        )
        .bind(library_id)
        .bind(album_id)
        .fetch_all(exec)
        .await?;

        Ok(results)
    }

    // ----------------------------------------------------
    // FIND BY ARTIST
    // ----------------------------------------------------
    pub async fn find_by_artist<'e, E>(
        exec: E,
        library_id: i64,
        artist_id: &str,
    ) -> Result<Vec<LibraryAlbumArtist>, sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {
        let results = sqlx::query_as::<_, LibraryAlbumArtist>(
            r#"
            SELECT *
            FROM library_album_artists
            WHERE library_id = ?1
              AND artist_id = ?2
            "#
        )
        .bind(library_id)
        .bind(artist_id)
        .fetch_all(exec)
        .await?;

        Ok(results)
    }

    // ----------------------------------------------------
    // DELETE ONE
    // ----------------------------------------------------

    pub async fn delete<'e, E>(
        exec: E,
        library_id: i64,
        album_id: &str,
        artist_id: &str,
    ) -> Result<(), sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {
        sqlx::query(
            r#"
            DELETE FROM library_album_artists
            WHERE library_id = ?1
              AND library_album_id = ?2
              AND artist_id = ?3
            "#
        )
        .bind(library_id)
        .bind(album_id)
        .bind(artist_id)
        .execute(exec)
        .await?;

        Ok(())
    }

    // ----------------------------------------------------
    // DELETE ALL BY ALBUM
    // ----------------------------------------------------

    pub async fn delete_by_album<'e, E>(
        exec: E,
        library_id: i64,
        album_id: &str,
    ) -> Result<(), sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {
        sqlx::query(
            r#"
            DELETE FROM library_album_artists
            WHERE library_id = ?1
              AND library_album_id = ?2
            "#
        )
        .bind(library_id)
        .bind(album_id)
        .execute(exec)
        .await?;

        Ok(())
    }
}
