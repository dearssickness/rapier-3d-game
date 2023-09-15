use bevy::prelude::*;
use bevy::render::mesh::shape::*;
//use bevy_inspector_egui::egui::Grid;
use bevy_rapier3d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_third_person_camera::*;
use bevy_debug_grid::*;

mod player;

pub use player::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(ThirdPersonCameraPlugin)
        .add_plugins(DebugGridPlugin::with_floor_grid(),)
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
//        .spawn(
//            PbrBundle {
//            mesh: meshes.add(Mesh::from(shape::Plane {size: 100.0, subdivisions: 0})),
//            material: materials.add(Color::hex("006B6E").unwrap().into()),
//            ..default()
//        })
        .spawn(
        Grid::default()
        )
        .insert(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(Ground)
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));

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
//    commands
//        .spawn((PbrBundle {
//            mesh: meshes.add(Mesh::from(shape::Capsule {
//            radius: 0.5, rings: 0, depth: 1.0, latitudes: 16, longitudes:32, uv_profile: CapsuleUvProfile::Aspect })),
//            material: materials.add(Color::rgb(1.3, 0.5, 1.3).into()),
//            ..default()
//        },
//        ThirdPersonCameraTarget))
//        .insert(RigidBody::Dynamic)
//        .insert(Collider::capsule_y (0.5, 0.5))
//        .insert(Restitution::coefficient(0.7))
//        .insert(Friction {
//            coefficient: 0.1,
//            combine_rule: CoefficientCombineRule::Min,
//        })
//        .insert(ExternalForce {
//            force: Vec3::new(0.0, 0.0, 0.0),
//            torque: Vec3::new(0.0, 0.0, 0.0),
//        })
//        .insert(TransformBundle::from(Transform::from_xyz(-3.0, 1.0, 0.0)))
//        .insert(Player)
//        .insert(Speed(2.5))
//        .insert(ExternalImpulse {
//            impulse: Vec3::new(0.0, 0.0, 0.0),
//            torque_impulse: Vec3::new(0.0, 0.0, 0.0),
//        })
//        .insert(Velocity {
//            linvel: Vec3::new(0.0, 0.0, 0.0),
//            angvel: Vec3::new(0.0, 0.0, 0.0),
//        })
//        .insert(Name::new("Player"));

}

pub fn shoot(
    mut player_transforms: Query<(&mut Transform, &mut Velocity), With<Player>>,
    cam_q: Query<&Transform, (With<Camera>, Without<Player>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mouse: Res<Input<MouseButton>>
){
    
    let cam = cam_q.single();

    let (player_transform, mut player_velocity) = player_transforms.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        let bullet_vector = cam.rotation.mul_vec3(
            Vec3::new(
                0.0,
                0.5,
                -1.0
        ));

        let player_backfire_vector = cam.rotation.mul_vec3(
            Vec3::new(
                0.0,
                0.0,
                10.0                
            ) 
        );
        
        if player_transform.translation.y <= 2.0 {
            player_velocity.linvel.x += player_backfire_vector.x;
            player_velocity.linvel.y += player_backfire_vector.y;
            player_velocity.linvel.z += player_backfire_vector.z;
        }
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
                player_transform.translation.x + bullet_vector.x,
                player_transform.translation.y + bullet_vector.y,
                player_transform.translation.z + bullet_vector.z
                )))
            .insert(Velocity {
                linvel: cam.rotation.mul_vec3(Vec3::new(
                    1.0,
                    1.0,
                    -40.0
                )),
                angvel: Vec3::new(0.0, 0.0, 0.0),
            })
            .insert(Dominance::group(10))
            .insert(ColliderMassProperties::Density(20.0))
            .insert(GravityScale(0.0))
            .insert(Bullet)
            .insert(Name::new("Bullet"));
    }
}