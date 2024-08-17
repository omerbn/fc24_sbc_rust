use std::collections::HashMap;
use crate::query::LeagueNationTeamRequirement;
use crate::permutations::PermutationFilter;
use crate::println_with_timestamp;

pub struct LNTPermutationFilter<'a>
{
    _requirement: &'a LeagueNationTeamRequirement,
    _map: HashMap<u32, u8>,
    _field_name: &'static str,
}

impl<'a> LNTPermutationFilter<'a>
{
    pub fn new(requirement: &'a LeagueNationTeamRequirement, field_name: &'static str) -> Self {
        LNTPermutationFilter {
            _requirement: requirement,
            _map: HashMap::new(),
            _field_name: field_name,
        }
    }
}

impl<'a> PermutationFilter<u32> for LNTPermutationFilter<'a>
{
    fn clear(&mut self) {
        self._map.clear();
    }

    fn delete(&mut self, obj: u32) {
        let value = self._map.get_mut(&obj).unwrap();
        if *value == 1 {
            self._map.remove(&obj);
        } else {
            *value -= 1;
        }
    }

    fn try_add(&mut self, obj: u32) -> bool {
        let total_keys = self._map.len();
        let value = self._map.get(&obj).unwrap_or(&0);
        if *value == 0 {
            match self._requirement.exact_in_squad {
                Some(exact_in_squad) => {
                    if total_keys == exact_in_squad as usize {
                        return false;
                    }
                }
                None => {}
            }
            match self._requirement.max_in_squad {
                Some(max_in_squad) => {
                    if total_keys == max_in_squad as usize {
                        return false;
                    }
                }
                None => {}
            }

            self._map.insert(obj, 1);
        } else {
            match self._requirement.exact_players_from_same {
                Some(exact_players_from_same) => {
                    if *value == exact_players_from_same {
                        return false;
                    }
                }
                None => {}
            }

            match self._requirement.max_players_from_same {
                Some(max_players_from_same) => {
                    if *value == max_players_from_same {
                        return false;
                    }
                }
                None => {}
            }
            self._map.insert(obj, *value + 1);
        }

        true
    }

    fn is_permutation_approved(&self, _permutation: &Vec<u32>) -> bool {
        match self._requirement.exact_in_squad {
            Some(exact_in_squad) => {
                if self._map.len() != exact_in_squad as usize {
                    return false;
                }
            }
            None => {}
        }

        match self._requirement.min_in_squad {
            Some(min_in_squad) => {
                if self._map.len() < min_in_squad as usize {
                    return false;
                }
            }
            None => {}
        }

        if self._requirement.exact_players_from_same.is_some() || self._requirement.min_players_from_same.is_some() {
            for (key, value) in self._map.iter() {
                
                match self._requirement.exact_players_from_same {
                    Some(exact_players_from_same) => {
                        if *value != exact_players_from_same {
                            return false;
                        }
                    }
                    None => {}
                }
                
                match self._requirement.min_players_from_same {
                    Some(min_players_from_same) => {
                        if *value < min_players_from_same {
                            return false;
                        }
                    }
                    None => {}
                }
            }
        }

        true
    }
}