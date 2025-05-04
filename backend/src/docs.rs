use crate::controller::authentication::*;
use crate::controller::user::*;
use crate::controller::stats::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(create_user, get_user, login, add_xp, change_bucks, change_coins, add_playtime))]
pub struct ApiDoc;
