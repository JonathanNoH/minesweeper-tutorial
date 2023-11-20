use bevy::prelude::*;
use board_plugin::BoardPlugin;

#[cfg(feature="debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();
    // Set up window
    app
        .add_plugins((DefaultPlugins, BoardPlugin));
    #[cfg(feature="debug")]
    app.add_plugins(WorldInspectorPlugin::new());
    app.add_systems(Startup, camera_setup);
    app.run();
}

fn camera_setup(mut commands: Commands) {
    // 2d
    commands.spawn(Camera2dBundle::new_with_far(0.0));
}