
use std::{i32, ops::{Add, Mul, Sub}};

use wasm_bindgen::prelude::*;

use crate::entities::{Marlin, Shark};

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct HexCoord {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

impl HexCoord {
    pub const ZERO: Self = Self { q: 0, r: 0, s: 0 };
}
#[wasm_bindgen]
impl HexCoord {
    #[wasm_bindgen(constructor)]
    pub fn new(q: i32, r: i32, s: i32) -> Self {
        assert!(q + r + s == 0);
        Self {
            q, r, s
        }
    }
    // Method to get an iterator of coordinates within a given Manhattan radius
    pub fn within_radius(&self, radius: i32) -> Vec<HexCoord> {
        let mut coords = Vec::new();

        for dq in -radius..=radius {
            for dr in i32::max(-dq, 0) - radius..=i32::min(-dq, 0) + radius {
                let ds = -dq - dr; // Ensure q + r + s = 0
                let new_coord = HexCoord {
                    q: self.q + dq,
                    r: self.r + dr,
                    s: self.s + ds,
                };
                coords.push(new_coord)
            }
        }
        coords
    }
    pub fn on_radius(&self, radius: i32) -> Vec<HexCoord> {
        let mut coords = Vec::new();

        for dq in -radius..=radius {
            let drs = if dq.abs() == radius {
                (i32::max(-dq, 0) - radius..=i32::min(-dq, 0) + radius).collect()
            } else {
                vec![i32::max(-dq, 0) - radius, i32::min(-dq, 0) + radius]
            };
            for dr in drs {
                let ds = -dq - dr; // Ensure q + r + s = 0
                
                let new_coord = HexCoord {
                    q: self.q + dq,
                    r: self.r + dr,
                    s: self.s + ds,
                };
                
                coords.push(new_coord)
            }
        }
        coords
    }

    // Method to calculate Manhattan distance
    pub fn distance(&self, other: &HexCoord) -> i32 {
        ((self.q - other.q).abs() + (self.r - other.r).abs() + (self.s - other.s).abs()) / 2
    }
    // Create a new HexCell
}
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexDir {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

impl HexDir {
    pub const ZERO: HexDir = HexDir { q: 0, r: 0, s: 0 };
    pub const NORTH: HexDir = HexDir { q: 0, r: 1, s: -1 };
    pub const NORTHEAST: HexDir = HexDir { q: 1, r: 0, s: -1 };
    pub const SOUTHEAST: HexDir = HexDir { q: 1, r: -1, s: 0 };
    pub const SOUTH: HexDir = HexDir { q: 0, r: -1, s: 1 };
    pub const SOUTHWEST: HexDir = HexDir { q: -1, r: 0, s: 1 };
    pub const NORTHWEST: HexDir = HexDir { q: -1, r: 1, s: 0 };

}
#[wasm_bindgen]
impl HexDir {
    
    #[wasm_bindgen(constructor)]
    pub fn new(q: i32, r: i32, s: i32) -> Self {
        assert!(q + r + s == 0);
        Self {q, r, s}
    }
    
    pub unsafe fn new_unchecked(q: i32, r: i32, s: i32) -> Self {
        debug_assert!(q + r + s == 0);
        Self {q, r, s}
    }
}

impl Add for HexDir {
    type Output = HexDir;

    fn add(self, other: HexDir) -> HexDir {
        HexDir {
            q: self.q + other.q,
            r: self.r + other.r,
            s: self.s + other.s,
        }
    }
}

impl Sub for HexDir {
    type Output = HexDir;

    fn sub(self, other: HexDir) -> HexDir {
        HexDir {
            q: self.q - other.q,
            r: self.r - other.r,
            s: self.s - other.s,
        }
    }
}

impl Mul<i32> for HexDir {
    type Output = HexDir;

    fn mul(self, scalar: i32) -> HexDir {
        HexDir {
            q: self.q * scalar,
            r: self.r * scalar,
            s: self.s * scalar,
        }
    }
}
impl Add<HexDir> for HexCoord {
    type Output = HexCoord;

    fn add(self, direction: HexDir) -> HexCoord {
        HexCoord {
            q: self.q + direction.q,
            r: self.r + direction.r,
            s: self.s + direction.s,
        }
    }
}

impl Sub<HexDir> for HexCoord {
    type Output = HexCoord;

    fn sub(self, other: HexDir) -> HexCoord {
        HexCoord {
            q: self.q - other.q,
            r: self.r - other.r,
            s: self.s - other.s,
        }
    }
}
impl Sub<HexCoord> for HexCoord {
    type Output = HexDir;

    fn sub(self, other: HexCoord) -> HexDir {
        HexDir {
            q: self.q - other.q,
            r: self.r - other.r,
            s: self.s - other.s,
        }
    }
}


#[derive(Debug, Default, Clone)]
pub struct HexCell {
    pub marlins: Vec<Marlin>,
    pub sharks: Vec<Shark>,
}

