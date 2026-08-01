use crate::controller::authentication::*;
use crate::controller::data::*;
use crate::controller::effects::*;
use crate::controller::fishmarket::*;
use crate::controller::friends::*;
use crate::controller::herb_quest::*;
use crate::controller::inventory::*;
use crate::controller::mail::*;
use crate::controller::missions::*;
use crate::controller::shop::*;
use crate::controller::stats::*;
use crate::controller::trading::*;
use crate::controller::user::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Users
        create_user,
        retrieve_username,
        change_password,
        // Authentication
        login,
        // UserData
        retrieve_player_data,
        // Inventory
        add_or_update_item,
        destroy_item,
        // Shop
        buy_item,
        // FishMarket
        sell_fishes,
        // Trading
        commit_trade,
        // Stats
        add_playtime,
        add_fish,
        select_item,
        // Mails
        create_mail,
        delete_mail,
        change_read_state,
        change_archive_state,
        // Friends
        remove_friend,
        add_friend_request,
        handle_friend_request,
        // Effects
        add_effect,
        remove_expired_effects,
        cleanup_all_expired_effects,
        // HerbQuest
        current_daily_quest,
        complete_daily_quest,
        accept_daily_quest,
        // Missions
        start_mission,
        progress_mission,
        complete_mission,
    ),
    tags(
        (name = "Users", description = "Account creation and account management."),
        (name = "Authentication", description = "Credential exchange for a JWT."),
        (name = "UserData", description = "Bulk read of everything belonging to one player."),
        (name = "Inventory", description = "Per-player item storage."),
        (name = "Shop", description = "Buying items for coins or bucks."),
        (name = "FishMarket", description = "Selling caught fish."),
        (name = "Trading", description = "Player-to-player item and currency trades."),
        (name = "Stats", description = "Playtime, caught fish and equipped item stats."),
        (name = "Mails", description = "In-game mailbox."),
        (name = "Friends", description = "Friend list and friend requests."),
        (name = "Effects", description = "Timed effects applied to a player."),
        (name = "HerbQuest", description = "Herb's global daily quest."),
        (name = "Missions", description = "Per-player mission progress."),
    ),
)]
pub struct ApiDoc;
