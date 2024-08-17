use indexmap::IndexSet;
use std::time::Instant;

use crate::filters::lnt::LNTPermutationFilter;
use crate::permutations::DistinctPermutationFilter;
use crate::permutations::PermutationsGenerator;
use crate::player::Player;
use crate::query::SearchQuery;
use crate::println_with_timestamp;

#[macro_export]
macro_rules! reduce_player_vec {
    ($players:expr, $filter:ident) => {
        $players
            .iter()
            .map(|list_of_players: &Vec<&Player>| {
                let mut s = IndexSet::new();
                for player in list_of_players.iter() {
                    s.insert(player.$filter);
                }
                return s.into_iter().collect();
            })
            .collect()
    };
}

#[macro_export]
macro_rules! rebuild_player_vec {
    ($permutation:expr, $filter:ident, $players:expr) => {
        $permutation
            .iter()
            .enumerate()
            .map(|(i, league_id)| {
                let mut pp: Vec<&Player> = Vec::new();
                for player in $players[i].iter() {
                    if player.$filter == *league_id {
                        pp.push(player);
                    }
                }
                pp
            })
            .collect()
    };
}

pub fn find<'a>(
    store: &'a crate::store::LocalStore,
    query: &'a SearchQuery,
    t0: Instant,
) -> Vec<Vec<&'a Player>> {
    let players_slice = filter_by_player_rating(store, &query);

    ///////////////////////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////////////////////
    // creating a vector of positions
    println_with_timestamp!("Creating a vector of positions");
    let allow_all_positions = query.allow_random_positions.unwrap_or(false);
    let v_players_in_positions: Vec<Vec<&Player>> = query
        .positions
        .iter()
        .map(|pos| {
            if allow_all_positions {
                return players_slice.iter().collect();
            } else {
                players_slice
                    .iter()
                    .filter(|player| player.possiblePositions.contains(pos))
                    .collect()
            }
        })
        .collect();
    println_with_timestamp!("Done");

    ///////////////////////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////////////////////
    let mut results: Vec<Vec<&Player>> = Vec::new();
    let mut hashed_results: IndexSet<String> = IndexSet::new();

    match &query.nations {
        Some(nations) => {
            let t: Vec<Vec<u32>> = reduce_player_vec!(v_players_in_positions, nation);
            let nation_permutations =
                PermutationsGenerator::new(&t, LNTPermutationFilter::new(nations, "nation"));
            for nation_permutation in nation_permutations {
                let n_v_players_in_positions: Vec<Vec<&Player>> =
                    rebuild_player_vec!(nation_permutation, nation, v_players_in_positions);
                match &query.leagues {
                    Some(leagues) => {
                        let t: Vec<Vec<u32>> =
                            reduce_player_vec!(n_v_players_in_positions, leagueId);
                        let league_permutations = PermutationsGenerator::new(
                            &t,
                            LNTPermutationFilter::new(leagues, "leagueId"),
                        );
                        let t1 = Instant::now();
                        if t1- t0 > std::time::Duration::from_secs(50) {
                            println_with_timestamp!("Time limit reached. total results={}", results.len());
                            return results;
                        }
                        for league_permutation in league_permutations {
                            let n_v_players_in_positions2 = rebuild_player_vec!(
                                league_permutation,
                                leagueId,
                                n_v_players_in_positions
                            );
                            let final_builder = PermutationsGenerator::new(
                                &n_v_players_in_positions2,
                                DistinctPermutationFilter::new(),
                            );
                            for permutation in final_builder {
                                // hash
                                let mut sorted_by_id: Vec<u64> = permutation.iter().map(|p| p.id).collect();
                                sorted_by_id.sort();
                                let hash = format!("{:?}", sorted_by_id);
                                if hashed_results.contains(&hash) {
                                    continue;
                                }
                                hashed_results.insert(hash);

                                // adding to results
                                results.push(permutation);
                                if query.max_results == results.len() as u32 {
                                    println_with_timestamp!(
                                        "Reached required number of results: {}",
                                        results.len()
                                    );
                                    return results;
                                }
                            }
                        }
                    }
                    None => (),
                }
            }
        }
        None => match &query.leagues {
            Some(leagues) => {
                let t: Vec<Vec<u32>> = reduce_player_vec!(v_players_in_positions, leagueId);
                let league_permutations =
                    PermutationsGenerator::new(&t, LNTPermutationFilter::new(leagues, "leagueId"));
                for league_permutation in league_permutations {
                    let n_v_players_in_positions: Vec<Vec<&Player>> =
                        rebuild_player_vec!(league_permutation, leagueId, v_players_in_positions);
                    let final_builder = PermutationsGenerator::new(
                        &n_v_players_in_positions,
                        DistinctPermutationFilter::new(),
                    );
                    for permutation in final_builder {
                        // hash
                        let mut sorted_by_id: Vec<u64> = permutation.iter().map(|p| p.id).collect();
                        sorted_by_id.sort();
                        let hash = format!("{:?}", sorted_by_id);
                        if hashed_results.contains(&hash) {
                            continue;
                        }
                        hashed_results.insert(hash);

                        // adding to results
                        results.push(permutation);
                        if query.max_results == results.len() as u32 {
                            println_with_timestamp!(
                                "Reached required number of results: {}",
                                results.len()
                            );
                            return results;
                        }
                    }
                }
            }
            None => (),
        },
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////////////////////
    let final_builder =
        PermutationsGenerator::new(&v_players_in_positions, DistinctPermutationFilter::new());
    let mut results: Vec<Vec<&Player>> = Vec::new();
    for permutation in final_builder {
        results.push(permutation);
        if query.max_results == results.len() as u32 {
            println_with_timestamp!("Reached required number of results: {}", results.len());
            break;
        }
    }

    return results;
}

fn filter_by_player_rating<'a>(
    store: &'a crate::store::LocalStore,
    query: &'a SearchQuery,
) -> &'a [Player] {
    let mut si = 0;
    let mut ei = store.players.len();
    let mut is_filtering_by_player_rating = false;

    // filtering by rating
    match &query.ratings {
        Some(ratings) => {
            if ratings.min_player.is_some() {
                si = crate::bsearch::binary_search_by_rating(
                    &store.players,
                    ratings.min_player.unwrap(),
                );
                is_filtering_by_player_rating = true;
            }
            if ratings.max_player.is_some() {
                ei = crate::bsearch::binary_search_by_rating(
                    &store.players,
                    ratings.max_player.unwrap() + 1,
                );
                is_filtering_by_player_rating = true;
            }
        }
        None => (),
    }

    let players_slice = store.create_slice(si, ei);
    if is_filtering_by_player_rating {
        println_with_timestamp!(
            "Filtered by player rating. Current player count={}",
            players_slice.len()
        );
    }
    players_slice
}
