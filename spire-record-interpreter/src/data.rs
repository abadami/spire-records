use serde::{Deserialize, Serialize};

pub enum Acts {
    Overgrowth,
    Underdocks,
    Hive,
    Glory
}

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub floor_added_to_deck: Option<u32>,
    pub id: String
}

#[derive(Serialize, Deserialize)]
pub struct CardChoice {
    pub card: Card,
    pub was_picked: bool
}

#[derive(Serialize, Deserialize)]
pub struct AncientTitle {
    pub key: String,
    pub table: String
}

#[derive(Serialize, Deserialize)]
pub struct AncientChoice {
    pub text_key: String,
    pub title: AncientTitle,
    pub was_chosen: bool
}

#[derive(Serialize, Deserialize)]
pub struct PlayerStats {
    pub card_choices: Option<Vec<CardChoice>>,
    pub ancient_choices: Option<Vec<AncientChoice>>,
    pub current_gold: u32,
    pub current_hp: u32,
    pub damage_taken: u32,
    pub gold_gained: u32,
    pub gold_lost: u32,
    pub gold_stolen: u32,
    pub hp_healed: u32,
    pub max_hp: u32,
    pub max_hp_gained: u32,
    pub max_hp_lost: u32,
    pub player_id: u32
}

#[derive(Serialize, Deserialize)]
pub struct Room {
    model_id: String,
    pub room_type: String,
    pub turns_taken: u32,
    pub monster_ids: Option<Vec<String>>
}

#[derive(Serialize, Deserialize)]
pub struct MapPoint {
    pub map_point_type: String,
    pub player_stats: Vec<PlayerStats>,
    pub room: Option<Vec<Room>>,
}

#[derive(Serialize, Deserialize)]
pub struct Badge {
    pub id: String,
    rarity: String,
}

#[derive(Serialize, Deserialize)]
pub struct LootItem {
    pub id: String,
    pub floor_added_to_deck: u32,
    pub current_upgrade_level: Option<u32>,
    pub slot_index: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub badge: Option<Vec<Badge>>,
    pub character: String,
    pub deck: Vec<LootItem>,
    pub relics: Vec<LootItem>,
    pub potion: Option<Vec<LootItem>>,
    pub max_potion_slot_count: u32,
    pub id: u32
}

#[derive(Serialize, Deserialize)]
pub struct Run {
    pub acts: Vec<String>,
    pub ascension: u32,
    build_id: String,
    pub game_mod: Option<String>,
    pub killed_by_encounter: String,
    pub killed_by_event: String,
    pub map_point_history: Vec<Vec<MapPoint>>,
    pub modifiers: Vec<String>,
    pub platform_type: String,
    pub players: Vec<Player>,
    pub run_time: u32,
    pub schema_version: u32,
    pub seed: String,
    pub start_time: u32,
    pub was_abandoned: bool,
    pub win: bool
}