use interface::CLI;
use level::Level;

mod map;
mod entities;
mod level;
mod interface;


fn main() {
    let level0 = Level::new(10, 5, 1, 0.5, Box::new(|_| 0.3), Box::new(|_| 0.0));
    let level1 = Level::new(20, 5, 1, 0.5, Box::new(|_| 0.3), Box::new(|_| 0.05));
    let mut levels = vec![level0, level1];
    let mut cli = CLI::new();
    for l in &mut levels {
        match l.start(&mut cli) {
            Ok(score) => println!("You win! score: {score}"),
            Err(score) => {
                println!("Game over! score: {score}");
                return;
            },
        }
    }
}
