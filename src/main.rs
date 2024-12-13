mod debug;
mod movement;
mod spaceship;
mod camera;

use bevy::prelude::*;
use spaceship::SpaceshipPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use camera::CameraPlugin;


fn main() {
    App::new()
    .insert_resource(ClearColor(Color::srgb(0., 0., 0.)))
    .insert_resource(AmbientLight{
        brightness: 0.75,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugins(SpaceshipPlugin)
    .add_plugins(MovementPlugin)
    .add_plugins(DebugPlugin)
    .add_plugins(CameraPlugin)
    .run();
}