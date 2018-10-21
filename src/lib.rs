#![cfg_attr(feature = "simd", feature(repr_simd, platform_intrinsics))]

mod octword;
#[macro_use]
mod block;
mod argon2;
pub mod verifier;
mod workers;

pub use argon2::{argon2d_simple, argon2i_simple, defaults, Argon2, ParamErr, Variant};
