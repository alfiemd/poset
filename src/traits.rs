use crate::PosetError;
use std::cmp::Ordering;

/// A trait to represent the behaviour of a partial order.
///
/// One needs to define the 'greater than or equal to' behaviour. But implementing this trait is
/// not a *guarantee* that the type is a partial order; this requires care in the function you
/// decide to implement.
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
