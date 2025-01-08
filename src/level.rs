use std::collections::HashMap;
use rand::{seq::{IteratorRandom, SliceRandom}, Rng};
use wasm_bindgen::prelude::*;

use crate::{entities::{Attacker, Damageable, Fisherman, Marlin, Shark}, map::{HexCell, HexCoord, HexDir}};

#[derive(Debug, Clone, Copy)]
pub enum UserAction {
    Move(HexDir),
    Discover,
    Capture(HexDir),
    Attack(HexCoord, usize)
}
pub trait UserInterface {
    fn new() -> Self;
    fn render(&mut self, game: &Game);
    fn start(&mut self);
    fn invalid_input(&mut self);
    fn prompt(&mut self, msg: String);
}


pub struct Level {
    target: usize,
    map: HashMap<HexCoord, HexCell>,
    fisherman: Fisherman,
    marlin_spawn_probability: Box<dyn Fn(i32) -> f32>,
    shark_spawn_probability: Box<dyn Fn(i32) -> f32>,
}
fn sample_poisson(lambda: f32) -> usize {
    let exp_lambda = (-lambda).exp();
    let mut result = 0;
    let mut p = 1.0;
    loop {
        p *= rand::thread_rng().gen::<f32>();
        if p < exp_lambda {
            return result;
        }
        result += 1;
    }
    
}
impl Level {
    const MARLIN_SPAWN_RADIUS: i32 = Fisherman::VISUAL_RADIUS + Marlin::MOVE_RADIUS;
    const SHARK_SPAWN_RADIUS: i32 = Fisherman::VISUAL_RADIUS + Shark::MOVE_RADIUS;

    pub fn new(target: usize, initial_hp: i32, attack_power: i32, capture_success_rate: f32, marlin_spawn_probability: Box<dyn Fn(i32) -> f32>, shark_spawn_probability: Box<dyn Fn(i32) -> f32>) -> Self {
        Self {
            target,
            map: HashMap::new(),
            fisherman: Fisherman::new(initial_hp, attack_power, capture_success_rate),
            marlin_spawn_probability,
            shark_spawn_probability
        }
    }
    fn action_marlins(&mut self) {
        let marlins_to_move = self.map.keys().copied().collect::<Vec<_>>().into_iter().flat_map(|c| {
            let Some(cell) = self.map.get_mut(&c) else {
                return vec![]
            };
            cell.marlins.drain(..).map(|m| (c, m)).collect()
        }).collect::<Vec<_>>();

        // Process each marlin's movement
        for (current_coord, marlin) in marlins_to_move {
            // Get possible neighboring coordinates
            let neighbors = current_coord.within_radius(Marlin::MOVE_RADIUS).into_iter().filter(|p| *p != Fisherman::HARBOR_COORD).collect::<Vec<_>>(); // Get neighbors within 1 radius, but not harbor
            // randomly choose a neighbor with least sharks
            let new_coord = neighbors.choose(&mut rand::thread_rng()).unwrap();
            if let Some(cell) = self.map.get_mut(new_coord) {
                cell.marlins.push(marlin);
            } else {
                self.map.insert(*new_coord, HexCell {
                    marlins: vec![marlin],
                    sharks: vec![],
                });
            }
        }
    }

