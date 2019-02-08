extern crate flexi_logger;

use flexi_logger::Logger;

extern crate amethyst;

use amethyst::prelude::*;
use amethyst::renderer::{Pipeline, DrawFlat2D, Stage, RenderBundle, DisplayConfig};
use amethyst::core::transform::TransformBundle;
use amethyst::ui::UiBundle;
use amethyst::input::InputBundle;

extern crate hbb_game;
use hbb_game::HyperBlastBrawlerGame;

fn main() -> Result<(), String> {
    Logger::with_env_or_str("warn")
        .start().map_err(|e| format!("{}", e))?;

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
        .clear_target([0., 0., 0., 1.], 1.)
        .with_pass(DrawFlat2D::new())
    );

    let mut game = || -> Result<_, amethyst::Error> {
        let game_data = GameDataBuilder::default()
            // .with_basic_renderer("resources/display_config.ron", DrawFlat2D::new(), true)?
            .with_bundle(RenderBundle::new(pipe, Some(DisplayConfig::load("resources/display_config.ron")))
                         .with_sprite_sheet_processor())?
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
