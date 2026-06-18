pub mod app;
pub mod event;
pub mod server;
pub mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> /* -> color_eyre::Result<()> */
{
    server::server::echo_server();

    /* color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result */
}
