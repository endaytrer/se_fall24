#[cfg(feature="cli")]
pub mod cli;

#[cfg(feature="web")]
pub mod web;


use crate::{entities::Fisherman, map::{HexCell, HexCoord, HexDir}};
use std::{collections::HashMap, future::Future};


#[derive(Debug, Clone, Copy)]
pub enum UserAction {
    Move(HexDir),
    Discover,
    Capture(HexDir),
    Attack(HexCoord, usize)
}
pub trait UserInterface {
    fn new() -> Self;
    fn render(&mut self, target: usize, fisherman: Fisherman, map: HashMap<HexCoord, HexCell>);
    fn input(&mut self) -> impl Future<Output = UserAction>;
    fn invalid_input(&mut self);
    fn prompt(&mut self, msg: String);
}
