#[macro_use]
extern crate log;
extern crate flexi_logger;

use flexi_logger::Logger;

extern crate amethyst;

use amethyst::prelude::*;
use amethyst::renderer::{DrawFlat, PosTex};
use amethyst::core::transform::TransformBundle;
use amethyst::ui::UiBundle;
use amethyst::input::InputBundle;

extern crate hbb_game;
use hbb_game::HyperBlastBrawlerGame;

fn main() -> Result<(), String> {
    Logger::with_env()
        .start().map_err(|e| format!("{}", e))?;

    let mut game = || -> Result<_, amethyst::Error> {
        let game_data = GameDataBuilder::default()
            .with_basic_renderer("resources/display_config.ron", DrawFlat::<PosTex>::new(), true)?
            .with_bundle(TransformBundle::new())?
            .with_bundle(UiBundle::<String, String>::new())?
            .with_bundle(InputBundle::<String, String>::new()
                         .with_bindings_from_file("resources/bindings_config.ron")?)?
        ;
        Application::new(".", HyperBlastBrawlerGame, game_data)
    }().map_err(|e| format!("{}", e))?;

    game.run();

    Ok(())
}
