mod debug;
mod movement;
mod spaceship;
mod camera;
mod asteroid;
mod asset_loader;

use bevy::prelude::*;
use spaceship::SpaceshipPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use camera::CameraPlugin;
use asteroid::AsteroidPlugin;
use asset_loader::AssetLoaderPlugin;


fn main() {
    App::new()
    .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
    .add_plugins(DefaultPlugins)
    .add_plugins(SpaceshipPlugin)
    .add_plugins(MovementPlugin)
    .add_plugins(DebugPlugin)
    .add_plugins(AssetLoaderPlugin)
    .add_plugins(CameraPlugin)
    .add_plugins(AsteroidPlugin)
    .run();
}