use crate::controller::authentication::*;
use crate::controller::mail::*;
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
    add_fish,

    create_mail,
    delete_mail,
    change_read_state,
    change_archive_state
))]
pub struct ApiDoc;
