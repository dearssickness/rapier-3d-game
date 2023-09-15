use bevy::prelude::*;
use bevy_third_person_camera::*;

use crate::Camera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        ThirdPersonCamera {
           zoom: Zoom::new(5.0, 10.0),
        ..default()
        },
        Camera3dBundle::default()
    ))
    .insert(Camera)
    .insert(Name::new("Camera"));
}