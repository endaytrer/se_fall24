

#[tokio::main(flavor = "current_thread")]
async fn main() {
    #[cfg(feature = "cli")]
    {
        use fisherman::interface::cli::CLI;
        use fisherman::level::Game;
        Game::<CLI>::new().start().await;
    }
}
