use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::query::SearchQuery;

// all challenges
lazy_static! {
    static ref Challenges: HashMap<String, SearchQuery> = {
        let mut m: HashMap<String, SearchQuery> = HashMap::new();

        let mut json = crate::utils::load_json::<SearchQuery>(
            r#"{
            "name": "FUTTIES Crafting Upgrade",
            "positions": ["GK", "LB", "CB", "CB", "RB", "LM", "CM", "CM", "RM", "ST", "ST"],
            "ratings": {
                "min_player": 75
            },
            "allow_random_positions": true
        }"#,
        )
        .unwrap();
        m.insert(json.name.to_string(), json.clone());

        json = crate::utils::load_json::<SearchQuery>(
            r#"{
            "name": "gold upgrade",
            "positions": ["GK", "LB", "CB", "CB", "RB", "LM", "CDM", "CAM", "RM", "ST", "ST"],
            "ratings": {
                "min_player": 75
            },
            "allow_random_positions": true
        }"#,
        )
        .unwrap();
        m.insert(json.name.to_string(), json.clone());

        json = crate::utils::load_json::<SearchQuery>(
        r#"{
            "name": "silver upgrade",
            "positions": ["GK", "LB", "CB", "CB", "RB", "LM", "CM", "CM", "RM", "ST", "ST"],
            "ratings": {
                "min_player": 65,
                "max_player": 74
            },
            "allow_random_positions": true
        }"#).unwrap();
        m.insert(json.name.to_string(), json.clone());


        json = crate::utils::load_json::<SearchQuery>(
            r#"{
                "name": "bronze upgrade",
                "positions": ["GK", "LB", "CB", "CB", "RB", "LM", "CM", "CM", "RM", "ST", "ST"],
                "ratings": {
                    "max_player": 64
                },
                "rarity": {
                    "common": "=11"
                },
                "allow_random_positions": true
            }"#
        ).unwrap();
        m.insert(json.name.to_string(), json.clone());

        
        json = crate::utils::load_json::<SearchQuery>(
            r#"{
                "name": "81+ player pick",
                "positions": ["GK", "LB", "CB", "CB", "RB", "LM", "CM", "CM", "RM", "ST", "ST"],
                "ratings": {
                    "min_squad": 87
                },
                "allow_random_positions": true
            }"#
        ).unwrap();
        m.insert(json.name.to_string(), json.clone());

        json = crate::utils::load_json::<SearchQuery>(
            r#"{
                "name": "82+ player pick",
                "positions": ["GK", "CB", "CB", "CM", "CM", "CM", "CF", "CF"],
                "ratings": {
                    "min_player": 75
                },
                "rarity": {
                    "conditions": {
                        "rare": ">2"
                    },
                    "allowed_rarities": ["rare", "common"]
                },
                "allow_random_positions": true
            }"#
        ).unwrap();
        m.insert(json.name.to_string(), json.clone());

        
        json = crate::utils::load_json::<SearchQuery>(
            r#"{
            "name": "82 challenge squad",
            "group": "upgrades",
            "positions": ["GK", "LB", "CB", "CB", "RB", "LM", "CM", "CDM", "CM", "RM", "ST"],
            "ratings": {
                "min_squad": 82,
                "min_player": 70
            },
            "allow_random_positions": true
        }"#
        ).unwrap();
        m.insert(json.name.to_string(), json.clone());

        json = crate::utils::load_json::<SearchQuery>(
            r#"{
            "group": "Foundations V",
            "name": "Better buildup",
            "positions": ["CM", "CM", "CM"],
            "ratings": {
                "max_player": 64
            },
            "chemistry": {
                "min_points_for_each_player": 1
            }
        }"#
        ).unwrap();
        m.insert(json.name.to_string(), json.clone());

        json = crate::utils::load_json::<SearchQuery>(
            r#"{
            "group": "Foundations V",
            "name": "Advancing attack",
            "positions": ["CF", "ST", "ST"],
            "ratings": {
                "max_player": 64
            },
            "chemistry": {
                "min_points_for_each_player": 1
            }
        }"#
        ).unwrap();
        m.insert(json.name.to_string(), json.clone());

        json = crate::utils::load_json::<SearchQuery>(
            r#"{
                "group": "Hybrid Leagues",
                "name": "Seven Leagute boots",
                "positions": ["GK", "CB", "CB", "CB", "LM", "CM", "CM", "RM", "LW", "ST", "RW"],
                "leagues": {
                    "exact_in_squad": 7,
                    "max_players_from_same": 3
                },
                "teams": {
                    "max_players_from_same": 3
                },
                "ratings": {
                    "min_squad": 78
                },
                "chemistry": {
                    "min_squad": 18,
                    "min_points_for_each_player": 1
                }
            }"#
        ).unwrap();
        m.insert(json.name.to_string(), json.clone());

        json = crate::utils::load_json::<SearchQuery>(
            r#"{
                "group": "Hybrid Leagues",
                "name": "First XI",
                "positions": ["GK", "LB", "CB", "CB", "RB", "LM", "CM", "CDM", "CM", "RM", "ST"],
                "leagues": {
                    "exact_in_squad": 11
                },
                "ratings": {
                    "min_player": 75
                },
                "rarity": {
                    "conditions": {
                        "rare": ">6"
                    }
                },
                "chemistry": {
                    "min_squad": 27
                }
            }"#
        ).unwrap();
        m.insert(json.name.to_string(), json.clone());

        json = crate::utils::load_json::<SearchQuery>(
            r#"{
                "group": "Hybrid Leagues",
                "name": "Whole nine yards",
                "positions": ["GK", "LB", "CB", "CB", "RB", "LM", "CDM", "CDM", "RM", "ST", "ST"],
                "leagues": {
                    "exact_in_squad": 9,
                    "max_players_from_same": 2
                },
                "teams": {
                    "max_players_from_same": 2
                },
                "rarity": {
                    "conditions": {
                        "rare": ">5"
                    }
                },
                "ratings": {
                    "min_squad": 80
                },
                "chemistry": {
                    "min_squad": 21
                }
            }"#
        ).unwrap();
        m.insert(json.name.to_string(), json.clone());

        json = crate::utils::load_json::<SearchQuery>(
            r#"{
                "group": "Icon:Kaka",
                "name": "Born Legend",
                "positions": ["GK", "LWB", "CB", "CB", "CB", "RWB", "CM", "CDM", "CM", "ST", "ST"],
                "ratings": {
                    "max_player": 64
                },
                "rarity": {
                    "rare": "=11"
                },
                "max_matches": 5
            }"#
        ).unwrap();
        m.insert(json.name.to_string(), json.clone());

        json = crate::utils::load_json::<SearchQuery>(
            r#"{
                "group": "Icon:Kaka",
                "name": "Rising Star",
                "positions": ["GK", "LB", "CB", "CB", "RB", "LM", "CAM", "CM", "CAM", "RM", "ST"],
                "ratings": {
                    "max_player": 74,
                    "min_player": 65
                },
                "rarity": {
                    "rare": "=11"
                }
            }"#
        ).unwrap();
        m.insert(json.name.to_string(), json.clone());

        json = crate::utils::load_json::<SearchQuery>(
            r#"{
                "group": "Icon:Kaka",
                "name": "Mid Icon",
                "positions": ["GK", "LB", "CB", "CB", "RB", "LM", "CAM", "CM", "CAM", "RM", "ST"],
                "ratings": {
                    "min_player": 75
                },
                "rarity": {
                    "rare": "=11"
                }
            }"#
        ).unwrap();
        m.insert(json.name.to_string(), json.clone());

        json = crate::utils::load_json::<SearchQuery>(
            r#"{
                "name": "Six of the best",
                "group": "Hybrid nations",
                "positions": ["GK", "LB", "CB", "CB", "RB", "LM", "CM", "CM", "RM", "ST", "ST"],
                "nations": {
                    "exact_in_squad": 6,
                    "max_players_from_same": 3
                },
                "teams": {
                    "max_players_from_same": 3
                },
                "ratings": {
                    "min_squad": 75
                },
                "chemistry": {
                    "min_squad": 18
                }
            }"#
        ).unwrap();
        m.insert(json.name.to_string(), json.clone());

        json = crate::utils::load_json::<SearchQuery>(
            r#"{
                "name": "Elite eight",
                "group": "Hybrid nations",
                "positions": ["GK", "LB", "CB", "CB", "RB", "CM", "CM", "CM", "LW", "ST", "RW"],
                "nations": {
                    "exact_in_squad": 8,
                    "max_players_from_same": 2
                },
                "teams": {
                    "max_players_from_same": 3
                },
                "rarity": {
                    "rare": ">4"
                },
                "ratings": {
                    "min_player": 75
                },
                "chemistry": {
                    "min_squad": 21
                }
            }"#
        ).unwrap();
        m.insert(json.name.to_string(), json.clone());


        json = crate::utils::load_json::<SearchQuery>(
            r#"{
                "name": "Around the world",
                "group": "Hybrid nations",
                "positions": ["GK", "LWB", "CB", "CB", "CB", "RWB", "CM", "CM", "CAM", "ST", "ST"],
                "nations": {
                    "exact_in_squad": 10
                },
                "rarity": {
                    "rare": ">7"
                },
                "ratings": {
                    "min_squad": 81
                },
                "chemistry": {
                    "min_points_for_each_player": 2,
                    "min_squad": 24
                }
            }"#
        ).unwrap();
        m.insert(json.name.to_string(), json.clone());

        json = crate::utils::load_json::<SearchQuery>(
            r#"{
                "name": "advanced",
                "group": "League and Nation Hybrid",
                "positions": ["GK", "LB", "CB", "CB", "RB", "LM", "CM", "CM", "CM", "RM", "ST"],
                "leagues": {
                    "exact_in_squad": 3,
                    "max_players_from_same": 5
                },
                "ratings": {
                    "min_squad": 79,
                    "max_player": 83,
                    "min_player": 62
                },
                "nations": {
                    "exact_in_squad": 4,
                    "max_players_from_same": 6
                },
                "chemistry": {
                    "min_squad": 29
                }
            }"#
        ).unwrap();
        m.insert(json.name.to_string(), json.clone());

        json = crate::utils::load_json::<SearchQuery>(
            r#"{
                "name": "crafting futties",
                "positions": ["GK", "LB", "CB", "CB", "RB", "LM", "CM", "CM", "RM", "ST", "ST"],
                "ratings": {
                    "min_player": 75
                },
                "rarity": {
                    "rare": ">0"
                },
                "allow_random_positions": true
            }"#
        ).unwrap();
        m.insert(json.name.to_string(), json.clone());

        json = crate::utils::load_json::<SearchQuery>(
            r#"{
                "name": "90+ tradeable TOTS",
                "group": "upgrades",
                "positions": ["GK", "LB", "CB", "CB", "RB", "LM", "CM", "CDM", "CM", "RM", "ST"],
                "ratings": {
                    "min_squad": 86,
                    "min_player": 78,
                    "max_player": 93
                },
                "allow_random_positions": true
            }"#
        ).unwrap();
        m.insert(json.name.to_string(), json.clone());

        json = crate::utils::load_json::<SearchQuery>(
            r#"{
                "name": "83 rated",
                "group": "upgrades",
                "positions": ["GK", "LB", "CB", "CB", "RB", "LM", "CDM", "RM", "CAM", "ST", "ST"],
                "ratings": {
                    "min_squad": 83,
                    "min_player": 77,
                    "max_player": 88
                },
                "rarity": {
                    "allowed_rarities": ["rare", "common"]
                },
                "allow_random_positions": true
            }"#
        ).unwrap();
        m.insert(json.name.to_string(), json.clone());

        json = crate::utils::load_json::<SearchQuery>(
            r#"{
                "name": "bronze upgrade",
                "group": "Ultimate Bronze upgrade",
                "positions": ["GK", "LB", "CB", "CB", "RB", "CM", "CM", "CM", "LW", "ST", "RW"],
                "ratings": {
                    "min_player": 50
                },
                "leagues": {
                    "exact_in_squad": 4,
                    "max_players_from_same": 3
                },
                "nations": {
                    "exact_in_squad": 2,
                    "min_players_from_same": 3
                }
            }"#
        ).unwrap();
        m.insert(json.name.to_string(), json.clone());

        m
    };
}

pub fn get_challenge(name: &str) -> Option<SearchQuery> {
    Challenges.get(name).cloned()
}

#[allow(dead_code)]
pub fn get_challenges() -> HashMap<String, SearchQuery> {
    Challenges.clone()
}