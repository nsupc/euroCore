use crate::core::error::Error;
use crate::core::state::AppState;
use crate::types::{AuthorizedUser, request, response};
use axum::extract::{Path, State};
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use axum::{Extension, Json};
use uuid::Uuid;

pub(crate) async fn create(
    State(state): State<AppState>,
    Extension(user): Extension<Option<AuthorizedUser>>,
    Json(input): Json<request::Template>,
) -> Result<impl IntoResponse, Error> {
    let user = match user {
        Some(user) => {
            if !user.claims.contains(&String::from("templates.create")) {
                return Err(Error::Unauthorized);
            }

            user
        }
        None => return Err(Error::Unauthorized),
    };

    let template = state.template_controller.create(input).await?;

    Ok((
        StatusCode::CREATED,
        [(header::LOCATION, format!("/templates/{}", &template.id))],
        Json(template),
    ))
}

pub(crate) async fn get(
    State(state): State<AppState>,
    Extension(user): Extension<Option<AuthorizedUser>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, Error> {
    let user = match user {
        Some(user) => {
            if !user.claims.contains(&String::from("templates.read")) {
                return Err(Error::Unauthorized);
            }

            user
        }
        None => return Err(Error::Unauthorized),
    };

    let template = state.template_controller.get(id).await?;

    Ok((StatusCode::CREATED, Json(template)))
}

pub(crate) async fn update(
    State(state): State<AppState>,
    Extension(user): Extension<Option<AuthorizedUser>>,
    Path(id): Path<Uuid>,
    Json(input): Json<request::Template>,
) -> Result<impl IntoResponse, Error> {
    let user = match user {
        Some(user) => {
            if !user.claims.contains(&String::from("templates.update")) {
                return Err(Error::Unauthorized);
            }

            user
        }
        None => return Err(Error::Unauthorized),
    };

    state.template_controller.update(id, input).await?;

    Ok(StatusCode::NO_CONTENT)
}
