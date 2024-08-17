use std::collections::HashSet;
use std::hash::Hash;
use crate::println_with_timestamp;

pub trait PermutationFilter<T>
{
    fn clear(&mut self) -> ();
    fn delete(&mut self, obj: T) -> ();
    fn try_add(&mut self, obj: T) -> bool;
    fn is_permutation_approved(&self, permutation: &Vec<T>) -> bool;
}

pub struct DistinctPermutationFilter<T>
where
    T: Copy + Eq + Hash,
{
    _objects: HashSet<T>,
}

impl<T> DistinctPermutationFilter<T>
where
    T: Copy + Eq + Hash,
{
    pub fn new() -> DistinctPermutationFilter<T> {
        DistinctPermutationFilter {
            _objects: HashSet::new(),
        }
    }
}

impl<T> PermutationFilter<T> for DistinctPermutationFilter<T>
where
    T: Copy + Eq + Hash,
{
    fn clear(&mut self) {
        self._objects.clear();
    }

    fn delete(&mut self, obj: T) {
        self._objects.remove(&obj);
    }

    fn try_add(&mut self, obj: T) -> bool {
        if self._objects.contains(&obj) {
            false
        } else {
            self._objects.insert(obj);
            true
        }
    }

    fn is_permutation_approved(&self, _permutation: &Vec<T>) -> bool {
        true
    }
}

pub struct PermutationsGenerator<'a, T, Filter>
where
    T: Copy,
    Filter: PermutationFilter<T>,
{
    _filter: Filter,
    _next_checkpoint: u32,
    _indices: [usize; 11],
    _permutation: Vec<T>,
    _by_positions: &'a Vec<Vec<T>>,
    _i: usize,
}

impl<'a, T, Filter> PermutationsGenerator<'a, T, Filter>
where
    T: Copy,
    Filter: PermutationFilter<T>,
{
    pub fn new(by_positions: &'a Vec<Vec<T>>, filter: Filter) -> PermutationsGenerator<T, Filter> {
        // let mut total_permutations = 1;
        // for i in by_positions.iter() {
        //     total_permutations *= i.len();
        // }
        // println_with_timestamp!(
        //     "new PermutationsGenerator. Total permutations: {}",
        //     total_permutations
        // );

        PermutationsGenerator {
            _filter: filter,
            _next_checkpoint: 100_000_000,
            _indices: [usize::MAX; 11],
            _permutation: vec![by_positions[0][0]; by_positions.len()],
            _by_positions: by_positions,
            _i: 0,
        }
    }

    fn print_state(&self) {
        println!("i={}, indices={:?}", self._i, self._indices);
    }
}

impl<'a, T, Filter> Iterator for PermutationsGenerator<'a, T, Filter>
where
    T: Copy,
    Filter: PermutationFilter<T>,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // found a match in previous call
        if self._i == self._by_positions.len() {
            self._i -= 1;
            self._filter.delete(self._permutation[self._i]);
        }

        loop {
            if self._i == 0 {
                self._filter.clear();
            }

            if self._indices[self._i] == usize::MAX {
                // first time
                self._indices[self._i] = 0;
            } else {
                self._indices[self._i] += 1;
            }
            // end of these entry indices
            if self._indices[self._i] >= self._by_positions[self._i].len() {
                if self._i == 0 {
                    return None;
                }
                self._indices[self._i] = usize::MAX;
                self._i -= 1;
                self._filter.delete(self._permutation[self._i]);
                continue;
            }

            // can we add this object?
            let obj: T = self._by_positions[self._i][self._indices[self._i]];
            if !self._filter.try_add(obj) {
                continue;
            }

            self._permutation[self._i] = obj;

            // if we have a full permutation
            if self._i == self._by_positions.len() - 1 {
                if self._filter.is_permutation_approved(&self._permutation) {
                    self._i += 1;
                    return Some(self._permutation.clone());
                }
            } else {
                self._i += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use std::time::Instant;

    fn assert_permutation(input: Vec<Vec<u32>>, output: Vec<Vec<u32>>) -> Duration {
        let start = Instant::now();
        let mut p = PermutationsGenerator::new(&input, DistinctPermutationFilter::new());
        let mut c = 0;
        let mut results: Vec<Vec<u32>> = Vec::new();
        while let Some(r) = p.next() {
            results.push(r);
            c += 1;
        }
        let duration = start.elapsed();
        // assert_eq!(c, output.len());
        // assert_eq!(results, output);

        duration
    }

    #[test]
    fn permutations_distinct() {
        let mut d: Duration = Duration::new(0, 0);
        d += assert_permutation(
            vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]],
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1],
            ],
        );
        d += assert_permutation(
            vec![vec![1, 2], vec![3, 4], vec![5, 6, 7]],
            vec![
                vec![1, 3, 5],
                vec![1, 3, 6],
                vec![1, 3, 7],
                vec![1, 4, 5],
                vec![1, 4, 6],
                vec![1, 4, 7],
                vec![2, 3, 5],
                vec![2, 3, 6],
                vec![2, 3, 7],
                vec![2, 4, 5],
                vec![2, 4, 6],
                vec![2, 4, 7],
            ],
        );
        d += assert_permutation(
            vec![vec![1, 2], vec![3, 1], vec![4, 2]],
            vec![vec![1, 3, 4], vec![1, 3, 2], vec![2, 3, 4], vec![2, 1, 4]],
        );
        println!("Time taken: {:?}, average={:?}", d, (d / 3));
    }
}
