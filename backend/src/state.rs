use std::sync::Arc;
use sea_orm::{Database, DatabaseConnection};

use crate::{config::AppConfig, repository::{data::DataRepositoryImpl, effects::EffectsRepositoryImpl, friends::FriendRepositoryImpl, herb_quest::HerbQuestRepositoryImpl, inventory::InventoryRepositoryImpl, mail::MailRepositoryImpl, missions::MissionRepositoryImpl, stats::StatsRepositoryImpl, user::UserRepositoryImpl}, service::{
    authentication::{AuthenticationService, AuthenticationServiceImpl}, data::{DataService, DataServiceImpl}, effects::{EffectsService, EffectsServiceImpl}, fishmarket::{FishmarketService, FishmarketServiceImpl}, friends::{FriendService, FriendServiceImpl}, herb_quest::{HerbQuestService, HerbQuestServiceImpl}, inventory::{InventoryService, InventoryServiceImpl}, mail::{MailService, MailServiceImpl}, missions::{MissionService, MissionServiceImpl}, shop::{ShopService, ShopServiceImpl}, stats::{StatsService, StatsServiceImpl}, trading::{TradeService, TradeServiceImpl}, user::{UserService, UserServiceImpl},
}};

pub struct AppState {
    pub db: DatabaseConnection,

    pub auth: Arc<dyn AuthenticationService>,
    pub data: Arc<dyn DataService>,
    pub effects: Arc<dyn EffectsService>,
    pub fishmarket: Arc<dyn FishmarketService>,
    pub friend: Arc<dyn FriendService>,
    pub herb: Arc<dyn HerbQuestService>,
    pub inventory: Arc<dyn InventoryService>,
    pub mail: Arc<dyn MailService>,
    pub mission: Arc<dyn MissionService>,
    pub shop: Arc<dyn ShopService>,
    pub stats: Arc<dyn StatsService>,
    pub trade: Arc<dyn TradeService>,
    pub user: Arc<dyn UserService>,
}

impl AppState {
    pub async fn new(config: &AppConfig) -> Result<Self, rocket::Error> {
        let db = Database::connect(&config.database_url)
            .await
            .expect("Failed to create SeaORM database connection");

        // Repositories
        let data_repository = DataRepositoryImpl::new();
        let effects_repository = EffectsRepositoryImpl::new();
        let friends_repository = FriendRepositoryImpl::new();
        let herb_repository = HerbQuestRepositoryImpl::new();
        let inventory_repository = InventoryRepositoryImpl::new();
        let mail_repository = MailRepositoryImpl::new();
        let mission_repository: MissionRepositoryImpl = MissionRepositoryImpl::new();
        let stats_repository = StatsRepositoryImpl::new();
        let user_repository = UserRepositoryImpl::new();

        // Services
        let user = Arc::new(UserServiceImpl::new(
            db.clone(),
            user_repository.clone(),
            stats_repository.clone(),
            inventory_repository.clone(),
            config.secret_key.clone(),
        ));

        let auth = Arc::new(AuthenticationServiceImpl::new(
            db.clone(),
            user_repository.clone(),
            config.secret_key.clone(),
        ));

        let data = Arc::new(DataServiceImpl::new(
            db.clone(),
            data_repository.clone(),
        ));

        let fishmarket = Arc::new(FishmarketServiceImpl::new(
            db.clone(),
            stats_repository.clone(),
            inventory_repository.clone(),
        ));

        let friend = Arc::new(FriendServiceImpl::new(
            db.clone(),
            friends_repository.clone(),
        ));

        let stats = Arc::new(StatsServiceImpl::new(
            db.clone(),
            stats_repository.clone(),
        ));

        let mail = Arc::new(MailServiceImpl::new(
            db.clone(),
            mail_repository.clone(),
        ));

        let mission: Arc<dyn MissionService> = Arc::new(MissionServiceImpl::new(
            db.clone(),
            mission_repository.clone(),
        ));

        let inventory = Arc::new(InventoryServiceImpl::new(
            db.clone(),
            inventory_repository.clone(),
        ));

        let effects = Arc::new(EffectsServiceImpl::new(
            db.clone(),
            effects_repository.clone(),
        ));

        let shop = Arc::new(ShopServiceImpl::new(
            db.clone(),
            stats_repository.clone(),
            inventory_repository.clone(),
        ));

        let trade = Arc::new(TradeServiceImpl::new(
            db.clone(),
            inventory_repository.clone(),
            stats_repository.clone(),
        ));

        let herb = Arc::new(HerbQuestServiceImpl::new(
            db.clone(),
            herb_repository.clone(),
            inventory_repository.clone(),
            stats_repository.clone(),
        ));

        Ok(Self {
            db,
            auth,
            data,
            effects,
            fishmarket,
            friend,
            herb,
            inventory,
            mail,
            mission,
            shop,
            stats,
            trade,
            user,
        })
    }
}