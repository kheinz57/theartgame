use bevy::prelude::{*, shape::Plane};
use bevy_flycam::prelude::*;

mod paintings;
use paintings::PaintingPlugin;
mod world;
use world::WorldPlugin;

fn main() {
  App::new()
    .add_plugins((DefaultPlugins,WorldPlugin,PaintingPlugin))
    .add_plugins(PlayerPlugin)
    .run();
}

