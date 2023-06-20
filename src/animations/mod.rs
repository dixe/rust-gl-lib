#![allow(for_loops_over_fallibles)]
use crate::na;
use crate::math::clamp01;
use crate::animations::skeleton::*;
use std::collections::{HashMap, HashSet};

pub mod skeleton;

pub mod sheet_animation;

mod types;
pub use self::types::*;

pub mod gltf_animation;
