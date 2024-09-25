use crate::AntichainIterator;
use crate::PosetError;
use crate::{PartialOrderBehaviour, PosetBehaviour};

#[cfg(feature = "rand")]
use rand::seq::SliceRandom;

#[cfg(feature = "graff")]
use graff::{Graph, GraphBehaviour};

/// A struct representing a poset.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Poset<T, F>
where
    F: PartialOrderBehaviour<Element = T>,
{
    elements: Vec<T>,
    compare: F,
}

impl<T, F> PartialOrderBehaviour for Poset<T, F>
where
    F: PartialOrderBehaviour<Element = T>,
{
    type Element = T;

    fn ge(&self, a: &Self::Element, b: &Self::Element) -> bool {
        self.compare.ge(a, b)
    }
}

impl<T, F> PosetBehaviour for Poset<T, F>
where
    F: PartialOrderBehaviour<Element = T>,
{
    type POrder = F;

    fn add(&mut self, element: impl Into<Self::Element>) {
        self.elements.push(element.into());
    }

    fn elements(&self) -> impl Iterator<Item = &Self::Element> {
        self.elements.iter()
    }

    fn replace_elements(&mut self, elements: impl IntoIterator<Item = impl Into<Self::Element>>) {
        self.elements = elements.into_iter().map(Into::into).collect();
    }

    fn replace_partial_order(&mut self, p_ord: impl Into<Self::POrder>) {
        self.compare = p_ord.into();
    }

    fn cardinality(&self) -> usize {
        self.elements.len()
    }

    fn partial_order(&self) -> &F {
        &self.compare
    }

    fn maxima(&self) -> Result<impl IntoIterator<Item = &T>, PosetError> {
        if self.elements.is_empty() {
            return Ok(vec![]);
        }

        let maxima = self
            .elements
            .iter()
            .filter(|v| !self.elements.iter().any(|w| self.gt(w, v)))
            .collect::<Vec<&T>>();

        if maxima.is_empty() {
            return Err(PosetError::NoMaxima);
        }

        Ok(maxima)
    }

    fn minima(&self) -> Result<impl IntoIterator<Item = &T>, PosetError> {
        if self.elements.is_empty() {
            return Ok(vec![]);
        }

        let minima = self
            .elements
            .iter()
            .filter(|v| !self.elements.iter().any(|w| self.lt(w, v)))
            .collect::<Vec<&T>>();

        if minima.is_empty() {
            return Err(PosetError::NoMinima);
        }

        Ok(minima)
    }

    fn cover(&self, x: &T, y: &T) -> bool {
        if !(self.gt(y, x)) {
            return false;
        }

        !self.elements.iter().any(|z| self.lt(x, z) && self.lt(z, y))
    }

    fn cover_in_pool<'a>(&self, x: &T, y: &T, pool: impl IntoIterator<Item = &'a T>) -> bool
    where
        T: 'a,
    {
        if !(self.gt(y, x)) {
            return false;
        }

        !pool.into_iter().any(|z| self.lt(x, z) && self.lt(z, y))
    }
}

