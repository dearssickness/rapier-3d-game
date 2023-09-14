use bevy::prelude::*;
use bevy::render::mesh::shape::*;
use bevy_rapier3d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_third_person_camera::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(ThirdPersonCameraPlugin)
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, player_movement)
        .add_systems(Update, shoot)
        .run();
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Camera;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Light;

#[derive(Component)]
struct Speed(f32);


fn setup_graphics(mut commands: Commands) {
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
            mesh: meshes.add(Mesh::from(shape::Capsule {
            radius: 0.5, rings: 0, depth: 0.0, latitudes: 16, longitudes:32, uv_profile: CapsuleUvProfile::Aspect })),
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
        .insert(Ball)
        .insert(Name::new("Ball"));

    /* Meant to be the player */
    commands
        .spawn((PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
            radius: 0.5, rings: 0, depth: 1.0, latitudes: 16, longitudes:32, uv_profile: CapsuleUvProfile::Aspect })),
            material: materials.add(Color::rgb(1.3, 0.5, 1.3).into()),
            ..default()
        },
        ThirdPersonCameraTarget))
        .insert(RigidBody::Dynamic)
        .insert(Collider::capsule_y (0.5, 0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(Friction {
            coefficient: 0.1,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(ExternalForce {
            force: Vec3::new(0.0, 0.0, 0.0),
            torque: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(TransformBundle::from(Transform::from_xyz(-3.0, 1.0, 0.0)))
        .insert(Player)
        .insert(Speed(2.5))
        .insert(ExternalImpulse {
            impulse: Vec3::new(0.0, 0.0, 0.0),
            torque_impulse: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(Velocity {
            linvel: Vec3::new(0.0, 0.0, 0.0),
            angvel: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(Name::new("Player"));

}

pub fn shoot(
    positions: Query<&Transform, With<Player>>,
    cam_q: Query<&Transform, (With<Camera>, Without<Player>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mouse: Res<Input<MouseButton>>
){
    
    let cam = cam_q.single();

    let position = positions.single();

    if mouse.just_pressed(MouseButton::Left) {
        let bullet_position = cam.rotation.mul_vec3(Vec3::new(
            0.0,
            0.0,
            -2.0
        ));

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.5, rings: 0, depth: 0.0, latitudes: 16, longitudes:32, uv_profile: CapsuleUvProfile::Aspect })),
                material: materials.add(Color::hex("ec1c24").unwrap().into()),
                ..default()
            })
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(0.5))
            .insert(Restitution::coefficient(0.7))
//            .insert(TransformBundle::from(Transform::from_xyz(position.translation.x,
//            position.translation.y + 2.0, position.translation.z)
            .insert(TransformBundle::from(Transform::from_xyz(
                position.translation.x + bullet_position.x,
                position.translation.y + bullet_position.y,
                position.translation.z + bullet_position.z
                )))
            .insert(Velocity {
                linvel: cam.rotation.mul_vec3(Vec3::new(
                    1.0,
                    1.0,
                    -20.0
                )),
                angvel: Vec3::new(0.0, 0.0, 0.0),
            })
            .insert(GravityScale(0.0))
            .insert(Bullet)
            .insert(Name::new("Bullet"));
    }
}

pub fn camera_direction(
//    time: Res<Time>,
    cam_q: &Query<&Transform, (With<Camera>, Without<Player>)>,
    keys: Res<Input<KeyCode>>,
) -> Vec3 {

    let cam = cam_q.single();
    
    let mut direction = Vec3::ZERO;
    
    // forward
    if keys.pressed(KeyCode::W) {
        direction += cam.forward();
    }
    
    // back
    if keys.pressed(KeyCode::S) {
        direction += cam.back();
    }
    
    // left
    if keys.pressed(KeyCode::A) {
        direction += cam.left();
    }
    
    // right
    if keys.pressed(KeyCode::D) {
        direction += cam.right();
    }
    
    direction.y = 0.0;
    
    direction
}

    fn player_movement(
        keys: Res<Input<KeyCode>>,
        mouse: Res<Input<MouseButton>>,
        mut velocities: Query<(&mut Transform, &mut Velocity), With<Player>>,
        cam_q: Query<&Transform, (With<Camera>, Without<Player>)>,
    ) {

    let direction = camera_direction(&cam_q, keys);
    
    let cam = cam_q.single();

    let movement = direction;//.normalize_or_zero();

    let (mut player_transform, mut vel) = velocities.single_mut();

    if mouse.just_pressed(MouseButton::Right) && player_transform.translation.y <= -0.9{
        vel.linvel.y = 3.5;
    }
    
//    println!("player translation y is {}", player_transform.translation.y);

    if player_transform.translation.y <= -0.9 {
        vel.linvel.x = movement.x * 2.0;
        vel.linvel.z = movement.z * 2.0;

    } else {
        vel.linvel.x = movement.x * 6.0;
        vel.linvel.z = movement.z * 6.0;

    }

    player_transform.rotation = Quat::from_xyzw(0.0, cam.rotation.y, 0.0, cam.rotation.w);
    
}

//fn setup_graphics(mut commands: Commands) {
//    // Add a camera so we can see the debug-render.
//    commands.spawn(Camera3dBundle {
//        transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
//        ..Default::default()
//    })
//    .insert(Camera)
//    .insert(Name::new("Camera"));
//}

//fn change_camera(
////    keyboard: Res<Input<KeyCode>>,
//    player_transforms: Query<&Transform, With<Player>>,
//    mut camera_transforms: Query<&mut Transform, (Without<Ground>, Without<Player>, Without<Light>, Without<Bullet>)>
//    
//){
//    let player_transform = player_transforms.single();
//    let mut camera_transform = camera_transforms.single_mut();
//
////    *camera_transform = camera_transform.looking_at(Vec3::new(player_transform.translation.x + 2.0,
////        player_transform.translation.y, player_transform.translation.z), Vec3::ZERO);
//
////    camera_transform.translation = Vec3::new(player_transform.translation.x - 3.0,
////        player_transform.translation.y + 3.0 , player_transform.translation.z);
//
//    *camera_transform = Transform::from_xyz(player_transform.translation.x - 3.0,
//        player_transform.translation.y + 3.0 , player_transform.translation.z)
//    .looking_at(Vec3::new(player_transform.translation.x + 2.0,
//        player_transform.translation.y, player_transform.translation.z), Vec3::ZERO);
//    
//    println!("player transform rotation w is {}", player_transform.rotation.w);
//    
//
//}

//pub fn change_cube_direction(
//    keyboard: Res<Input<KeyCode>>,
//    mut velocities: Query<&mut Velocity, With<Player>>    
//){
//    
//    if keyboard.pressed(KeyCode::R) {
//        for mut vel in velocities.iter_mut() {
//            vel.linvel = Vec3::new(0.0, 0.0, 0.0);
//            vel.angvel = Vec3::new(0.0, 1.0, 0.0);
//        }
//    }
//
//    if keyboard.pressed(KeyCode::W) {
//        for mut vel in velocities.iter_mut() {
//            vel.linvel = Vec3::new(1.0, 0.0, 0.0);
//            vel.angvel = Vec3::new(0.0, 0.0, 0.0);
//        }
//    }
//}
//

//fn camera_controls(
//    keyboard: Res<Input<KeyCode>>,
//    mut camera_query: Query<&mut Transform, With<Camera3d>>,
//    time: Res<Time>,
//) {
//    let mut camera = camera_query.single_mut();
//
//    let mut forward = camera.forward();
//    forward.y = 0.0;
//    forward = forward.normalize();
//
//    let mut left = camera.left();
//    left.y = 0.0;
//    left = left.normalize();
//
//    let speed = 10.0;
//    let rotate_speed = 1.0;
//
//    if keyboard.pressed(KeyCode::W) {
//        camera.translation += forward * time.delta_seconds() * speed;
//    }
//    if keyboard.pressed(KeyCode::S) {
//        camera.translation -= forward * time.delta_seconds() * speed;
//    }
//    if keyboard.pressed(KeyCode::A) {
//        camera.translation += left * time.delta_seconds() * speed;
//    }
//    if keyboard.pressed(KeyCode::D) {
//        camera.translation -= left * time.delta_seconds() * speed;
//    }
//    if keyboard.pressed(KeyCode::Q) {
//        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds())
//    }
//    if keyboard.pressed(KeyCode::E) {
//        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds())
//    }
//}

//fn print_ball_altitude(rotations: Query<&Transform, With<Player>>) {
//    for rotation in rotations.iter() {
//        println!("Ball altitude: {}", rotation.rotation);
//    }
//}
