use bevy::prelude::*;
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 0.5)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "new app".to_string(),
                ..Default::default()
            },
            add_primary_window: true,
            exit_on_all_closed: true,
            close_when_requested: true,
        }))
        .add_startup_system(spawn_camera)
        .run();
    println!("Hello, world!");
}
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0),
        ..Default::default()
    });
}
