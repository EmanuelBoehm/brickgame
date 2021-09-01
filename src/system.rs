use bevy::{prelude::*, sprite::collide_aabb::{collide, Collision}};

use crate::{Collider, GameState, MousePos, Scoreboard, components::{MoveDirection, Speed}, entity::Ball};

pub fn ball_movement_system(
    time: Res<Time>,
    mut ball_query: Query<(&Ball, &mut Transform, &Speed, &MoveDirection)>,
) {
    // clamp the timestep to stop the ball from escaping when the game starts
    let delta_seconds = f32::min(0.2, time.delta_seconds());
    for (_ball, mut transform, speed, direction) in ball_query.iter_mut() {
        transform.translation +=
            Vec3::new(direction.0, direction.1, 0.).normalize() * speed.0 * delta_seconds;
    }
}

pub fn ball_collision_system(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut ball_query: Query<(&mut Ball, &Transform, &Sprite, &mut MoveDirection)>,
    mut collider_query: Query<(Entity, &mut Collider, &Transform, &Sprite)>,
) {
    for (mut _ball, ball_transform, sprite, mut direction) in ball_query.iter_mut() {
        let ball_size = sprite.size;

        // check collision with walls
        for (collider_entity, mut collider, transform, sprite) in collider_query.iter_mut() {
            let collision = collide(
                ball_transform.translation,
                ball_size,
                transform.translation,
                sprite.size,
            );
            if let Some(collision) = collision {
                // block health gets checked
                if let Collider::Block(a) = *collider {
                    if a >= 1 {
                        *collider = Collider::Block(a - 1);
                    } else {
                        scoreboard.score += 1;
                        commands.entity(collider_entity).despawn();
                    }
                }

                // reflect the ball when it collides
                let mut reflect_x = false;
                let mut reflect_y = false;

                // only reflect if the ball's velocity is going in the opposite direction of the
                // collision
                match collision {
                    Collision::Left => reflect_x = direction.0 > 0.0,
                    Collision::Right => reflect_x = direction.0 < 0.0,
                    Collision::Top => reflect_y = direction.1 < 0.0,
                    Collision::Bottom => reflect_y = direction.1 > 0.0,
                }

                // reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    direction.0 = -direction.0;
                }

                // reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    direction.1 = -direction.1;
                }
            }
        }
    }
}
pub fn mouse_listener_system(
    btns: Res<Input<MouseButton>>,
    windows: Res<Windows>,

    mut mouse_pos: ResMut<MousePos>,
    mut app_state: ResMut<State<GameState>>,

) {
    let window = windows.get_primary().unwrap();
    
    if btns.just_pressed(MouseButton::Right) {
       
        // For multi-window applications, you need to use a specific window ID here.
        if let Some(position) = window.cursor_position() {
            *mouse_pos = position;

        }
        app_state.set(GameState::Shooting);
        println!("App state is about to be changed with {}", *mouse_pos);
        
    }
}