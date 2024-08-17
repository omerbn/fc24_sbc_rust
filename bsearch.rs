use std::usize;

use crate::player::Player;

pub fn binary_search_by_rating(arr: &Vec<Player>, target: u8) -> usize {
    let mut low = 0;
    let mut high = arr.len();
    
    while low < high {
        let mid = low + (high - low) / 2;
        
        if arr[mid].rating < target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    
    // `low` is the insertion point
    if low < arr.len() && arr[low].rating == target {
        // Find the first occurrence of the target
        while low > 0 && arr[low - 1].rating == target {
            low -= 1;
        }
        return low;
    }
    
    // If the target is not found, return the index of the next larger value
    if low < arr.len() {
        return low;
    }
    
    // Return usize::MAX if no larger value exists
    arr.len()
}



#[cfg(test)]
mod tests {
    use super::*;

    fn assert_case(input: Vec<Player>, target: u8, expected: usize) {
        let res = binary_search_by_rating(&input, target);
        assert_eq!(res, expected);
    }

    fn create_player_with_rating(rating: u8) -> Player {
        Player { id: 0, rating, ..Default::default() }
    }

    fn create_players_with_ratings(ratings: Vec<u8>) -> Vec<Player> {
        ratings.into_iter().map(|r| create_player_with_rating(r)).collect()
    }

    #[test]
    fn test_binary_search_by_rating() {
        assert_case(create_players_with_ratings(vec![1, 3, 5, 7, 9, 11, 13]), 6, 3);
        assert_case(create_players_with_ratings(vec![1, 3, 5, 7, 9, 11, 13]), 9, 4);
        assert_case(create_players_with_ratings(vec![1, 3, 5, 7, 9, 11, 13]), 0, 0);
        assert_case(create_players_with_ratings(vec![1, 3, 5, 7, 9, 11, 13]), 14, 7);
        assert_case(create_players_with_ratings(vec![1, 3, 5, 7, 9, 11, 13]), 12, 6);
        assert_case(create_players_with_ratings(vec![1, 1, 1, 2, 9, 11, 13]), 1, 0);
        assert_case(create_players_with_ratings(vec![1, 1, 1, 2, 9, 11, 13]), 2, 3);
        assert_case(create_players_with_ratings(vec![1, 1, 1, 2, 9, 11, 13]), 3, 4);
        assert_case(create_players_with_ratings(vec![1, 1, 1, 2, 9, 11, 13, 14]), 3, 4);
        assert_case(create_players_with_ratings(vec![1, 1, 1, 2, 9, 11, 13]), 14, 7);
    }


}
