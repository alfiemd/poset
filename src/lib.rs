#![deny(missing_docs)]
#![deny(unused_crate_dependencies)]

//! # poset
//!
//! A simple implementation of posets.
//!
//! [Rust](https://www.rust-lang.org/) already provides some tools to analyse posets; we can define
//! partial order on a type `T` by implementing [`PartialOrd`]. Doing so affords us the ability to
//! use operators as we naturally would, writing things like `a >= b`.
//!
//! But implementing [`PartialOrd`] is not ideal when we wish to consider multiple partial orders
//! on a type. For example, what if we wanted to consider the natural numbers (i.e. [`u32`]) with
//! the divisbility relation: `a >= b` if and only if `a % b == 0`.
//!
//! The purpose of this crate is to provide both an ergonomic way to work with various partial
//! orders, and also helpful tools to study those associated posets (by generating things like
//! [Hasse diagrams](https://en.wikipedia.org/wiki/Hasse_diagram)). It is within the future scope
//! of this crate to provide such things for more general relations, too.
//!
//! If you want the crate to be finished quicker, then you could consider contributing. :)
//!
//! # Example
//!
//! ```rust
//! # use poset::{Poset, PartialOrder, PartialOrderBehaviour};
//! # use std::error::Error;
//! # fn main() -> Result<(), Box<dyn Error>> {
//! // `a >= b` if and only if `b` divides `a`
//! let divis = PartialOrder::new(|a: &i32, b: &i32| a % b == 0);
//!
//! // 3 is *comparable* with 6
//! assert!(divis.cp(&3, &6));
//! // 4 is *incomparable* with 6
//! assert!(divis.ip(&4, &6));
//! // 3 divides 15
//! assert!(divis.lt(&3, &15));
//!
//! let pos = Poset::with_elements(1..16, divis);
//! let chain_decomp = pos.chain_decomposition()?;
//! let antichains = pos.antichains(chain_decomp);
//!
//! // see https://oeis.org/A051026
//! assert_eq!(antichains.count(), 1133);
//!
//! // if you want to generate Hasse diagrams
//! #[cfg(feature = "graff")]
//! {
//!     use graff::{Graph, GraphBehaviour};
//!
//!     let g = pos.hasse()?;
//!     assert_eq!(g.edge_count(), 19);
//! }
//! # Ok(())
//! # }
//! ```

mod antichain_iterator;
mod errors;
mod partial_order;
mod poset;
mod traits;

pub use antichain_iterator::*;
pub use errors::*;
pub use partial_order::PartialOrder;
pub use poset::Poset;
pub use traits::*;
