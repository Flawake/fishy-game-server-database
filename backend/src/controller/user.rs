use crate::domain::LoginResponse;
use rocket::post;
use rocket::routes;
use rocket::serde::json::Json;
use rocket::State;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use crate::state::AppState;

/// Request body for creating a user.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct CreateUserRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

/// Request body for changing a password.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct ChangePasswordRequest {
    pub username: String,
    pub new_password: String,
}

/// Request body for requesting a players username.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct RetrieveUsernameRequest {
    pub email: String,
}

// Utoipa is the crate that generates swagger documentation for your endpoints.
// The documentation for each endpoint is combined in docs.rs
// Make sure to add your endpoint in docs.rs when you write new endpoints.
#[utoipa::path(
    post,
    path = "/account/register",
    request_body = CreateUserRequest,
    responses(
        // create_user returns Json<LoginResponse> (code + jwt), not a bool.
        (status = 200, description = "User created successfully", body = LoginResponse, content_type = "application/json"),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Creates a user. The email and username should be unique.",
    operation_id = "register",
    tag = "Users"
)]
#[post("/register", data = "<payload>")]
async fn create_user(
    payload: Json<CreateUserRequest>,
    state: &State<AppState>,
) -> Json<LoginResponse> {
    match state.user
        .create(
            payload.username.clone(),
            payload.email.clone(),
            payload.password.clone(),
        )
        .await
    {
        Ok(res) => Json(res),
        Err(_) => Json(LoginResponse {
            code: 401,
            jwt: String::from(""),
        }),
    }
}

#[utoipa::path(
    post,
    path = "/account/retrieve_username",
    request_body = RetrieveUsernameRequest,
    responses(
        (status = 200, description = "Username send successfull", body = bool, content_type = "application/json"),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Sends the username of the account the email belongs to to the mail address",
    operation_id = "retrieve_username",
    tag = "Users"
)]
#[post("/retrieve_username", data = "<payload>")]
async fn retrieve_username(
    payload: Json<RetrieveUsernameRequest>,
    state: &State<AppState>,
) -> Json<bool> {
    match state.user.retrieve_username(payload.email.clone()).await {
        Ok(res) => Json(res),
        Err(_) => Json(false),
    }
}

#[utoipa::path(
    post,
    path = "/account/change_password",
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Changed password", body = bool, content_type = "application/json"),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Changes a users password",
    operation_id = "change_password",
    tag = "Users"
)]
#[post("/change_password", data = "<payload>")]
async fn change_password(
    payload: Json<ChangePasswordRequest>,
    state: &State<AppState>,
) -> Json<bool> {
    // TODO: We need a way to verify a user is actually changing the password of it's own account
    match state.user
        .change_password(payload.username.clone(), payload.new_password.clone())
        .await
    {
        Ok(res) => Json(res),
        Err(_) => Json(false),
    }
}

// Combine all the user routes.
pub fn user_routes() -> Vec<rocket::Route> {
    routes![create_user, retrieve_username, change_password]
}
