#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

mod bsearch;
mod challenges;
mod filters;
mod macros;
mod permutations;
mod player;
mod query;
mod store;
mod utils;
mod worker;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gstore = store::GlobalStore::new();
    let lstore = gstore.get_local_store();

    run_challenge(&lstore, "FUTTIES Crafting Upgrade");

    // match lstore.get_player_by_name("Kaká") {
    //     Some(player) => {
    //         println!("{:?}", player);
    //     }
    //     None => {
    //         println!("Player not found");
    //     }
    // }

    // let p1 = lstore.get_player_by_name("Ka2ká").unwrap_or_else(|| {
    //     panic!("Player not found");
    // });

    Ok(())
}

fn run_challenge(store: &store::LocalStore, name: &str) {
    let t0 = std::time::Instant::now();
    // match challenges::get_challenge(name) {
    //     Some(challenge) => {
    //         println!("{:?}", challenge);
    //     }
    //     None => {
    //         println!("Challenge {} not found", name);
    //     }
    // }
    let q = crate::utils::load_json::<crate::query::SearchQuery>(
        r#"{
        "name": "FUTTIES Crafting Upgrade",
        "positions": ["GK", "LB", "CB", "CB", "RB", "LM", "CM", "CM", "RM", "ST", "ST"],
        "ratings": {
            "min_player": 75
        },
        "leagues": {
            "exact_in_squad": 4
        }
    }"#,
    )
    .unwrap();
    let results = worker::find(store, &q, t0);

    let t1 = std::time::Instant::now();
    println!("Challenge elapsed time: {:?}", t1 - t0);

    if results.len() == 0 {
        println!("No results found");
        return;
    }

    // sorting results by rating
    let mut result_to_rating_map: HashMap<u32, u32> = HashMap::new();
    for (index, result) in results.iter().enumerate() {
        let mut total_rating: u32 = 0;
        for player in result {
            total_rating += player.rating as u32;
        }
        result_to_rating_map.insert(index as u32, total_rating);
    }
    let mut sorted_entries: Vec<_> = result_to_rating_map.into_iter().collect();
    sorted_entries.sort_by_key(|&(_, rating)| rating);
    let sorted_results: Vec<&Vec<&player::Player>> = sorted_entries
        .iter()
        .map(|(index, _)| &results[*index as usize])
        .collect();

    
    // printing the first result
    let fresult = &sorted_results[0];
    for (index, player) in fresult.iter().enumerate() {
        println!(
            "{} {} ({})",
            &q.positions[index], player.__fullname, player.rating
        );
    }

}
