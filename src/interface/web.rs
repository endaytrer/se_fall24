use std::collections::HashMap;

use wasm_bindgen::prelude::*;


use crate::{entities::{Fisherman, Marlin, Shark}, map::{HexCell, HexCoord}};

use super::{UserAction, UserInterface};

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum WasmUserActionType {
    Move,
    Discover,
    Capture,
    Attack,
}
impl Default for WasmUserActionType {
    fn default() -> Self {
        Self::Move
    }
}

#[wasm_bindgen]
#[derive(Default, Debug, Clone, Copy)]
pub struct WasmUserAction {
    pub action_type: WasmUserActionType,
    pub param_0: HexCoord,
    pub param_1: usize,
}

#[wasm_bindgen]
#[derive(Default, Clone)]
pub struct WebUI {
    target: Option<usize>,
    fisherman: Option<Fisherman>,
    map: HashMap<HexCoord, HexCell>,
    pub input_buffer: WasmUserAction,
}

#[wasm_bindgen]
impl WebUI {

    pub fn get_fisherman(&self) -> Option<Fisherman> {
        self.fisherman.clone()
    }

    pub fn get_marlins_at(&self, coord: HexCoord) -> Vec<Marlin> {
        let Some(t) = self.map.get(&coord) else {
            return vec![];
        };
        t.marlins.clone()
    }

    pub fn get_sharks_at(&self, coord: HexCoord) -> Vec<Shark> {
        let Some(t) = self.map.get(&coord) else {
            return vec![];
        };
        t.sharks.clone()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);

}
// #[wasm_bindgen(module = "/www/index.ts")]
// extern "C" {
//     async fn wait_input();
// }
impl UserInterface for WebUI {
    fn new() -> Self {
        WebUI::default()
    }
    fn render(&mut self, target: usize, fisherman: Fisherman, map: HashMap<HexCoord, HexCell>) {
        self.target = Some(target);
        self.fisherman = Some(fisherman);
        self.map = map;
    }

    async fn input(&mut self) -> UserAction {
        todo!()
    }

    fn invalid_input(&mut self) {
        log("invalid input")
    }
    
    fn prompt(&mut self, msg: String) {
        log(&msg)
    }
}

