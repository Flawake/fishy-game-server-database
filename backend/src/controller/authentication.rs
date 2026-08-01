use crate::domain::LoginResponse;
use rocket::http::Status;
use rocket::post;
use rocket::response::status;
use rocket::routes;
use rocket::serde::json::Json;
use rocket::State;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct LoginRequest {
    username: String,
    password: String,
}

// Return type should later be CreateUserRepsonse
#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse, content_type = "application/json"),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Recieve a jwt when creditials are valid.",
    operation_id = "login",
    tag = "Authentication"
)]
#[post("/login", data = "<payload>")]
async fn login(
    payload: Json<LoginRequest>,
    state: &State<AppState>,
) -> Result<Json<LoginResponse>, status::Custom<String>> {
    match state.auth
        .login(payload.username.clone(), payload.password.clone())
        .await
    {
        Ok(jwt) => match jwt {
            Some(res) => Ok(Json(res)),
            None => Err(status::Custom(
                Status::InternalServerError,
                "Access denied".to_string(),
            )),
        },
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Internal server error".to_string(),
        )),
    }
}

pub fn authentication_routes() -> Vec<rocket::Route> {
    routes![login]
}
