use serde::{Deserialize, Serialize};

enum Acts {
    Overgrowth,
    Underdocks,
    Hive,
    Glory
}

#[derive(Serialize, Deserialize)]
struct Card {
    floor_added_to_deck: u32,
    id: String
}

#[derive(Serialize, Deserialize)]
struct CardChoice {
    card: Card,
    was_picked: bool
}

#[derive(Serialize, Deserialize)]
struct AncientTitle {
    key: String,
    table: String
}

#[derive(Serialize, Deserialize)]
struct AncientChoice {
    text_key: String,
    title: AncientTitle,
    was_chosen: bool
}

#[derive(Serialize, Deserialize)]
struct PlayerStats {
    card_choices: Vec<CardChoice>,
    ancient_choices: Vec<AncientChoice>,
    current_gold: u32,
    current_hp: u32,
    damage_taken: u32,
    gold_gained: u32,
    gold_lost: u32,
    gold_stolen: u32,
    hp_healed: u32,
    max_hp: u32,
    max_hp_gained: u32,
    max_hp_lost: u32,
    player_id: u32
}

#[derive(Serialize, Deserialize)]
struct Room {
    model_id: String,
    room_type: String,
    turns_taken: u32,
    monster_ids: Option<Vec<String>>
}

#[derive(Serialize, Deserialize)]
struct MapPoint {
    map_point_type: String,
    player_stats: Vec<PlayerStats>,
    room: Vec<Room>,
}

#[derive(Serialize, Deserialize)]
struct Badge {
    id: String,
    rarity: String,
}

#[derive(Serialize, Deserialize)]
struct LootItem {
    id: String,
    floor_added_to_deck: u32,
    current_upgrade_level: Option<u32>,
    slot_index: Option<u32>,
}

#[derive(Serialize, Deserialize)]
struct Player {
    badge: Option<Vec<Badge>>,
    character: String,
    deck: Vec<LootItem>,
    relics: Vec<LootItem>,
    potion: Vec<LootItem>,
    max_potion_slot_count: u32,
    id: u32
}

#[derive(Serialize, Deserialize)]
pub struct Run {
    acts: Vec<String>,
    ascension: u32,
    build_id: String,
    game_mod: String,
    killed_by_encounter: String,
    killed_by_event: String,
    map_point_history: Vec<Vec<MapPoint>>,
    modifiers: Vec<String>,
    platform_type: String,
    players: Vec<Player>,
    run_time: u32,
    schema_version: u32,
    seed: String,
    start_time: u32,
    was_abandoned: bool,
    win: bool
}