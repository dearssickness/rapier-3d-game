use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
//use bevy_inspector_egui::egui::Grid;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_third_person_camera::*;
use bevy_debug_grid::*;

mod player;
mod camera;
mod map;
mod shooting;

pub use player::*;
pub use camera::*;
pub use map::*;
pub use shooting::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(ThirdPersonCameraPlugin)
        .add_plugins(DebugGridPlugin::with_floor_grid())
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_map)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, player_movement)
        .add_systems(Update, shoot)
        .add_systems(Update, despawn_bullet)
        .run();
}

#[derive(Component)]
struct Speed(f32);