    fn action_sharks(&mut self) {
        let fisherman_coord = self.fisherman.get_coord();
        // if Fisherman's position has a shark, each shark in the position will attack the fisherman
        if let Some(cell) = self.map.get(&fisherman_coord) {
            cell.sharks.iter().for_each(|s| s.attack(&mut self.fisherman));
        }
        // if shark's position has marlins, it will randomly attack one of the marlins.
        let sharks_to_move = self.map.keys().copied().collect::<Vec<_>>().into_iter().filter(|p| *p != fisherman_coord).flat_map(|c| {
            let Some(cell) = self.map.get_mut(&c) else {
                return vec![]
            };
            // no move.
            {
                let HexCell{sharks, marlins} = cell;
                if sharks.is_empty() {
                    return vec![]
                }
                // attack marlins in the cell
                if !marlins.is_empty() {
                    for s in sharks {
                        s.attack(marlins.choose_mut(&mut rand::thread_rng()).unwrap());
                    }
                    return vec![];
                }
            }
            // with move
            let sharks = cell.sharks.drain(..).collect::<Vec<_>>();
            let move_targets = c.within_radius(Shark::MOVE_RADIUS).into_iter().filter(|p| *p != Fisherman::HARBOR_COORD);

            if fisherman_coord.distance(&c) <= Shark::VISUAL_RADIUS {
                // if fisherman is within the visual radius of shark, shark will swim to the closest position
                let mut closest_dist = i32::MAX;
                let mut closest_coords = vec![];
                for t in move_targets {
                    let new_dist = fisherman_coord.distance(&t);
                    if new_dist < closest_dist {
                        closest_dist = new_dist;
                        closest_coords = vec![t]
                    } else if new_dist == closest_dist {
                        closest_coords.push(t);
                    }
                }
                sharks.into_iter().map(|s| (s, *closest_coords.choose(&mut rand::thread_rng()).unwrap())).collect()
            } else {
                // if there is hurt marlins within smell radius of shark, shark will choose the closest marlin and swim to the closest position to that marlin.
                for radius in 1..=Shark::SMELL_RADIUS {
                    let hurt_marlin_positions = c.on_radius(radius).into_iter().filter(|target_pos| {
                        let Some(cell) = self.map.get(target_pos) else {
                            return false
                        };
                        cell.marlins.iter().any(|p| p.is_hurt())
                    }).collect::<Vec<_>>();
                    if hurt_marlin_positions.is_empty() {
                        continue;
                    }
                    return sharks.into_iter().map(|s| (s, {
                        let target_pos = hurt_marlin_positions.choose(&mut rand::thread_rng()).unwrap();
                        let mut closest_dist = i32::MAX;
                        let mut closest_coords: Vec<HexCoord> = vec![];
                        for t in move_targets.clone() {
                            let new_dist = target_pos.distance(&t);
                            if new_dist < closest_dist {
                                closest_dist = new_dist;
                                closest_coords = vec![t]
                            } else if new_dist == closest_dist {
                                closest_coords.push(t);
                            }
                        }
                        *closest_coords.choose(&mut rand::thread_rng()).unwrap()
                    })).collect()
                }
                // there are no marlins and fisherman available, randomly swims.
                sharks.into_iter().map(|s| 
                    (s, move_targets.clone().choose(&mut rand::thread_rng()).unwrap())
                ).collect()
            }
        }).collect::<Vec<_>>();
        for (shark, target_coord) in sharks_to_move {
            if let Some(cell) = self.map.get_mut(&target_coord) {
                cell.sharks.push(shark);
            } else {
                self.map.insert(target_coord, HexCell {
                    marlins: vec![],
                    sharks: vec![shark],
                });
            }
        }
    }
    fn kill_died_creatures(&mut self) {
        for c in self.map.keys().copied().collect::<Vec<_>>() {
            
            let Some(HexCell{sharks, marlins}) = &mut self.map.get_mut(&c) else {
                continue;
            };
            *sharks = sharks.drain(..).filter(|p| p.is_alive()).collect();
            *marlins = marlins.drain(..).filter(|p| p.is_alive()).collect();

        }
    }
    fn despawn_cells(&mut self) {
        self.map.retain(|k, v| {
            self.fisherman.get_coord().distance(k) <= Fisherman::VISUAL_RADIUS && (!v.marlins.is_empty() || !v.sharks.is_empty())
        });
    }
    fn test_game_over(&self) -> Option<Result<usize, usize>> {
        // test alive first
        if !self.fisherman.is_alive() {
            return Some(Err(self.fisherman.get_captured_marlins()))
        }
        if self.fisherman.get_coord() == Fisherman::HARBOR_COORD && self.fisherman.get_captured_marlins() >= self.target {
            return Some(Ok(self.fisherman.get_captured_marlins()))
        }
        None
    }
    fn spawn_new_creatures(&mut self) {
        for radius in Fisherman::VISUAL_RADIUS+1..=Self::MARLIN_SPAWN_RADIUS {
            let cells = self.fisherman.get_coord().on_radius(radius);
            for cell in cells {
                let from_center = cell.distance(&Fisherman::HARBOR_COORD);
                let lambda = (self.marlin_spawn_probability)(from_center);
                let num = sample_poisson(lambda);
                let marlins = vec![Marlin::new(); num];
                
                if let Some(v) = self.map.get_mut(&cell) {
                    v.marlins.extend(marlins)
                } else {
                    self.map.insert(cell, HexCell { marlins, sharks: vec![] });
                }
            }
        }
        for radius in Fisherman::VISUAL_RADIUS+1..=Self::SHARK_SPAWN_RADIUS {
            let cells = self.fisherman.get_coord().on_radius(radius);
            for cell in cells {
                let from_center = cell.distance(&Fisherman::HARBOR_COORD);
                let lambda = (self.shark_spawn_probability)(from_center);
                let num = sample_poisson(lambda);
                let sharks = vec![Shark::new(); num];

                if let Some(v) = self.map.get_mut(&cell) {
                    v.sharks.extend(sharks)
                } else {
                    self.map.insert(cell, HexCell { marlins: vec![], sharks });
                }
            }
        }
    }
    // fn action_player(&mut self, interface: &mut impl UserInterface) {
    //     interface.render(self.target, self.get_fisherman(), self.get_map());
    //     loop {
    //         if match interface.input() {
    //             crate::interface::UserAction::Move(dir) => self.fisherman.operate(dir),
    //             crate::interface::UserAction::Discover => self.fisherman.discover_marlins(&mut self.map),
    //             crate::interface::UserAction::Capture(dir) => self.fisherman.capture_marlins(self.fisherman.get_coord() + dir, &mut self.map),
    //             crate::interface::UserAction::Attack(coord, index) => {
    //                 self.fisherman.attack_shark(self.map.get_mut(&coord).and_then(|c| c.sharks.get_mut(index)))
    //             },
    //         } {
    //             break;
    //         }
    //         interface.invalid_input();
    //     }
    // }
    // fn turn(&mut self, interface: &mut impl UserInterface) -> Option<Result<usize, usize>> {
    //     self.spawn_new_creatures();
    //     self.action_player(interface);
    //     self.action_marlins();
    //     self.action_sharks();
    //     self.kill_died_creatures();
    //     self.despawn_cells();
    //     self.test_game_over()
    // }
    pub fn advance(&mut self, input: UserAction) -> Result<Option<Result<usize, usize>>, ()> {
        if !match input {
            UserAction::Move(dir) => self.fisherman.operate(dir),
            UserAction::Discover => self.fisherman.discover_marlins(&mut self.map),
            UserAction::Capture(dir) => self.fisherman.capture_marlins(self.fisherman.get_coord() + dir, &mut self.map),
            UserAction::Attack(coord, index) => {
                self.fisherman.attack_shark(self.map.get_mut(&coord).and_then(|c| c.sharks.get_mut(index)))
            },
        } {
            return Err(())
        }
        self.action_marlins();
        self.action_sharks();
        self.kill_died_creatures();
        self.despawn_cells();
        if let Some(ans) = self.test_game_over() {
            return Ok(Some(ans));
        }
        self.spawn_new_creatures();
        Ok(None)
    }

