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
            #[allow(clippy::match_on_vec_items)]
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
