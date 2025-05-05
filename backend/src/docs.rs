use crate::controller::authentication::*;
use crate::controller::stats::*;
use crate::controller::user::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
    create_user,
    get_user,
    login,
    add_xp,
    change_bucks,
    change_coins,
    add_playtime,
    add_fish
))]
pub struct ApiDoc;