    pub fn get_fisherman(&self) -> Fisherman {
        self.fisherman.clone()
    }

    pub fn get_map(&self) -> HashMap<HexCoord, HexCell> {
        self.map.clone()
    }


    // pub fn start(&mut self, interface: &mut impl UserInterface) -> Result<usize, usize> {
    //     loop {
    //         if let Some(res) = self.turn(interface) {
    //             return res;
    //         } else {
    //             continue;
    //         }
    //     }
    // }
}


#[wasm_bindgen]
pub enum InputResult {
    InvalidInput,
    Ok,
    LevelPassed,
    LevelFailed,
    GamePassed,
}
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
impl WasmUserAction {
    pub fn move_action(dir: HexCoord) -> Self {
        Self {
            action_type: WasmUserActionType::Move,
            param_0: dir,
            param_1: 0,
        }
    }
    pub fn discover_action() -> Self {
        Self {
            action_type: WasmUserActionType::Discover,
            param_0: HexCoord::ZERO,
            param_1: 0,
        }
    }
    pub fn capture_action(dir: HexCoord) -> Self {
        Self {
            action_type: WasmUserActionType::Capture,
            param_0: dir,
            param_1: 0,
        }
    }
    pub fn attack_action(coord: HexCoord, index: usize) -> Self {
        Self {
            action_type: WasmUserActionType::Attack,
            param_0: coord,
            param_1: index,
        }
    }
}
impl Into<UserAction> for WasmUserAction {
    fn into(self) -> UserAction {
        match self.action_type {
            WasmUserActionType::Move => UserAction::Move(self.param_0 - HexCoord::ZERO),
            WasmUserActionType::Discover => UserAction::Discover,
            WasmUserActionType::Capture => UserAction::Capture(self.param_0 - HexCoord::ZERO),
            WasmUserActionType::Attack => UserAction::Attack(self.param_0, self.param_1),
        }
    }
}

