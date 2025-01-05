use std::collections::HashMap;

use crate::map::{HexCell, HexCoord, HexDir};

pub trait Damageable {
    fn take_damage(&mut self, amount: i32);
    fn get_hp(&self) -> i32;
    fn get_initial_hp(&self) -> i32;
    fn is_alive(&self) -> bool;
    fn is_hurt(&self) -> bool;
}

pub trait Attacker<T: Damageable> {
    fn attack(&self, target: &mut T);
}


#[derive(Debug, Clone)]
pub struct Marlin {
    discovered: bool, // Indicates whether this Marlin has been discovered
    hp: i32,
}

impl Marlin {
    const INITIAL_HP: i32 = 4;
    pub const MOVE_RADIUS: i32 = 1;
    pub const fn new() -> Self {
        Marlin {
            discovered: false,
            hp: Self::INITIAL_HP,
        }
    }
    #[inline]
    pub fn is_discovered(&self) -> bool {
        self.discovered
    }
}

impl PartialEq for Marlin {
    fn eq(&self, other: &Self) -> bool {
        self as *const Marlin == other as *const Marlin
    }
}
impl Damageable for Marlin {
    fn take_damage(&mut self, amount: i32) {
        self.hp -= amount;
    }

    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    fn is_hurt(&self) -> bool {
        self.hp < Self::INITIAL_HP
    }
    
    #[inline]
    fn get_hp(&self) -> i32 {
        self.hp
    }
    
    #[inline]
    fn get_initial_hp(&self) -> i32 {
        Self::INITIAL_HP
    }
}

#[derive(Debug, Clone)]
pub struct Shark {
    hp: i32,
}
impl Shark {
    const INITIAL_HP: i32 = 2;
    const ATTACK_POWER: i32 = 1;
    pub const MOVE_RADIUS: i32 = 1;
    pub const VISUAL_RADIUS: i32 = 2;
    pub const SMELL_RADIUS: i32 = 3;
    pub const fn new() -> Self {
        Shark {
            hp: Self::INITIAL_HP
        }
    }
}

impl Damageable for Shark {
    fn take_damage(&mut self, amount: i32) {
        self.hp -= amount;
    }

    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    fn is_hurt(&self) -> bool {
        self.hp < Self::INITIAL_HP
    }
    
    #[inline]
    fn get_hp(&self) -> i32 {
        self.hp
    }
    
    #[inline]
    fn get_initial_hp(&self) -> i32 {
        Self::INITIAL_HP
    }
}

impl <T: Damageable> Attacker<T> for Shark {
    fn attack(&self, target: &mut T) {
        target.take_damage(Self::ATTACK_POWER);
    }
}


#[derive(Debug, Clone)]
pub struct Fisherman {
    coordinate: HexCoord,
    hp: i32,
    initial_hp: i32,
    attack_power: i32,
    captured_marlins: usize,
    capture_success_rate: f32,
}
impl Fisherman {
    pub const HARBOR_COORD: HexCoord = HexCoord::ZERO;
    const MOVE_RADIUS: i32 = 1;
    const CAPTURE_RADIUS: i32 = 1;
    const DISCOVER_RADIUS: i32 = 2;
    pub const VISUAL_RADIUS: i32 = 4;
    const CAPTURE_FAIL_DAMAGE: i32 = 1;
    pub fn new(initial_hp: i32, attack_power: i32, capture_success_rate: f32) -> Self {
        Self {
            coordinate: Self::HARBOR_COORD,
            hp: initial_hp,
            initial_hp,
            attack_power,
            captured_marlins: 0,
            capture_success_rate,
        }
    }
    pub fn operate(&mut self, dir: HexDir) -> bool {
        let new_coord = self.coordinate + dir;
        if self.coordinate.distance(&new_coord) > Self::MOVE_RADIUS {
            return false
        }
        self.coordinate = new_coord;
        true
    }
    pub fn discover_marlins(&self, grid: &mut HashMap<HexCoord, HexCell>) -> bool {
        if self.coordinate == Self::HARBOR_COORD {
            return false;
        }
        for coord in self.coordinate.within_radius(Self::DISCOVER_RADIUS) {
            if let Some(cell) = grid.get_mut(&coord) {
                cell.marlins.iter_mut().for_each(|m| m.discovered = true);
            }
        }
        true
    }
    /// Method to capture Marlins in a selected cell
    /// return if input is correct.
    pub fn capture_marlins(&mut self, coord: HexCoord, grid: &mut HashMap<HexCoord, HexCell>) -> bool {
        if self.coordinate == Self::HARBOR_COORD {
            return false;
        }
        // Check if the target coordinate is within the capture radius

        if self.coordinate.distance(&coord) > Self::CAPTURE_RADIUS {
            return false;
        }

        // Check if there are discovered Marlins in the selected cell
        let Some(cell) = grid.get_mut(&coord) else {
            return true;
        };
        let new_marlins = cell.marlins.iter().filter_map(|s| {
            if !s.discovered {
                return Some(s.clone()) // keep all
            }
            if self.attempt_capture() {
                // success, remove marlin
                None
            } else {
                let mut new_marlin = s.clone();
                new_marlin.take_damage(Self::CAPTURE_FAIL_DAMAGE);
                Some(new_marlin)
            }
        }).collect::<Vec<_>>();
        // don't count died marlins, so calculate capture num before killing marlins
        let capture_num = cell.marlins.len() - new_marlins.len();
        // don't kill died marlins yet. Recycle by the end of turn
        cell.marlins = new_marlins;

        // add to captured_marlins
        self.captured_marlins += capture_num;
        true
    }

    // Method to attempt to capture a Marlin based on success rate
    fn attempt_capture(&self) -> bool {
        // Simulate capture based on success rate
        let success_chance = rand::random::<f32>(); // Random number between 0.0 and 1.0
        success_chance < self.capture_success_rate
    }
    pub fn attack_shark(&self, shark: Option<&mut Shark>) -> bool {
        if self.coordinate == Self::HARBOR_COORD {
            return false;
        }
        let Some(shark) = shark else {
            return false;
        };
        self.attack(shark);
        true
    }
    #[inline]
    pub fn get_coord(&self) -> HexCoord {
        self.coordinate
    }
    #[inline]
    pub fn get_captured_marlins(&self) -> usize {
        self.captured_marlins
    }
}


impl Damageable for Fisherman {
    fn take_damage(&mut self, amount: i32) {
        self.hp -= amount;
    }

    fn is_alive(&self) -> bool {
        self.hp > 0
    }
    
    fn is_hurt(&self) -> bool {
        self.hp < self.initial_hp
    }
    #[inline]
    fn get_hp(&self) -> i32 {
        self.hp
    }

    #[inline]
    fn get_initial_hp(&self) -> i32 {
        self.initial_hp
    }
}

// Implement the Attacker trait for Fisherman targeting Sharks
impl Attacker<Shark> for Fisherman {
    fn attack(&self, target: &mut Shark) {
        target.take_damage(self.attack_power);
    }
}