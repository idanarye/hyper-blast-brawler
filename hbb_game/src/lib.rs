extern crate amethyst;

mod game_data;
mod debugging;

pub use game_data::HyperBlastBrawlerGame;

// use amethyst::prelude::*;
use amethyst::core::bundle::SystemBundle;

pub struct HBBBundle;

impl SystemBundle<'_, '_> for HBBBundle {
    fn build(self, dispatcher: &mut amethyst::shred::DispatcherBuilder) -> amethyst::core::bundle::Result<()> {
        dispatcher.add(debugging::DebugInfoSystem, "debug_info_system", &[]);
        Ok(())
    }
}