impl From<UserAction> for WasmUserAction {
    fn from(value: UserAction) -> Self {
        match value {
            UserAction::Move(hex_dir) => Self {
                action_type: WasmUserActionType::Move,
                param_0: HexCoord::ZERO + hex_dir,
                param_1: 0,
            },
            UserAction::Discover => Self {
                action_type: WasmUserActionType::Discover,
                param_0: HexCoord::ZERO,
                param_1: 0,
            },
            UserAction::Capture(hex_dir) => Self {
                action_type: WasmUserActionType::Capture,
                param_0: HexCoord::ZERO + hex_dir,
                param_1: 0
            },
            UserAction::Attack(hex_coord, index) => Self {
                action_type: WasmUserActionType::Attack,
                param_0: hex_coord,
                param_1: index
            },
        }
    }
}
#[wasm_bindgen]
pub struct Game {
    current_level: Level,
    current_score: usize,
    levels: std::vec::IntoIter<Level>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
    
        let level0 = Level::new(5, 5, 1, 0.5, Box::new(|_| 0.3), Box::new(|_| 0.0));
        let level1 = Level::new(10, 5, 1, 0.5, Box::new(|_| 0.3), Box::new(|_| 0.05));
        let level2 = Level::new(15, 5, 1, 0.5, Box::new(|_| 0.25), Box::new(|_| 0.07));
        let level_iter = vec![level1, level2].into_iter();
        Self {
            current_level: level0,
            levels: level_iter,
            current_score: 0,
        }
    }

    #[inline]
    pub fn get_fisherman(&self) -> Fisherman {
        self.current_level.fisherman.clone()
    }

    #[inline]
    pub fn get_target(&self) -> usize {
        self.current_level.target
    }

    pub fn get_shark_num_at(&self, coord: &HexCoord) -> usize {
        let Some(cell) = self.current_level.map.get(coord) else {
            return 0;
        };
        cell.sharks.len()
    }

    pub fn get_discovered_marlin_num_at(&self, coord: &HexCoord) -> usize {
        let Some(cell) = self.current_level.map.get(coord) else {
            return 0;
        };
        cell.marlins.iter().filter(|t| t.is_discovered()).count()
    }

    pub fn get_nth_shark_at(&self, coord: &HexCoord, n: usize) -> Option<Shark> {
        Some(self.current_level.map.get(coord)?.sharks.get(n)?.clone())
    }
    #[inline]
    pub fn get_score(&self) -> usize{
        self.current_score
    }
    pub fn handle_action(&mut self, input: WasmUserAction) -> InputResult {
        let input = input.into();
        let Ok(res) = self.current_level.advance(input) else {
            return InputResult::InvalidInput;
        };
        let Some(game_over) = res else { return InputResult::Ok };
        match game_over {
            Ok(score) => {
                self.current_score = score;
                let Some(next_level) = self.levels.next() else {
                    return InputResult::GamePassed;
                };
                self.current_level = next_level;
                InputResult::LevelPassed
            },
            Err(score) => { 
                self.current_score = score;
                InputResult::LevelFailed
            },
        }
    }
}
