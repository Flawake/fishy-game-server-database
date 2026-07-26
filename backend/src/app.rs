use chrono::Utc;
use rocket::Build;
use rocket::Rocket;
use rocket_cors::Cors;
use rocket_cors::AllowedOrigins;
use rocket_cors::{AllowedHeaders, CorsOptions};
use tokio::time;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::controller::authentication::authentication_routes;
use crate::controller::data::data_routes;
use crate::controller::effects::effect_routes;
use crate::controller::fishmarket::fishmarket_routes;
use crate::controller::friends::friend_routes;
use crate::controller::herb_quest::herb_quest_routes;
use crate::controller::inventory::inventory_routes;
use crate::controller::mail::mail_routes;
use crate::controller::missions::mission_routes;
use crate::controller::shop::shop_routes;
use crate::controller::stats::stats_routes;
use crate::controller::trading::trade_routes;
use crate::controller::user::user_routes;
use crate::docs::ApiDoc;
use crate::{config::AppConfig, state::AppState};


fn start_background_tasks(state: &AppState) {
    let herb_quest_service = state.herb.clone();

    tokio::spawn(async move {
        if let Err(e) = herb_quest_service.ensure_quest_on_boot().await {
            eprintln!("[HerbQuest] Failed to ensure a quest on boot: {:?}", e);
        }

        loop {
            let next = crate::service::herb_quest::next_rollover(Utc::now());

            let sleep_duration = (next - Utc::now())
                .to_std()
                .unwrap_or(std::time::Duration::from_secs(0));

            time::sleep(sleep_duration).await;

            if let Err(e) = herb_quest_service.roll_over_quest().await {
                eprintln!("[HerbQuest] Failed to roll over quest: {:?}", e);
            }
        }
    });
}

pub fn create_cors() -> Cors {
    // Alow request from any origin.
    // You should customize this if you want to make your backend more secure.
    CorsOptions {
        allowed_origins: AllowedOrigins::all(), // Allow all origins
        allowed_headers: AllowedHeaders::some(&["Authorization", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Failed to create CORS configuration")
}

pub async fn build(config: AppConfig) -> Result<Rocket<Build>, rocket::Error> {
    let state = AppState::new(&config).await?;
    let cors = create_cors();

    start_background_tasks(&state);

    Ok(
        rocket::custom(config.rocket)
            .manage(state)
            .mount(
                "/",
                SwaggerUi::new("/docs/<_..>")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .mount("/account", user_routes())
            .mount("/auth", authentication_routes())
            .mount("/stats", stats_routes())
            .mount("/mail", mail_routes())
            .mount("/missions", mission_routes())
            .mount("/inventory", inventory_routes())
            .mount("/data", data_routes())
            .mount("/fish_market", fishmarket_routes())
            .mount("/friend", friend_routes())
            .mount("/effects", effect_routes())
            .mount("/shop", shop_routes())
            .mount("/trade", trade_routes())
            .mount("/herb_quest", herb_quest_routes())
            .attach(cors)
    )
}