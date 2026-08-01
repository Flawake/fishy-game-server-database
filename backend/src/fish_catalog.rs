//! Fish catalog + Herb quest generation.
//!
//! The list of fishes is embedded from `fishes.json` at compile time (via `include_str!`)
//! so it is always available at runtime, including in the slim Docker image which only
//! ships the compiled binary.
//!
//! Herb's daily quest is generated here: pick a random area, then draw 2-4 individual
//! fish from that area's Common/Uncommon pool (repeats allowed, so "3 the same + 1 other"
//! or "2 and 2" are all possible), and reward 1 coin per requested fish.

use std::collections::HashMap;
use std::sync::OnceLock;

use rand::seq::SliceRandom;
use rand::Rng;
use serde::Deserialize;

/// The fish catalog, embedded at compile time.
const FISHES_JSON: &str = include_str!("../fishes.json");

#[derive(Debug, Deserialize)]
struct FishesFile {
    fishes: Vec<FishDef>,
}

#[derive(Debug, Deserialize)]
struct FishDef {
    id: i32,
    rarity: String,
    locations: Vec<String>,
}

// Area enum ids, mirrored from the game (Assets/Core/World/Area.cs).
pub const AREA_FUSETA_BEACH: i32 = 2;
pub const AREA_SELVA_BANDEIRA: i32 = 3;
pub const AREA_GREENFIELDS: i32 = 4;

/// Areas Herb can appear in (and pick quest fishes from).
pub const HERB_AREAS: [i32; 3] = [AREA_FUSETA_BEACH, AREA_SELVA_BANDEIRA, AREA_GREENFIELDS];

/// Maps a `fishes.json` location name to the game's Area enum id.
fn location_to_area(location: &str) -> Option<i32> {
    match location {
        "Beach" => Some(AREA_FUSETA_BEACH),
        "Tropical" => Some(AREA_SELVA_BANDEIRA),
        "Greenfields" => Some(AREA_GREENFIELDS),
        _ => None,
    }
}

/// Only 1/2 star fish (Common/Uncommon) are used for Herb quests.
fn is_eligible_rarity(rarity: &str) -> bool {
    matches!(rarity, "COMMON" | "UNCOMMON")
}

/// area_id -> eligible (Common/Uncommon) fish ids catchable in that area.
/// Parsed once from the embedded catalog and cached.
fn eligible_pools() -> &'static HashMap<i32, Vec<i32>> {
    static POOLS: OnceLock<HashMap<i32, Vec<i32>>> = OnceLock::new();
    POOLS.get_or_init(|| {
        let parsed: FishesFile =
            serde_json::from_str(FISHES_JSON).expect("embedded fishes.json must be valid JSON");

        let mut pools: HashMap<i32, Vec<i32>> = HashMap::new();
        for fish in parsed.fishes {
            if !is_eligible_rarity(&fish.rarity) {
                continue;
            }
            for location in &fish.locations {
                if let Some(area_id) = location_to_area(location) {
                    pools.entry(area_id).or_default().push(fish.id);
                }
            }
        }
        pools
    })
}

/// A freshly generated Herb quest, before it is persisted.
pub struct GeneratedQuest {
    pub area_id: i32,
    pub reward_coins: i32,
    /// (fish_id, amount) pairs the quest asks for.
    pub fishes: Vec<(i32, i32)>,
}

/// Generates a new random Herb quest.
///
/// Picks a random usable area, draws a total of 2-4 individual fish from that area's
/// Common/Uncommon pool (with repeats), groups them into (fish_id, amount) entries and
/// rewards 1 coin per requested fish.
pub fn generate_quest() -> GeneratedQuest {
    let pools = eligible_pools();
    let mut rng = rand::thread_rng();

    // Defensive: only areas that actually have an eligible fish can host a quest.
    let usable_areas: Vec<i32> = HERB_AREAS
        .iter()
        .copied()
        .filter(|area| pools.get(area).is_some_and(|pool| !pool.is_empty()))
        .collect();

    let area_id = *usable_areas
        .choose(&mut rng)
        .expect("at least one Herb area must have eligible fish");
    let pool = &pools[&area_id];

    let total_fish: i32 = rng.gen_range(2..=4);

    let mut counts: HashMap<i32, i32> = HashMap::new();
    for _ in 0..total_fish {
        let fish_id = *pool
            .choose(&mut rng)
            .expect("pool was checked to be non-empty");
        *counts.entry(fish_id).or_insert(0) += 1;
    }

    GeneratedQuest {
        area_id,
        reward_coins: total_fish, // 1 coin per requested fish
        fishes: counts.into_iter().collect(),
    }
}
