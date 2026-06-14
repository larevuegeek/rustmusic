//! Media streaming handlers.
//!
//! `GET /media/{id}` streams an audio file with proper `Content-Type` and
//! HTTP `Range:` request support (essential for seek from the amp).
//! `GET /cover/{id}` streams cover images. The `id` is prefixed with the
//! cover kind : `track:<uuid>`, `album:<uuid>`, `artist:<uuid>`.
//!
//! Heavy lifting (range parsing, last-modified, mime detection) is delegated
//! to `tower-http::services::ServeFile` so we don't reinvent HTTP.

use axum::body::Body;
use axum::extract::{Path, Request, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tower::ServiceExt;
use tower_http::services::ServeFile;

use crate::core::dlna_server::http::router::AppState;
use crate::core::dlna_server::library::provider::CoverKind;

/// `GET /media/{id}` — stream an audio file with full HTTP range support.
pub async fn stream_media(
    Path(id): Path<String>,
    State(state): State<AppState>,
    request: Request<Body>,
) -> Response {
    let path = match state.provider.track_path(&id).await {
        Ok(p) => p,
        Err(e) => {
            log::warn!("stream_media : track_path({}) failed : {}", id, e);
            return (
                StatusCode::NOT_FOUND,
                format!("Track {} not found", id),
            )
                .into_response();
        }
    };

    log::debug!(
        "Streaming track id={} ({} bytes, range={})",
        id,
        path.metadata().map(|m| m.len()).unwrap_or(0),
        request
            .headers()
            .get("range")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("none")
    );

    match ServeFile::new(&path).oneshot(request).await {
        Ok(resp) => resp.map(Body::new).into_response(),
        Err(e) => {
            log::error!("ServeFile error for {:?} : {}", path, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("stream error: {}", e),
            )
                .into_response()
        }
    }
}

/// `GET /cover/{id}` — stream a cover image.
///
/// `id` must be prefixed : `track:<uuid>`, `album:<uuid>`, or `artist:<uuid>`.
pub async fn stream_cover(
    Path(id): Path<String>,
    State(state): State<AppState>,
    request: Request<Body>,
) -> Response {
    let Some((kind, real_id)) = parse_cover_id(&id) else {
        return (StatusCode::BAD_REQUEST, "expected <kind>:<id>").into_response();
    };

    let path = match state.provider.cover_path(kind, real_id).await {
        Ok(p) => p,
        Err(_) => return (StatusCode::NOT_FOUND, "cover not found").into_response(),
    };

    match ServeFile::new(&path).oneshot(request).await {
        Ok(resp) => resp.map(Body::new).into_response(),
        Err(e) => {
            log::error!("cover ServeFile error for {:?} : {}", path, e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("stream error: {}", e)).into_response()
        }
    }
}

fn parse_cover_id(id: &str) -> Option<(CoverKind, &str)> {
    let (prefix, rest) = id.split_once(':')?;
    let kind = match prefix {
        "track" => CoverKind::Track,
        "album" => CoverKind::Album,
        "artist" => CoverKind::Artist,
        _ => return None,
    };
    Some((kind, rest))
}
