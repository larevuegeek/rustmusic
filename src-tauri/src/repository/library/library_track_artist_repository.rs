use sqlx::Result;
use crate::entity::library::library_track_artist::{
    LibraryTrackArtist,
    LibraryTrackArtistCreate,
};

pub struct LibraryTrackArtistRepository;

impl LibraryTrackArtistRepository {

    // ✅ Upsert (évite doublon UNIQUE(library_track_id, artist_id))
    pub async fn insert_library_track_artist<'e, E>(
        exec: E,
        data: LibraryTrackArtistCreate,
    ) -> Result<LibraryTrackArtist>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {
        let record = sqlx::query_as::<_, LibraryTrackArtist>(
            r#"
            INSERT INTO library_track_artists (
                library_id,
                library_track_id,
                artist_id
            )
            VALUES (?, ?, ?)
            ON CONFLICT(library_track_id, artist_id)
            DO UPDATE SET
                library_id = excluded.library_id
            RETURNING *
            "#
        )
        .bind(data.library_id)
        .bind(data.library_track_id)
        .bind(data.artist_id)
        .fetch_one(exec)
        .await?;

        Ok(record)
    }

    // ✅ Trouver tous les artistes d'un track
    pub async fn find_by_track_id<'e, E>(
        exec: E,
        library_track_id: &str,
    ) -> Result<Vec<LibraryTrackArtist>>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {
        let records = sqlx::query_as::<_, LibraryTrackArtist>(
            r#"
            SELECT *
            FROM library_track_artists
            WHERE library_track_id = ?
            "#
        )
        .bind(library_track_id)
        .fetch_all(exec)
        .await?;

        Ok(records)
    }

    // ✅ Trouver tous les tracks d'un artiste
    pub async fn find_by_artist_id<'e, E>(
        exec: E,
        artist_id: &str,
    ) -> Result<Vec<LibraryTrackArtist>>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {
        let records = sqlx::query_as::<_, LibraryTrackArtist>(
            r#"
            SELECT *
            FROM library_track_artists
            WHERE artist_id = ?
            "#
        )
        .bind(artist_id)
        .fetch_all(exec)
        .await?;

        Ok(records)
    }

    // ✅ Delete par track
    pub async fn delete_by_track_id<'e, E>(
        exec: E,
        library_track_id: &str,
    ) -> Result<()>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {
        sqlx::query(
            r#"
            DELETE FROM library_track_artists
            WHERE library_track_id = ?
            "#
        )
        .bind(library_track_id)
        .execute(exec)
        .await?;

        Ok(())
    }

    // ✅ Delete relation spécifique
    pub async fn delete_relation<'e, E>(
        exec: E,
        library_track_id: &str,
        artist_id: &str,
    ) -> Result<()>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {
        sqlx::query(
            r#"
            DELETE FROM library_track_artists
            WHERE library_track_id = ?
            AND artist_id = ?
            "#
        )
        .bind(library_track_id)
        .bind(artist_id)
        .execute(exec)
        .await?;

        Ok(())
    }
}
