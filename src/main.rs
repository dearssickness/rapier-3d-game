use bevy::{prelude::*, render::camera};
use bevy_rapier3d::prelude::*;
use bevy::{pbr::NotShadowCaster, prelude::*, utils::FloatOrd};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, print_ball_altitude)
        .add_systems(Update, camera_controls)
        .add_systems(Update, move_cube)
        .add_systems(Update, shoot)
        .add_systems(Update, change_cube_direction)
        .add_systems(Update, change_camera)
        .run();
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Camera;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Light;

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    })
    .insert(Camera)
    .insert(Name::new("Camera"));
}

fn change_camera(
//    keyboard: Res<Input<KeyCode>>,
    player_transforms: Query<&Transform, With<Player>>,
    mut camera_transforms: Query<&mut Transform, (Without<Ground>, Without<Player>, Without<Light>, Without<Bullet>)>
    
){
    let player_transform = player_transforms.single();
    let mut camera_transform = camera_transforms.single_mut();

    camera_transform.rotation =  player_transform.rotation;
    camera_transform.translation = player_transform.translation;
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    /* Create the ground. */
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {size: 100.0, subdivisions: 0})),
            material: materials.add(Color::hex("006B6E").unwrap().into()),
            ..default()
        })
        .insert(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(Ground)
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    // Create light    
    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 15000.0,
                range: 150.0,
                shadows_enabled: false,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 15.0, 4.0),
            ..default()
        })
        .insert(Light)
        .insert(Name::new("Light"));

    /* Create the bouncing ball. */
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Circle {radius: 0.5, vertices: 100})),
            material: materials.add(Color::rgb(1.3, 0.5, 1.3).into()),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)))
        .insert(ExternalForce {
            force: Vec3::new(0.0, 0.0, 0.0),
            torque: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(Bullet)
        .insert(Name::new("Bullet"));

    /* Meant to be the player */
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {min_x: -0.5, max_x:0.5, min_y:-0.5, max_y: 0.5, min_z:-0.5, max_z:0.5})),
            material: materials.add(Color::rgb(1.3, 0.5, 1.3).into()),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid (0.5, 0.5, 0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(Friction {
            coefficient: 0.1,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(ExternalForce {
            force: Vec3::new(0.0, 0.0, 0.0),
            torque: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(TransformBundle::from(Transform::from_xyz(-3.0, 4.0, 0.0)))
        .insert(Player)

        .insert(Velocity {
            linvel: Vec3::new(0.0, 0.0, 0.0),
            angvel: Vec3::new(0.0, 0.0, 0.0),
        })

        .insert(Name::new("Player"));

}


pub fn shoot(
    keyboard: Res<Input<KeyCode>>,
    positions: Query<&Transform, With<Player>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
){
    for position in positions.iter() {
        if keyboard.just_pressed(KeyCode::Space) {
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Circle {radius: 0.5, vertices: 100})),
                    material: materials.add(Color::hex("ec1c24").unwrap().into()),
                    ..default()
                })
                .insert(RigidBody::Dynamic)
                .insert(Collider::ball(0.5))
                .insert(Restitution::coefficient(0.7))
                .insert(TransformBundle::from(Transform::from_xyz(
                    position.translation.x + 1.0, position.translation.y + 1.0, position.translation.z)))
                .insert(ExternalForce {
                    force: Vec3::new(10.0, 0.0, 0.0),
                    torque: Vec3::new(0.0, 0.0, 0.0),
                })
                .insert(GravityScale(0.0))
                .insert(Name::new("Bullet"));
    }
    }
}

pub fn change_cube_direction(
    keyboard: Res<Input<KeyCode>>,
    mut velocities: Query<&mut Velocity, With<Player>>    
){
    
    if keyboard.pressed(KeyCode::R) {
        for mut vel in velocities.iter_mut() {
            vel.linvel = Vec3::new(0.0, 0.0, 0.0);
            vel.angvel = Vec3::new(0.0, 1.0, 0.0);
        }
    }
}

pub fn move_cube(
    keyboard: Res<Input<KeyCode>>,
    mut ext_forces: Query<&mut ExternalForce, With<Player>>,
){
    let mut ext_force = ext_forces.single_mut();
    if keyboard.pressed(KeyCode::Right) {
        ext_force.force = Vec3::new(2.0, 0.0, 0.0);
    }

    if keyboard.just_released(KeyCode::Right) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = Vec3::new(0.0, 0.0, 0.0);
        }
    }

    if keyboard.pressed(KeyCode::Left) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = Vec3::new(-2.0, 0.0, 0.0);
        }
    }

    if keyboard.just_released(KeyCode::Left) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = Vec3::new(0.0, 0.0, 0.0);
        }
    }
    
    if keyboard.pressed(KeyCode::Up) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = Vec3::new(0.0, 0.0, -2.0);
        }
    }

    else if keyboard.just_released(KeyCode::Up) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = Vec3::new(0.0, 0.0, 0.0);
        }
    }

     if keyboard.pressed(KeyCode::Down) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = Vec3::new(0.0, 0.0, 2.0);
        }
    }

    if keyboard.just_released(KeyCode::Down) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = Vec3::new(0.0, 0.0, 0.0);
        }
    }   

     if keyboard.pressed(KeyCode::Up) & keyboard.pressed(KeyCode::Right) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = Vec3::new(2.0, 0.0, -2.0);
        }
    }

    if keyboard.just_released(KeyCode::Up) & keyboard.pressed(KeyCode::Right) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = Vec3::new(0.0, 0.0, 0.0);
        }
    }

     if keyboard.pressed(KeyCode::Up) & keyboard.pressed(KeyCode::Left) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = Vec3::new(-2.0, 0.0, -2.0);
        }
    }

    if keyboard.just_released(KeyCode::Up) & keyboard.pressed(KeyCode::Left) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = Vec3::new(0.0, 0.0, 0.0);
        }
    }

    if keyboard.pressed(KeyCode::Down) & keyboard.pressed(KeyCode::Left) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = Vec3::new(-2.0, 0.0, 2.0);
        }
    }

    if keyboard.just_released(KeyCode::Down) & keyboard.pressed(KeyCode::Left) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = Vec3::new(0.0, 0.0, 0.0);
        }
    }
    
   if keyboard.pressed(KeyCode::Down) & keyboard.pressed(KeyCode::Right) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = Vec3::new(2.0, 0.0, 2.0);
        }
    }

    if keyboard.just_released(KeyCode::Down) & keyboard.pressed(KeyCode::Right) {
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = Vec3::new(0.0, 0.0, 0.0);
        }
    } 

}

fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();

    let mut forward = camera.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut left = camera.left();
    left.y = 0.0;
    left = left.normalize();

    let speed = 10.0;
    let rotate_speed = 1.0;

    if keyboard.pressed(KeyCode::W) {
        camera.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::S) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::A) {
        camera.translation += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::D) {
        camera.translation -= left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds())
    }
    if keyboard.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds())
    }
}

fn print_ball_altitude(rotations: Query<&Transform, With<Player>>) {
    for rotation in rotations.iter() {
        println!("Ball altitude: {}", rotation.rotation);
    }
}