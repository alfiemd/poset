use crate::PartialOrderBehaviour;
use std::marker::PhantomData;

/// A struct to represent a partial order over a type `T`. It holds only the function for
/// determining whether one element is 'greater than or equal to' another element.
///
/// # Example
///
/// ```
/// # use poset::{PartialOrder, PartialOrderBehaviour};
/// # use std::cmp::Ordering;
/// let p = PartialOrder::new(|a: &usize, b: &usize| a % b == 0);
///
/// assert!(p.lt(&2, &4));
/// assert!(p.pc(&3, &5).is_none());
/// assert!(p.cp(&8, &24));
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PartialOrder<T, F>
where
    F: Fn(&T, &T) -> bool,
{
    ge: F,
    _marker: PhantomData<T>,
}

impl<T, F> PartialOrderBehaviour for PartialOrder<T, F>
where
    F: Fn(&T, &T) -> bool,
{
    type Element = T;

    fn ge(&self, a: &T, b: &T) -> bool {
        (self.ge)(a, b)
    }
}

impl<T, F> PartialOrder<T, F>
where
    F: Fn(&T, &T) -> bool,
{
    /// Construct a new `PartialOrder`.
    pub fn new(ge: F) -> PartialOrder<T, F> {
        PartialOrder {
            ge,
            _marker: PhantomData,
        }
    }
}
