//! Assert for approximations.
//!
//! These macros compare numbers, such as two floating point numbers,
//! where one number may be very close to another number but not quite equal.
//!
//! * [`assert_approx_eq!(a, b)`](macro@crate::assert_approx_eq) ≈ a is approximately equal to b
//!
//! * [`assert_approx_ne!(a, b)`](macro@crate::assert_approx_ne) ≈ a is approximately not equal to b
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a: f32 = 1.0000001;
//! let b: f32 = 1.0000011;
//! assert_approx_eq!(a, b);
//! ```

pub mod assert_approx_eq;
pub mod assert_approx_ne;

//pub mod assert_approx_eq_using_absolute_error;
//pub mod assert_approx_ne_using_absolute_error;

pub mod assert_approx_eq_using_relative_error;
//pub mod assert_approx_ne_using_relative_error;
