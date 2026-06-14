use tauri::State;

use crate::{
    entity::queue::queue_track::QueueTrack, 
    mapper::queue::queue_state_view::QueueStateView, 
    repository::queue::{
        queue_state_repository::QueueStateRepository, 
        queue_track_repository::QueueTrackRepository
    }, 
    state::AppState
};

#[tauri::command]
pub async fn get_queue(
    state: State<'_, AppState>, 
    profil_id: i64
) -> Result<QueueStateView, String> {

    let queue_state_view: QueueStateView = QueueStateRepository::get_queue(&state.pool, profil_id)
            .await
            .map_err(|e| format!("Failed to get queue state : {}", e))?;

    Ok(queue_state_view)
}

#[tauri::command]
pub async fn add_queue_track(
    state: State<'_, AppState>, 
    profil_id: i64,
    payload: QueueTrack
) -> Result<(), String> {

    QueueTrackRepository::add_track(&state.pool, profil_id, payload)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn update_queue_state_index(
    state: State<'_, AppState>, 
    profil_id: i64,
    current_index: i32
) -> Result<(), String> {

    QueueStateRepository::update_current_index(&state.pool, profil_id, current_index)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn update_queue_state_shuffled(
    state: State<'_, AppState>, 
    profil_id: i64,
    is_shuffled: bool
) -> Result<(), String> {

    QueueStateRepository::update_is_shuffled(&state.pool, profil_id, is_shuffled)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn update_queue_state_repeat_mode(
    state: State<'_, AppState>, 
    profil_id: i64,
    repeat_mode: &str
) -> Result<(), String> {

    QueueStateRepository::update_repeat_mode(&state.pool, profil_id, repeat_mode)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn remove_queue_track(
    state: State<'_, AppState>, 
    profil_id: i64,
    queue_id: String
) -> Result<(), String> {

    QueueTrackRepository::remove_track(&state.pool, profil_id, &queue_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn replace_queue_tracks(
    state: State<'_, AppState>, 
    profil_id: i64,
    payload: Vec<QueueTrack>
) -> Result<(), String> {
    QueueTrackRepository::replace_all(&state.pool, profil_id, payload)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn clear_queue(
    state: State<'_, AppState>, 
    profil_id: i64
) -> Result<(), String> {

    QueueStateRepository::clear_queue(&state.pool, profil_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}