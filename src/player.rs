use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::render::mesh::shape::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

//fn spawn_player(
//    
//    mut commands: Commands,
//    mut meshes: ResMut<Assets<Mesh>>,
//    mut materials: ResMut<Assets<StandardMaterial>>
//
//){
//    commands
//    .spawn((PbrBundle {
//        mesh: meshes.add(Mesh::from(shape::Capsule {
//        radius: 0.5, rings: 0, depth: 1.0, latitudes: 16, longitudes:32, uv_profile: CapsuleUvProfile::Aspect })),
//        material: materials.add(Color::rgb(1.3, 0.5, 1.3).into()),
//        ..default()
//    },
//    ThirdPersonCameraTarget))
//    .insert(RigidBody::Dynamic)
//    .insert(Collider::capsule_y (0.5, 0.5))
//    .insert(Restitution::coefficient(0.7))
//    .insert(Friction {
//        coefficient: 0.1,
//        combine_rule: CoefficientCombineRule::Min,
//    })
//    .insert(ExternalForce {
//        force: Vec3::new(0.0, 0.0, 0.0),
//        torque: Vec3::new(0.0, 0.0, 0.0),
//    })
//    .insert(TransformBundle::from(Transform::from_xyz(-3.0, 1.0, 0.0)))
//    .insert(Player)
//    .insert(Speed(2.5))
//    .insert(ExternalImpulse {
//        impulse: Vec3::new(0.0, 0.0, 0.0),
//        torque_impulse: Vec3::new(0.0, 0.0, 0.0),
//    })
//    .insert(Velocity {
//        linvel: Vec3::new(0.0, 0.0, 0.0),
//        angvel: Vec3::new(0.0, 0.0, 0.0),
//    })
//    .insert(Name::new("Player"));
//}

fn player_movement(
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut velocities: Query<(&mut Transform, &mut Velocity), With<Player>>,
    cam_q: Query<&Transform, (With<Camera>, Without<Player>)>,
) {

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

    let movement = direction;//.normalize_or_zero();

    let (mut player_transform, mut vel) = velocities.single_mut();

    if mouse.pressed(MouseButton::Right) && player_transform.translation.y <= 1.1{
        vel.linvel.y = 3.5;
    }
    
    println!("player translation y is {}", player_transform.translation.y);

    if player_transform.translation.y <= 1.1 {
        vel.linvel.x = movement.x * 4.0;
        vel.linvel.z = movement.z * 4.0;

    } else {
        vel.linvel.x = movement.x * 8.0;
        vel.linvel.z = movement.z * 8.0;

    }

    player_transform.rotation = Quat::from_xyzw(0.0, cam.rotation.y, 0.0, cam.rotation.w);
    
}