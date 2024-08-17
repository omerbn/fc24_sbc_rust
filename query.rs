use serde_json::Value;
use std::collections::HashSet;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone, Default)]
pub struct AmountFilter {
    pub total: Option<u8>,
    pub min: Option<u8>,
    pub max: Option<u8>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone, Default)]
pub struct MinMax {
    pub max_player: Option<u8>,
    pub min_player: Option<u8>,
    pub min_squad: Option<u8>,
    pub max_squad: Option<u8>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone, Default)]
pub struct Rarity {
    pub allowed_rarities: Option<Vec<String>>,
    pub conditions: Option<Value>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone, Default)]
pub struct Chemistry {
    pub min_points_for_each_player: Option<u8>,
    pub min_squad: Option<u8>,    
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone, Default)]
pub struct LeagueNationTeamRequirement {
    pub exact_in_squad: Option<u8>,
    pub min_in_squad: Option<u8>,
    pub max_in_squad: Option<u8>,

    pub max_players_from_same: Option<u8>,
    pub min_players_from_same: Option<u8>,
    pub exact_players_from_same: Option<u8>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone, Default)]
pub struct SearchQuery {
    pub group: Option<String>,
    pub name: String,
    pub positions: Vec<String>,
    pub leagues: Option<LeagueNationTeamRequirement>,
    pub nations: Option<LeagueNationTeamRequirement>,
    pub teams: Option<LeagueNationTeamRequirement>,
    pub ratings: Option<MinMax>,
    pub chemistry: Option<Chemistry>,
    pub rarity: Option<Rarity>,
    pub quality: Option<Value>,
    pub players_to_skip: Option<HashSet<u64>>,
    pub hinted_players: Option<HashSet<String>>,
    pub hinted_players_are_must: Option<bool>,

    #[serde(default="default_max_results")]
    pub max_results: u32,

    pub max_running_time_sec: Option<u32>,
    pub allow_random_positions: Option<bool>,
}

fn default_max_results() -> u32{
    1_000_000
}