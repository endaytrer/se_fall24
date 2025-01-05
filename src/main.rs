use interface::CLI;
use level::Level;

mod map;
mod entities;
mod level;
mod interface;


fn main() {
    let level0 = Level::new(5, 5, 1, 0.5, Box::new(|_| 0.3), Box::new(|_| 0.0));
    let level1 = Level::new(10, 5, 1, 0.5, Box::new(|_| 0.3), Box::new(|_| 0.05));
    let level2 = Level::new(15, 5, 1, 0.5, Box::new(|_| 0.25), Box::new(|_| 0.07));
    let mut levels = vec![level0, level1, level2];
    let mut cli = CLI::new();
    for (i, l) in &mut levels.iter_mut().enumerate() {
        match l.start(&mut cli) {
            Ok(score) => {
                println!("You win level {}! score: {score}", i + 1)

            },
            Err(score) => {
                println!("Game over! score: {score}");
                return;
            },
        }
    }

    println!("Congrats! You win all levels!");
}