impl<T, F> Poset<T, F>
where
    F: PartialOrderBehaviour<Element = T>,
{
    /// Construct a new poset with no elements but a partial order.
    pub fn new(compare: F) -> Self {
        Poset {
            elements: vec![],
            compare,
        }
    }

    /// Construct a new poset as a collection of elements and a type that implements
    /// [`PartialOrderBehaviour`].
    pub fn with_elements(elements: impl IntoIterator<Item = impl Into<T>>, compare: F) -> Self {
        Poset {
            elements: elements.into_iter().map(Into::into).collect(),
            compare,
        }
    }

    /// Return the minimal element(s) of a `pool` of elements, according to the partial order
    /// of the poset.
    pub fn minima_in_pool<'a>(&self, pool: impl IntoIterator<Item = &'a T>) -> Option<Vec<&'a T>> {
        let pool_vec: Vec<&'a T> = pool.into_iter().collect();

        let minima = pool_vec
            .iter()
            .filter(|&&v| !pool_vec.iter().any(|w| self.lt(w, v)))
            .copied()
            .collect::<Vec<&'a T>>();

        Some(minima)
    }

    /// Return a random, maximal antichain.
    #[cfg(feature = "rand")]
    #[must_use]
    pub fn rnd_maximal_antichain<R>(&self, rng: &mut R) -> Vec<&T>
    where
        R: rand::Rng + ?Sized,
    {
        let mut indices: Vec<usize> = (0..self.elements.len()).collect();
        indices.shuffle(rng);

        let mut antichain: Vec<&T> = vec![];
        'outer: for &index in &indices {
            for &a in &antichain {
                if !self.ip(a, &self.elements[index]) {
                    continue 'outer;
                }
            }
            antichain.push(&self.elements[index]);
        }

        antichain
    }
}

impl<T, F> Poset<T, F>
where
    T: PartialEq,
    F: PartialOrderBehaviour<Element = T>,
{
    /// Returns a hasse diagram of the poset.
    ///
    /// # Errors
    ///
    /// This code should not error; the indices being passed to create edges should be valid.
    #[cfg(feature = "graff")]
    pub fn hasse(&self) -> Result<Graph<&T>, graff::GraphError> {
        let mut g = Graph::<&T>::default();
        g.add_vertices(&self.elements);

        for i in 0..self.elements.len() {
            for j in 0..self.elements.len() {
                if self.cover(&self.elements[i], &self.elements[j]) {
                    g.add_edge((i, j))?;
                }
            }
        }

        Ok(g)
    }
}

impl<T, F> Poset<T, F>
where
    T: PartialEq,
    F: PartialOrderBehaviour<Element = T>,
{
    /// Return a chain decomposition of the poset.
    ///
    /// # Errors
    ///
    /// Returns an error if it cannot find any minimal elements in a non-empty pool while
    /// generating the chains, but such an element should exist if the partial order is valid.
    pub fn chain_decomposition(&self) -> Result<Vec<Vec<&T>>, Box<dyn std::error::Error>> {
        let mut vertices = self.elements.iter().collect::<Vec<&T>>();
        let mut chains = vec![];

        while !vertices.is_empty() {
            chains.push(self.chain_from_pool(&mut vertices)?);
        }

        Ok(chains)
    }

    /// Return a chain from a `pool` of elements, according to the partial order of the poset.
    ///
    /// # Errors
    ///
    /// Returns an error if it cannot find any minimal elements in a non-empty pool, but such an
    /// element should exist if the partial order is valid.
    pub fn chain_from_pool<'a>(
        &self,
        pool: &mut Vec<&'a T>,
    ) -> Result<Vec<&'a T>, Box<dyn std::error::Error>> {
        if pool.is_empty() {
            return Ok(vec![]);
        }

        let other = pool.clone();

        let elem = self
            .minima_in_pool(other.clone())
            .ok_or("there should be a minimal element")?;

        let mut chain = vec![elem[0]];

        let mut latest = chain[0];

        'outer: loop {
            for x in &other {
                if self.cover_in_pool(latest, x, other.clone()) {
                    chain.push(x);
                    latest = x;
                    continue 'outer;
                }
            }
            break;
        }

        pool.retain(|x| !chain.contains(x));

        Ok(chain)
    }
}

impl<T, F> Poset<T, F>
where
    T: Clone,
    F: PartialOrderBehaviour<Element = T>,
{
    /// Returns an [`AntichainIterator`] given a list of `chains`.
    #[must_use]
    pub fn antichains<'a>(&'a self, chains: Vec<Vec<&'a T>>) -> AntichainIterator<'a, 'a, T, F> {
        AntichainIterator::new(chains, &self.compare)
    }
}
