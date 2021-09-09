
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::{Collider, GameState, MousePos, Scoreboard, Shooter, components::Movement, constants::CONFIG, entity::{Ball, Block}, resource::HasWon};

pub fn ball_movement_system(
    mut ball_query: Query<(&mut Transform, &Movement), With<Ball>>,
) {
    for (mut transform, movement) in ball_query.iter_mut() {
        transform.translation += 
            Vec3::new(movement.x(), movement.y(),0.) * CONFIG.ballspeed;
    }
}

pub fn ball_collision_system(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut ball_query: Query<(Entity, &Transform, &Sprite, &mut Movement), With<Ball>>,
    mut collider_query: Query<(
        Entity,
        &mut Collider,
        &Transform,
        &Sprite,
        Option<&Children>,
    )>,
    mut collider_text_query: Query<&mut Text>,
) {
    for (ball_entity, ball_transform, ball_sprite, mut movement) in ball_query.iter_mut() {
        // checking borders and flip if on wall or despawn on ground
        if ball_transform.translation.y < -CONFIG.window_height / 2.{
            commands.entity(ball_entity).despawn();
            continue;
        }
        if ball_transform.translation.y > CONFIG.window_height / 2. && movement.y() > 0. {
            movement.flip_y();
        }
        if ball_transform.translation.x < -CONFIG.window_width / 2. && movement.x() < 0. {
            movement.flip_x();
        }
        if ball_transform.translation.x > CONFIG.window_width / 2. && movement.x() > 0. {
            movement.flip_x();
        }

        for (collider_entity, mut collider, transform, sprite, children) in
            collider_query.iter_mut()
        {
            let collision = collide(
                ball_transform.translation,
                ball_sprite.size,
                transform.translation,
                sprite.size,
            );
            if let Some(collision) = collision {
                // block health gets checked
                if let Collider::Block(health) = *collider {
                    if health > 1 {
                        *collider = Collider::Block(health - 1);
                        if let Some(children) = children {
                            if let Ok(mut child) = collider_text_query.get_mut(children[0]) {
                                child.sections[0].value =
                                    (child.sections[0].value.parse::<u32>().unwrap() - 1)
                                        .to_string();
                            }
                        }
                    } else {
                        scoreboard.score += 1;
                        commands.entity(collider_entity).despawn_recursive();
                    }
                    // reflect the ball when it collides
                    let mut reflect_x = false;
                    let mut reflect_y = false;

                    // only reflect if the ball's velocity is going in the opposite direction of the
                    // collision
                    match collision {
                        Collision::Left => reflect_x = true,
                        Collision::Right => reflect_x = true,
                        Collision::Top => reflect_y = true ,
                        Collision::Bottom => reflect_y = true,
                    }

                    // reflect velocity on the x-axis if we hit something on the x-axis
                    if reflect_x {
                        movement.flip_x();
                    }

                    // reflect velocity on the y-axis if we hit something on the y-axis
                    if reflect_y {
                        movement.flip_y();
                    }
                }
            }
        }
    }
}
pub fn check_balls_system(
    ball_query: Query<&Ball>,
    mut game_state: ResMut<State<GameState>>,
    mut shooter_count: ResMut<Shooter>,
) {
    if *game_state.current() == GameState::Shooting {
        if ball_query.iter().len() == 0 {
            shooter_count.finished = false;
            shooter_count.shooted = 0;
            let _ = game_state.set(GameState::MovingBlocks);
        }
    }
}
pub fn check_blocks_system(
    block_query: Query<&Block>,
    mut game_state: ResMut<State<GameState>>,
    mut has_won: ResMut<HasWon>,
) {
    if *game_state.current() == GameState::Shooting {
        if block_query.iter().len() == 0 {
            *has_won = Some(true);
            println!("has won");
            let _ = game_state.set(GameState::Init);
        }
    }
}
pub fn move_blocks_system(
    mut collider_query: Query<&mut Transform, With<Block>>,
    mut game_state: ResMut<State<GameState>>,
    mut has_won: ResMut<HasWon>,

) {
    for mut transform in collider_query.iter_mut() {
        if transform.translation.y <= -CONFIG.window_height / 2. + 2. * CONFIG.block_size {
            *has_won = Some(false);
            let _ = game_state.set(GameState::Init);
            return;
        }
        transform.translation += Vec3::new(0., -CONFIG.block_size, 0.);
    }
    let _ = game_state.set(GameState::Aiming);
}

pub fn mouse_listener_system(
    btns: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,

    windows: Res<Windows>,
    mut mouse_pos: ResMut<MousePos>,
    mut game_state: ResMut<State<GameState>>,
) {
    match *game_state.current() {
        GameState::Aiming => {
            let window = windows.get_primary().unwrap();

            if btns.just_pressed(MouseButton::Left) {
                // For multi-window applications, you need to use a specific window ID here.
                if let Some(position) = window.cursor_position() {
                    *mouse_pos = position;
                }
                game_state.set(GameState::Shooting).unwrap();
            }
        },
        GameState::Init => {
            if keys.pressed(KeyCode::Space) {
                let _ = game_state.set(GameState::Aiming);
            }
        },
        GameState::Shooting => {
            if keys.pressed(KeyCode::A) {
                let _ = game_state.set(GameState::MovingBlocks);
            }
        }
        _ => {}
    }

}

pub fn button_system(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>),
    >,
    mut has_won: ResMut<HasWon>,
    mut game_state: ResMut<State<GameState>>,

) {
    for interaction in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                let _ = game_state.set(GameState::MovingBlocks);
                *has_won = None;

            }
            Interaction::Hovered => {
            }
            Interaction::None => {
            }
        }
    }
}
pub fn despawn_button_system(
    mut commands: Commands,
    mut button_query: Query<Entity, With<Button>>,
) {
    button_query
    .iter_mut()
    .for_each(|e| commands.entity(e).despawn_recursive());
}

pub fn despawn_blocks_system(
    mut commands: Commands,
    mut block_query: Query<Entity, With<Block>>,
) {
    block_query
    .iter_mut()
    .for_each(|e| commands.entity(e).despawn_recursive());
}
pub fn despawn_balls_system(
    mut commands: Commands,
    mut ball_query: Query<Entity, With<Ball>>,
    mut shooter: ResMut<Shooter>,
) {
    shooter.reset();
    ball_query
    .iter_mut()
    .for_each(|e| commands.entity(e).despawn_recursive());
}