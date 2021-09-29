//! A libaray that aim to make use of opengl eaiser to start with, or add
//! to project where graphics is not the main focus.
//! While still allowing all the low level access needed for performance
//! and customizability.
//!
//! # Examples
//!
//! See [examples](https://github.com/dixe/rust-gl-lib/tree/master/examples)
//!

pub use nalgebra as na;

pub mod gl;
pub mod shader;
pub mod buffer;
pub mod objects;
pub mod camera;
pub mod text_rendering;
