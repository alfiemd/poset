use crate::PartialOrderBehaviour;

/// A struct representing an iterator over the antichains from a set of chains.
pub struct AntichainIterator<'a, 'b, T, F>
where
    F: PartialOrderBehaviour<Element = T>,
{
    vectors: Vec<Vec<&'a T>>,
    indices: Vec<Option<usize>>,
    finished: bool,
    p_ord: &'b F,
}

impl<'a, 'b, T, F> AntichainIterator<'a, 'b, T, F>
where
    T: Clone,
    F: PartialOrderBehaviour<Element = T>,
{
    /// Construct a new `AntichainIterator`, given a list of chains and a partial order.
    ///
    /// The input is expected to be a valid chain decomposition: chains should be pairwise
    /// non-overlapping according to the partial order equality.
    pub fn new(vectors: Vec<Vec<&'a T>>, p_ord: &'b F) -> Self {
        AntichainIterator {
            indices: vec![None; vectors.len()],
            vectors,
            finished: false,
            p_ord,
        }
    }

    fn is_incomparable(&self, combination: &[&T]) -> bool {
        for (i, item1) in combination.iter().enumerate() {
            for item2 in combination.iter().skip(i + 1) {
                if self.p_ord.cp(item1, item2) {
                    return false;
                }
            }
        }
        true
    }

    fn advance_indices(&mut self) -> bool {
        for i in (0..self.indices.len()).rev() {
            match self.indices[i] {
                None => {
                    if !self.vectors[i].is_empty() {
                        self.indices[i] = Some(0);
                        return true;
                    }
                }
                Some(idx) if idx + 1 < self.vectors[i].len() => {
                    self.indices[i] = Some(idx + 1);
                    return true;
                }
                _ => {
                    self.indices[i] = None;
                }
            }
        }
        false
    }
}

impl<'a, 'b, T, F> Iterator for AntichainIterator<'a, 'b, T, F>
where
    T: Clone,
    F: PartialOrderBehaviour<Element = T>,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.finished {
            let combination: Vec<&T> = self
                .indices
                .iter()
                .enumerate()
                .filter_map(|(i, &idx)| idx.and_then(|idx| self.vectors[i].get(idx)))
                .copied()
                .collect();

            if !self.advance_indices() {
                self.finished = true;
            }

            if self.is_incomparable(&combination) {
                return Some(combination.into_iter().cloned().collect());
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::AntichainIterator;
    use crate::PartialOrder;

    #[test]
    fn empty_chains_yield_only_empty_antichain() {
        let p_ord = PartialOrder::new(usize::ge);
        let mut iter: AntichainIterator<'_, '_, usize, _> = AntichainIterator::new(vec![], &p_ord);

        assert_eq!(iter.next(), Some(vec![]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn singleton_chains_generate_expected_antichains() {
        let p_ord = PartialOrder::new(usize::ge);
        let chains = vec![vec![&1_usize], vec![&2_usize]];
        let antichains: Vec<Vec<usize>> = AntichainIterator::new(chains, &p_ord).collect();

        assert_eq!(antichains, vec![vec![], vec![2], vec![1]]);
    }

    #[test]
    fn empty_chain_is_skipped_while_iterating() {
        let p_ord = PartialOrder::new(usize::ge);
        let chains = vec![vec![&1_usize], vec![], vec![&3_usize]];
        let antichains: Vec<Vec<usize>> = AntichainIterator::new(chains, &p_ord).collect();

        assert_eq!(antichains, vec![vec![], vec![3], vec![1]]);
    }
}
