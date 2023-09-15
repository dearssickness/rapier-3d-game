use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::render::mesh::shape::*;

#[derive(Component)]
pub struct Bullet;

use crate::Player;

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