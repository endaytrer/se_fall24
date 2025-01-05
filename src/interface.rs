use termion::input::TermRead;

use crate::{entities::{Damageable, Fisherman}, map::{HexCell, HexCoord, HexDir}};

use core::f32;
use std::{collections::HashMap, io::{stdin, stdout, Read, Write}};
use termion::{color, style};
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
        println!("{}{}\n{}{}                         The Old Man and the Sea{}", termion::clear::All, termion::cursor::Goto(1, 1), color::Fg(color::Green), style::Bold, style::Reset);
        println!("{}                                       by Endaytrer{}\n", style::Italic, style::Reset);
        println!("============================== How to play ==============================");
        println!("{}Objective{}: Capture at least TARGET marlins (as much as possible) and", style::Bold, style::Reset);
        println!("return to harbor, avoid sharks to keep HP above 0.\n");
        println!("Only {}discovered{} marlins are shown on map, all sharks are shown. Sharks", style::Bold, style::Reset);
        println!("will chase you very closely!\n");
        println!("==============================  Key Binds  ==============================");
        println!("  W     ->      Move Up                   |  Shift + W ->     Capture Up");
        println!("  X     ->      Move Down                 |  Shift + X ->     Capture Down");
        println!("  Q     ->      Move Upleft               |  Shift + Q ->     Capture Upleft");
        println!("  Z     ->      Move Downleft             |  Shift + Z ->     Capture Downleft");
        println!("  E     ->      Move Upright              |  Shift + E ->     Capture Upright");
        println!("  C     ->      Move Downright            |  Shift + C ->     Capture Downright");
        println!("  S     ->      Stay In Place             |  Shift + S ->     Capture Current");
        println!("  [Key] + Enter -> Commit Action          |");
        println!("  Enter         -> Find Nearby Marlins    |");
        println!("=========================================================================");
        println!("\n                     {}PRESS ANY KEY TO CONTINUE{}", style::Bold, style::Reset);
        let mut stdin = stdin().lock();
        let mut byte = [0u8];
        stdin.read_exact(&mut byte).unwrap();
        CLI {
            target: 0,
            map: HashMap::new(),
        }
    }
    fn render_map(map: HashMap<HexCoord, HexCell>, fisherman: &Fisherman) {
        // first line
        const N_US: usize = 6;
        const N_SLASH: usize = 2;
        let order: usize = Fisherman::VISUAL_RADIUS as usize;

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
        println!();
    }

    fn render_compass(fisherman: &Fisherman) -> String {
        let dist = fisherman.get_coord().distance(&Fisherman::HARBOR_COORD);
        if dist == 0 {
            return "0NM from harbor".to_string()
        }
        let dir = Fisherman::HARBOR_COORD - fisherman.get_coord();
        let zero = dir.r;
        let sixty = dir.q;
        let x = (zero as f32) + (sixty as f32) / 2f32;
        let y = (sixty as f32) * 3f32.sqrt() / 2f32;
        let rad = f32::atan2(y, x); // (-PI, PI)
        let rad = if rad < 0.0 { f32::consts::TAU + rad } else { rad };
        let deg = rad / f32::consts::PI * 180f32;
        let (arrow, dir) = match deg {
            0.0..22.5 | 337.5..=360.0 => ("↑", "N"),
            22.5..67.5   => ("↗", "NE"),
            67.5..112.5  => ("→", "E"),
            112.5..157.5 => ("↘", "SE"),
            157.5..202.5 => ("↓", "S"),
            202.5..247.5 => ("↙", "SW"),
            247.5..292.5 => ("←", "W"),
            292.5..337.5 => ("↖", "NW"),
            _ => panic!("degree conversion failed")
        };
        format!("{} {}°{:2>} {}NM from harbor", arrow, deg as i32, dir, dist)
    }
}

impl UserInterface for CLI {
    fn render(&mut self, target: usize, fisherman: Fisherman, map: HashMap<HexCoord, HexCell>) {
        self.target = target;
        self.map = map.clone();
    
        let mut heart_format = String::new();
        for _ in 0..fisherman.get_hp() {
            heart_format += "♥ "
        }for _ in fisherman.get_hp()..fisherman.get_initial_hp() {
            heart_format += "♡ "
        }
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        Self::render_map(map, &fisherman);
        if fisherman.get_captured_marlins() >= target {
            print!("{}", color::Fg(color::Green));
        }
        print!("target: {}/{target}{}, {}, ", fisherman.get_captured_marlins(), style::Reset, Self::render_compass(&fisherman));
        if fisherman.get_hp() as f32 / fisherman.get_initial_hp() as f32 <= 0.25 {
            print!("{}", color::Fg(color::Red));
        }
        print!("HP: {}{} Operation: ", heart_format, style::Reset);
        stdout().flush().unwrap();
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