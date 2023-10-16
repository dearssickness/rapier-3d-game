use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::render::mesh::shape::*;
use bevy_third_person_camera::*;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::Speed;

static COLLISION_FLAG: AtomicBool = AtomicBool::new(true);

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
   
   mut commands: Commands,
   mut meshes: ResMut<Assets<Mesh>>,
   mut materials: ResMut<Assets<StandardMaterial>>

){
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
       coefficient: 0.0,
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
   .insert(ActiveEvents::COLLISION_EVENTS)
   .insert(Name::new("Player"));
}

pub fn direction_sync(
    keys: &Res<Input<KeyCode>>,
    cam_q: &Query<&Transform, (With<Camera>, Without<Player>)>,
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

pub fn player_movement(
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    cam_q: Query<&Transform, (With<Camera>, Without<Player>)>,
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    mut collision_events: EventReader<CollisionEvent>,

){
    let cam = cam_q.single();
    
    let movement = direction_sync(
        &keys, &cam_q
    );
    
    let (mut player_transform, mut player_velocity) = player_query.single_mut();
    
//    println!("player translation y is {}", player_transform.translation.y);

    for collision in collision_events.iter(){
//        println!("event {:?}", collision);
        match collision {
            CollisionEvent::Stopped(_, _, _) => {
                set_collisions(false);
            },
            _ =>{
                set_collisions(true);
            }
        }    
    }
    
    let collision_flag = COLLISION_FLAG.load(Ordering::Relaxed);
    
    if collision_flag == true{
        if (keys.pressed(KeyCode::W) || keys.pressed(KeyCode::S))
        && (keys.pressed(KeyCode::A) || keys.pressed(KeyCode::D)){
            player_velocity.linvel.x = movement.x * 3.0;
            player_velocity.linvel.z = movement.z * 3.0;
        }else{
            player_velocity.linvel.x = movement.x * 4.0;
            player_velocity.linvel.z = movement.z * 4.0;
        }

        if mouse.pressed(MouseButton::Right){
            player_velocity.linvel.y = 3.5;
        }
    }else{
        if (keys.pressed(KeyCode::W) || keys.pressed(KeyCode::S))
        && (keys.pressed(KeyCode::A) || keys.pressed(KeyCode::D)){
            player_velocity.linvel.x = movement.x * 6.0;
            player_velocity.linvel.z = movement.z * 6.0;
        }else{
            player_velocity.linvel.x = movement.x * 8.0;
            player_velocity.linvel.z = movement.z * 8.0;
        }
    }
    
    player_transform.rotation = Quat::from_xyzw(0.0, cam.rotation.y, 0.0, cam.rotation.w);

}


/* Not a Bevy system */
pub fn set_collisions(
    collision_stat: bool
){
    if collision_stat == true{
        COLLISION_FLAG.store(true ,Ordering::Relaxed)
    }else{
        COLLISION_FLAG.store(false ,Ordering::Relaxed)
    }
}