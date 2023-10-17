use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::render::mesh::shape::*;
use bevy_debug_grid::*;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Light;


#[derive(Component)]
pub struct BlockAge;

pub fn setup_map(
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
        
    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(1.5, 1.6, 1.5))
        .insert(PbrBundle{
            mesh: meshes.add(Mesh::from(shape::Box {min_x: -1.5, max_x:1.5, min_y:-1.6, max_y:1.6, min_z:-1.5, max_z:1.5} )),
            ..default()
        })
        .insert(BlockAge)
        .insert(TransformBundle::from(Transform::from_xyz(4.0, 1.6, 4.0)));
}
