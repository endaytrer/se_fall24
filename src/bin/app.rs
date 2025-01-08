
fn main() {
    #[cfg(feature = "cli")]
    {
        use fisherman::level::InputResult;
        use fisherman::cli::CLI;
        use fisherman::level::Game;
        let mut game = Game::new();
        let mut cli = CLI::new();
        loop {
            cli.render(&game);
            let input_res = loop {
                let res = game.handle_action(cli.input().into());
                if let InputResult::InvalidInput = res {
                    CLI.invalid_input();
                } else {
                    break res
                }
            };
            match input_res {
                fisherman::level::InputResult::InvalidInput => unreachable!(),
                fisherman::level::InputResult::Ok => {},
                fisherman::level::InputResult::LevelPassed => {
                    cli.prompt(format!("Level passed! score: {}", game.get_score()));
                },
                fisherman::level::InputResult::LevelFailed => {
                    cli.prompt(format!("Level failed! score: {}", game.get_score()));
                    return;
                },
                fisherman::level::InputResult::GamePassed => {
                    cli.prompt(format!("Congrats! you win all levels! score: {}", game.get_score()));
                    return;
                },
            }
        }
    }
}
