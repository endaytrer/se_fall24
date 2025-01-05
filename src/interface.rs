use termion::input::TermRead;

use crate::{entities::Fisherman, map::{HexCell, HexCoord, HexDir}};

use std::{collections::HashMap, io::stdin};

pub enum UserAction {
    Move(HexDir),
    Discover,
    Capture(HexDir),
    Attack(HexCoord, usize)
}
pub trait UserInterface {
    fn render(&mut self, target: usize, fisherman: Fisherman, map: HashMap<HexCoord, HexCell>);
    fn input(&mut self) -> UserAction;
    fn invalid_input(&mut self);
}

pub struct CLI {
    target: usize,
    map: HashMap<HexCoord, HexCell>
}

impl CLI {
    pub fn new() -> Self {
        CLI {
            target: 0,
            map: HashMap::new(),
        }
    }
    fn render_map(&mut self, map: HashMap<HexCoord, HexCell>, fisherman: &Fisherman) {
        // first line
        const N_US: usize = 6;
        const N_SLASH: usize = 2;
        let order: usize = Fisherman::VISUAL_RADIUS as usize + 1;

        // let order = 0;
        // beginning
        for _ in 0..(order * (N_US + N_SLASH) + N_SLASH) { print!(" "); }
        for _ in 0..N_US { print!("_"); }
        println!();
        for i in 0..order {
            for j in 0..(N_SLASH - 1) {
                
                for _ in 0..((order - i) * (N_US + N_SLASH) + N_SLASH - j - 1) { print!(" "); }
                for k in 0..=i {
                    let q0 = -(i as i32) + 2 * (k as i32);
                    let q1 = -(i as i32) + 2 * (k as i32) + 1;
                    let r = (order as i32) - (k as i32);
                    let s0: i32 = (i as i32) - (order as i32) - (k as i32);
                    let s1 = (i as i32) - (order as i32) - (k as i32) - 1;
                    let coord0 = fisherman.get_coord() + unsafe { HexDir::new_unchecked(q0, r, s0) };
                    let coord1 = fisherman.get_coord() + unsafe { HexDir::new_unchecked(q1, r, s1) };
                    let cell0 = map.get(&coord0);
                    let cell1 = map.get(&coord1);
                    print!("/");
                    // 0th line: marlins
                    if let Some(c) = cell0 {
                        let discovered_marlins = c.marlins.iter().filter(|p| p.is_discovered()).collect::<Vec<_>>();
                        if j == 0 && !discovered_marlins.is_empty() {
                            print!(" M{:>3} ", discovered_marlins.len());
                        } else {
                            // print!("{:>2}{:>2}{:>2}", coord0.get_q(), coord0.get_r(), coord0.get_s());
                            print!("      ")
                        }
                    } else {
                        // print!("{:>2}{:>2}{:>2}", coord0.get_q(), coord0.get_r(), coord0.get_s());
                        print!("      ")
                    }
                    for _ in 0..(N_US - 6 + 2 * j) { print!(" "); }
                    print!("\\");
                    // 2nd line: shark
                    if k != i {
                        if let Some(c) = cell1 {
                            if j == 0 && !c.sharks.is_empty() {
                                print!("  S{:>3}  ", c.sharks.len());
                            } else {
                                // print!(" {:>2}{:>2}{:>2} ", coord1.get_q(), coord1.get_r(), coord1.get_s());
                                print!("        ");
                            }
                        } else {
                            // print!(" {:>2}{:>2}{:>2} ", coord1.get_q(), coord1.get_r(), coord1.get_s());
                            print!("        ")
                        }
                        for _ in 0..(N_US + 2 * (N_SLASH - j - 1) - 8) { print!(" "); }
                    }
                }
                println!();
            }

            for _ in 0..((order - i - 1) * (N_US + N_SLASH) + N_SLASH) { print!(" "); }
            for _ in 0..N_US { print!("_"); }
            for k in 0..=i {
                let q = -(i as i32) + 2 * (k as i32);
                let r = (order as i32) - (k as i32);
                let s = (i as i32) - (order as i32) - (k as i32);
                let coord = fisherman.get_coord() + unsafe { HexDir::new_unchecked(q, r, s) };
                print!("/");

                if coord == fisherman.get_coord() && coord == Fisherman::HARBOR_COORD {
                    print!("|Player|");
                } else if coord  == Fisherman::HARBOR_COORD {
                    print!("[||||||]");
                } else if coord == fisherman.get_coord() {
                    print!(" Player ");
                } else {
                    // print!(" {:>2}{:>2}{:>2} ", coord.get_q(), coord.get_r(), coord.get_s());
                    print!("        ")
                }
                for _ in 0..(N_US + 2 * (N_SLASH - 1) - 8) { print!(" "); }
                print!("\\");
                for _ in 0..N_US { print!("_"); }
            }
            println!();
        }
        // intermediate section
        for i in 0..=order {
            for j in 0..(N_SLASH - 1) {
                for _ in 0..(N_SLASH - j - 1) { print!(" "); }
                for k in 0..=order {

                    let q0 = -(order as i32) + 2 * (k as i32);
                    let q1 = -(order as i32) + 2 * (k as i32) + 1;
                    let r = (order as i32) - (i as i32) - (k as i32);
                    let s0: i32 = - (k as i32) + (i as i32);
                    let s1 = - (k as i32) + (i as i32) - 1;
                    let coord0 = fisherman.get_coord() + unsafe { HexDir::new_unchecked(q0, r, s0) };
                    let coord1 = fisherman.get_coord() + unsafe { HexDir::new_unchecked(q1, r, s1) };
                    let cell0 = map.get(&coord0);
                    let cell1 = map.get(&coord1);
                    print!("/");

                    // 0th line: marlins
                    if let Some(c) = cell0 {
                        let discovered_marlins = c.marlins.iter().filter(|p| p.is_discovered()).collect::<Vec<_>>();
                        if j == 0 && !discovered_marlins.is_empty() {
                            print!(" M{:>3} ", discovered_marlins.len());
                        } else {
                            // print!("{:>2}{:>2}{:>2}", coord0.get_q(), coord0.get_r(), coord0.get_s());
                            print!("      ")
                        }
                    } else {
                        // print!("{:>2}{:>2}{:>2}", coord0.get_q(), coord0.get_r(), coord0.get_s());
                        print!("      ")
                    }
                    for _ in 0..(N_US - 6 + 2 * j) { print!(" "); }
                    print!("\\");

                    // 2nd line: shark
                    if k != order {
                        if let Some(c) = cell1 {
                            if j == 0 && !c.sharks.is_empty() {
                                print!("  S{:>3}  ", c.sharks.len());
                            } else {
                                // print!(" {:>2}{:>2}{:>2} ", coord1.get_q(), coord1.get_r(), coord1.get_s());
                                print!("        ");
                            }
                        } else {
                            // print!(" {:>2}{:>2}{:>2} ", coord1.get_q(), coord1.get_r(), coord1.get_s());
                            print!("        ")
                        }
                        for _ in 0..(N_US + 2 * (N_SLASH - j - 1) - 8) { print!(" "); }
                    }
                }
                println!();
            }
            for k in 0..=order {

                let q = -(order as i32) + 2 * (k as i32);
                let r = (order as i32) - (i as i32) - (k as i32);
                let s: i32 = - (k as i32) + (i as i32);
                let coord = fisherman.get_coord() + unsafe { HexDir::new_unchecked(q, r, s) };
                print!("/");
                if coord == fisherman.get_coord() && coord == Fisherman::HARBOR_COORD {
                    print!("|Player|");
                } else if coord  == Fisherman::HARBOR_COORD {
                    print!("[||||||]");
                } else if coord == fisherman.get_coord() {
                    print!(" Player ");
                } else {
                    // print!(" {:>2}{:>2}{:>2} ", coord.get_q(), coord.get_r(), coord.get_s());
                    print!("        ")
                }
                for _ in 0..(N_US + 2 * N_SLASH - 2 - 8) { print!(" "); }
                print!("\\");
                if k != order {
                    for _ in 0..N_US { print!("_"); }
                }
            }
            println!();

            for j in 0..(N_SLASH - 1) {
                for _ in 0..j { print!(" "); }
                for k in 0..=order {
                    let q0 = -(order as i32) + 2 * (k as i32);
                    let q1 = -(order as i32) + 2 * (k as i32) + 1;
                    let r0 = (order as i32) - (i as i32) - (k as i32);
                    let r1 = (order as i32) - (i as i32) - (k as i32) - 1;
                    let s: i32 = -(k as i32) + (i as i32);
                    let coord0 = fisherman.get_coord() + unsafe { HexDir::new_unchecked(q0, r0, s) };
                    let coord1 = fisherman.get_coord() + unsafe { HexDir::new_unchecked(q1, r1, s) };
                    let cell0 = map.get(&coord0);
                    let cell1 = map.get(&coord1);

                    print!("\\");
                    // 2nd line: shark
                    if let Some(c) = cell0 {
                        if j == 0 && !c.sharks.is_empty() {
                            print!("  S{:>3}  ", c.sharks.len());
                        } else {
                            // print!(" {:>2}{:>2}{:>2} ", coord0.get_q(), coord0.get_r(), coord0.get_s());
                            print!("        ");
                        }
                    } else {
                        // print!(" {:>2}{:>2}{:>2} ", coord0.get_q(), coord0.get_r(), coord0.get_s());
                        print!("        ")
                    }
                    for _ in 0..(N_US + 2 * (N_SLASH - j - 1) - 8) { print!(" "); }
                    
                    print!("/");
                    if k != order {

                        // 0th line: marlins
                        if let Some(c) = cell1 {

                            let discovered_marlins = c.marlins.iter().filter(|p| p.is_discovered()).collect::<Vec<_>>();
                            if j == 0 && !discovered_marlins.is_empty() {
                                print!(" M{:>3} ", discovered_marlins.len());
                            } else {
                                // print!("{:>2}{:>2}{:>2}", coord1.get_q(), coord1.get_r(), coord1.get_s());
                                print!("      ")
                            }
                        } else {
                            // print!("{:>2}{:>2}{:>2}", coord1.get_q(), coord1.get_r(), coord1.get_s());
                            print!("      ")
                        }
                        for _ in 0..(N_US - 6 + 2 * j) { print!(" "); }
                    }
                }
                for _ in 0..N_SLASH { print!(" "); }
                println!();
            }
            for _ in 0..(N_SLASH - 1) { print!(" "); }

            for k in 0..=order {

                let q = -(order as i32) + 2 * (k as i32) + 1;
                let r = (order as i32) - (i as i32) - (k as i32) - 1;
                let s: i32 = - (k as i32) + (i as i32);
                let coord = fisherman.get_coord() + unsafe { HexDir::new_unchecked(q, r, s) };
                print!("\\");
                for _ in 0..N_US { print!("_"); }
                print!("/");
                if k != order {
                    if coord == fisherman.get_coord() && coord == Fisherman::HARBOR_COORD {
                        print!("|Player|");
                    } else if coord  == Fisherman::HARBOR_COORD {
                        print!("[||||||]");
                    } else if coord == fisherman.get_coord() {
                        print!(" Player ");
                    } else {
                        // print!(" {:>2}{:>2}{:>2} ", coord.get_q(), coord.get_r(), coord.get_s());
                        print!("        ")
                    }
                    for _ in 0..(N_US + 2 * (N_SLASH - 1) - 8) { print!(" "); }
                }
            }
            println!();
        }
        // ending
        for i in 0..order {
            for j in 0..(N_SLASH - 1) {
                for _ in 0..((i + 1) * (N_US + N_SLASH) + j) { print!(" "); }
                for k in 0..(order - i) {

                    let q0 = -(order as i32) + 2 * (k as i32 + 1) + (i as i32) - 1;
                    let q1 = -(order as i32) + 2 * (k as i32 + 1) + (i as i32);
                    let r0 = -(i as i32) - (k as i32 + 1);
                    let r1 = -(i as i32) - (k as i32 + 1) - 1;
                    let s: i32 = (order as i32) - (k as i32 + 1) + 1;
                    let coord0 = fisherman.get_coord() + unsafe { HexDir::new_unchecked(q0, r0, s) };
                    let coord1 = fisherman.get_coord() + unsafe { HexDir::new_unchecked(q1, r1, s) };
                    let cell0 = map.get(&coord0);
                    let cell1 = map.get(&coord1);
                    print!("\\");

                    // 2nd line: shark
                    if let Some(c) = cell0 {
                        if j == 0 && !c.sharks.is_empty() {
                            print!("  S{:>3}  ", c.sharks.len());
                        } else {
                            // print!(" {:>2}{:>2}{:>2} ", coord0.get_q(), coord0.get_r(), coord0.get_s());
                            print!("        ");
                        }
                    } else {
                        // print!(" {:>2}{:>2}{:>2} ", coord0.get_q(), coord0.get_r(), coord0.get_s());
                        print!("        ")
                    }
                    for _ in 0..(N_US + 2 * (N_SLASH - j - 1) - 8) { print!(" "); }
                    print!("/");
                    
                    if k != order - i - 1 {
                        // 0th line: marlins
                        if let Some(c) = cell1 {

                            let discovered_marlins = c.marlins.iter().filter(|p| p.is_discovered()).collect::<Vec<_>>();
                            if j == 0 && !discovered_marlins.is_empty() {
                                print!(" M{:>3} ", discovered_marlins.len());
                            } else {
                                // print!("{:>2}{:>2}{:>2}", coord1.get_q(), coord1.get_r(), coord1.get_s());
                                print!("      ")
                            }
                        } else {
                            // print!("{:>2}{:>2}{:>2}", coord1.get_q(), coord1.get_r(), coord1.get_s());
                            print!("      ")
                        }
                        for _ in 0..(N_US - 6 + 2 * j) { print!(" "); }
                    }
                }
                for _ in 0..((order - i) * (N_US + N_SLASH) + N_SLASH) { print!(" "); }
                println!();
            }

            for _ in 0..((i + 1) * (N_US + N_SLASH) + N_SLASH - 1) { print!(" "); }
            for k in 0..(order - i) {

                let q = -(order as i32) + 2 * (k as i32 + 1) + (i as i32);
                let r = -(i as i32) - (k as i32 + 1) - 1;
                let s: i32 = (order as i32) - (k as i32 + 1) + 1;
                let coord = fisherman.get_coord() + unsafe { HexDir::new_unchecked(q, r, s) };
                print!("\\");
                for _ in 0..N_US { print!("_"); }
                print!("/");

                if k != order - i - 1 {
                    if coord == fisherman.get_coord() && coord == Fisherman::HARBOR_COORD {
                        print!("|Player|");
                    } else if coord  == Fisherman::HARBOR_COORD {
                        print!("[||||||]");
                    } else if coord == fisherman.get_coord() {
                        print!(" Player ");
                    } else {
                        // print!(" {:>2}{:>2}{:>2} ", coord.get_q(), coord.get_r(), coord.get_s());
                        print!("        ")
                    }
                    for _ in 0..(N_US + 2 * (N_SLASH - 1) - 8) { print!(" "); }
                }
            }
            println!();
        }
    }
}

