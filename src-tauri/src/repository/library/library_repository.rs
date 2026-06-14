use crate::entity::library::library::{Library, LibraryCreate};

pub struct LibraryRepository;

impl LibraryRepository {

    pub async fn insert_library<'e, E>(
        exec: E,
        library: &LibraryCreate,
    ) -> Result<Library, sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {

        let inserted: Library = sqlx::query_as::<_, Library>(
            r#"
            INSERT INTO library (
                profil_id,
                name,
                description,
                position,
                created_at,
                updated_at
            )
            VALUES (
                ?1, ?2, ?3, ?4,
                CURRENT_TIMESTAMP,
                CURRENT_TIMESTAMP
            )
            RETURNING
                id,
                profil_id,
                name,
                description,
                cover,
                position,
                total_tracks,
                total_albums,
                total_artists,
                created_at,
                updated_at
            "#
        )
        .bind(library.profil_id)
        .bind(&library.name)
        .bind(&library.description)
        .bind(0)
        .fetch_one(exec)
        .await?;

        Ok(inserted)
    }

    pub async fn find_library_by_id<'e, E>(
        exec: E,
        library_id: i64,
    ) -> Result<Library, sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {

        let library: Library = sqlx::query_as::<_, Library>(
            r#"
            SELECT
                id,
                profil_id,
                name,
                description,
                cover,
                position,
                total_tracks,
                total_albums,
                total_artists,
                created_at,
                updated_at
            FROM library
            WHERE id = ?
            "#
        )
        .bind(library_id)
        .fetch_one(exec)
        .await?;

        Ok(library)
    }

    /// All libraries across all profiles, ordered by position then id.
    /// Used by features that span profiles (e.g. DLNA server exposing every lib).
    pub async fn find_all<'e, E>(exec: E) -> Result<Vec<Library>, sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {
        sqlx::query_as::<_, Library>(
            r#"
            SELECT
                id,
                profil_id,
                name,
                description,
                cover,
                position,
                total_tracks,
                total_albums,
                total_artists,
                created_at,
                updated_at
            FROM library
            ORDER BY position ASC, id ASC
            "#
        )
        .fetch_all(exec)
        .await
    }

    pub async fn find_libraries_by_profil_id<'e, E>(
        exec: E,
        profil_id: i64,
    ) -> Result<Vec<Library>, sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {

        let libraries: Vec<Library> = sqlx::query_as::<_, Library>(
            r#"
            SELECT
                id,
                profil_id,
                name,
                description,
                cover,
                position,
                total_tracks,
                total_albums,
                total_artists,
                created_at,
                updated_at
            FROM library
            WHERE profil_id = ?
            ORDER BY position ASC
            "#
        )
        .bind(profil_id)
        .fetch_all(exec)
        .await?;

        Ok(libraries)
    }

    pub async fn remove_library<'e, E>(
        exec: E,
        library_id: i64,
    ) -> Result<(), sqlx::Error>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>
    {

        sqlx::query(
            r#"
            DELETE FROM library
            WHERE id = ?
            "#
        )
        .bind(library_id)
        .execute(exec)
        .await?;

        Ok(())
    }
}
