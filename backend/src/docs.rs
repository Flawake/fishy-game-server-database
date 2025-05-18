use crate::controller::authentication::*;
use crate::controller::data::*;
use crate::controller::inventory::*;
use crate::controller::mail::*;
use crate::controller::stats::*;
use crate::controller::user::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
    create_user,
    get_user,
    login,

    select_item,
    add_xp,
    change_bucks,
    change_coins,
    add_playtime,
    add_fish,

    create_mail,
    delete_mail,
    change_read_state,
    change_archive_state,

    add_item,
    increase_item,
    destroy_item,
    degrade_item,

    retreive_player_data,
))]
pub struct ApiDoc;