impl UserInterface for CLI {
    fn render(&mut self, target: usize, fisherman: Fisherman, map: HashMap<HexCoord, HexCell>) {
        println!("target: {target}, fisherman: {fisherman:?}");
        self.target = target;
        self.map = map.clone();
        self.render_map(map, &fisherman);
    }
    
    fn input(&mut self) -> UserAction {
        let mut stdin = stdin().lock();
        loop {

            let line = stdin.read_line().unwrap().unwrap();
            let line = line.trim();
            match line {
                "s" => return UserAction::Move(HexDir::ZERO),
                "w" => return UserAction::Move(HexDir::NORTH),
                "x" => return UserAction::Move(HexDir::SOUTH),
                "q" => return UserAction::Move(HexDir::NORTHWEST),
                "z" => return UserAction::Move(HexDir::SOUTHWEST),
                "e" => return UserAction::Move(HexDir::NORTHEAST),
                "c" => return UserAction::Move(HexDir::SOUTHEAST),

                "" => return UserAction::Discover,

                "S" => return UserAction::Capture(HexDir::ZERO),
                "W" => return UserAction::Capture(HexDir::NORTH),
                "X" => return UserAction::Capture(HexDir::SOUTH),
                "Q" => return UserAction::Capture(HexDir::NORTHWEST),
                "Z" => return UserAction::Capture(HexDir::SOUTHWEST),
                "E" => return UserAction::Capture(HexDir::NORTHEAST),
                "C" => return UserAction::Capture(HexDir::SOUTHEAST),
                _ => println!("Invalid action.")
            }
        }
    }
    
    fn invalid_input(&mut self) {
        println!("Invalid input!");
    }
}