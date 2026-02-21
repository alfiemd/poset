use crate::PosetError;
use std::cmp::Ordering;

/// A trait to represent the behaviour of a partial order.
///
/// The intended use is to implement [`PartialOrderBehaviour::ge`] and use the provided default
/// methods for the other relation predicates. Overriding methods like `eq`, `le`, `lt`, `gt`,
/// `cp`, or `ip` is possible, but those overrides must remain consistent with `ge`, which requires
/// the implementer to take care.
pub trait PartialOrderBehaviour {
    /// A type representing the elements that a partial order compares.
    type Element;

    /// Returns a partial order comparison between two elements.
    fn pc(&self, a: &Self::Element, b: &Self::Element) -> Option<Ordering> {
        match (self.ge(a, b), self.ge(b, a)) {
            (true, true) => Some(Ordering::Equal),
            (true, _) => Some(Ordering::Greater),
            (_, true) => Some(Ordering::Less),
            _ => None,
        }
    }

    /// Returns whether `a <= b` in the partial order.
    fn le(&self, a: &Self::Element, b: &Self::Element) -> bool {
        self.ge(b, a)
    }

    /// Returns whether `a >= b` in the partial order.
    fn ge(&self, a: &Self::Element, b: &Self::Element) -> bool;

    /// Returns whether `a > b` in the partial order.
    fn gt(&self, a: &Self::Element, b: &Self::Element) -> bool {
        self.ge(a, b) && !self.ge(b, a)
    }

    /// Returns whether `a < b` in the partial order.
    fn lt(&self, a: &Self::Element, b: &Self::Element) -> bool {
        self.ge(b, a) && !self.ge(a, b)
    }

    /// Returns whether `a == b` in the partial order.
    fn eq(&self, a: &Self::Element, b: &Self::Element) -> bool {
        self.ge(a, b) && self.ge(b, a)
    }

    /// Returns whether `a` is *incomparable* with `b` in the partial order.
    fn ip(&self, a: &Self::Element, b: &Self::Element) -> bool {
        !self.ge(a, b) && !self.ge(b, a)
    }

    /// Returns whether `a` is *comparable* with `b` in the partial order.
    fn cp(&self, a: &Self::Element, b: &Self::Element) -> bool {
        self.ge(a, b) || self.ge(b, a)
    }

    /// Validate reflexivity for a finite set of elements.
    ///
    /// Returns `true` iff `a >= a` holds for every provided element.
    ///
    /// This assumes deterministic behaviour from the relation implementation.
    fn validate_reflexive<'a>(&self, elements: impl IntoIterator<Item = &'a Self::Element>) -> bool
    where
        Self::Element: 'a,
    {
        elements.into_iter().all(|a| self.ge(a, a))
    }

    /// Validate transitivity for a finite set of elements.
    ///
    /// Returns `true` iff whenever `a >= b` and `b >= c`, we also have `a >= c`, for all triples
    /// of provided elements.
    ///
    /// This assumes deterministic behaviour from the relation implementation.
    fn validate_transitive(&self, elements: &[&Self::Element]) -> bool {
        for a in elements {
            for b in elements {
                if self.ge(a, b) {
                    for c in elements {
                        if self.ge(b, c) && !self.ge(a, c) {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }
}

/// A trait representing the behaviour of a poset.
pub trait PosetBehaviour: PartialOrderBehaviour {
    /// A type representing the partial order function.
    type POrder: PartialOrderBehaviour<Element = Self::Element>;

    /// Add an element to the poset.
    fn add(&mut self, element: impl Into<Self::Element>);

    /// Returns an iterable over the elements of the poset.
    fn elements(&self) -> impl Iterator<Item = &Self::Element>;

    /// Replace the elements of the poset.
    fn replace_elements(&mut self, elements: impl IntoIterator<Item = impl Into<Self::Element>>);

    /// Replace the partial order of the poset.
    fn replace_partial_order(&mut self, p_ord: impl Into<Self::POrder>);

    /// Return the number of elements in the poset.
    fn cardinality(&self) -> usize;

    /// Return a reference to the partial order of the poset.
    fn partial_order(&self) -> &Self::POrder;

    /// Validate reflexivity on the current elements of the poset.
    ///
    /// This assumes deterministic behaviour from the relation implementation.
    fn is_reflexive(&self) -> bool {
        self.partial_order().validate_reflexive(self.elements())
    }

    /// Validate transitivity on the current elements of the poset.
    ///
    /// This assumes deterministic behaviour from the relation implementation.
    fn is_transitive(&self) -> bool {
        let elements = self.elements().collect::<Vec<_>>();
        self.partial_order().validate_transitive(&elements)
    }

    /// Return the maximal element(s) of the poset, which must exist unless the poset has no
    /// elements.
    ///
    /// # Errors
    ///
    /// This function will return a [`PosetError::NoMaxima`] if the poset is non-empty but has
    /// no maximal elements, indicating that the chosen partial order is invalid.
    fn maxima(&self) -> Result<impl IntoIterator<Item = &Self::Element>, PosetError>;

    /// Return the minimal element(s) of the poset, which must exist unless the poset has no
    /// elements.
    ///
    /// # Errors
    ///
    /// This function will return a [`PosetError::NoMinima`] if the poset is non-empty but has
    /// no maximal elements, indicating that the chosen partial order is invalid.
    fn minima(&self) -> Result<impl IntoIterator<Item = &Self::Element>, PosetError>;

    /// Returns whether `x` is covered by `y` in the poset.
    fn cover(&self, x: &Self::Element, y: &Self::Element) -> bool;

    /// Returns whether 'x' is covered by 'y' in the set of elements `pool`, according to the
    /// partial order of the poset.
    fn cover_in_pool<'a>(
        &self,
        x: &Self::Element,
        y: &Self::Element,
        pool: impl IntoIterator<Item = &'a Self::Element>,
    ) -> bool
    where
        Self::Element: 'a;
}
