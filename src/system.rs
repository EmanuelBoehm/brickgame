
use bevy::prelude::*;
use heron::{CollisionEvent, Velocity};

use crate::{Collider, GameState, MousePos, Shooter, components::CollisionLayer, constants::CONFIG, entity::{Ball, Block}, resource::HasWon};

pub fn collision_events(
    mut events: EventReader<CollisionEvent>,
    mut block_query: Query<(Entity, &mut Collider), With<Block>>,
) {
    events
    .iter()
    // We care about when the entities "start" to collide
    .filter(|e| e.is_started())
    .filter_map(|event| {
        let (entity_1, entity_2) = event.rigid_body_entities();
        
        let (layers_1, layers_2) = event.collision_layers();

        if layers_1.contains_group(CollisionLayer::Block) {
            Some(entity_1) 
        } else if layers_2.contains_group(CollisionLayer::Block) {
            Some(entity_2)
        } else {
            // This event is not the collision between an enemy and the player. We can ignore it.
            None
        }
    })
    .for_each(|block_entity| {
        let mut collider = block_query.get_mut(block_entity).unwrap().1;
        if let Collider::Block(health) = *collider {
            if health > 0 {
                *collider = Collider::Block(health - 1);
            }
        }
    });
}

pub fn update_block_text(
    mut commands: Commands,
    mut block_query: Query<(Entity, &Children, &Collider), (With<Block>, Changed<Collider>)>,
    mut collider_text_query: Query<&mut Text>,
    mut ball_destroy_ev: EventWriter<BallDestroyEvent>,

){
    for (entity, children,collider) in block_query.iter_mut() {
        if let Collider::Block(health) = collider {
            if health >= &1 {
                if let Ok(mut child) = collider_text_query.get_mut(children[0]) {
                    child.sections[0].value = health.to_string();
                }
            } else {
                commands.entity(entity).despawn_recursive();
                ball_destroy_ev.send(BallDestroyEvent);
            }
        }
    }
}
pub fn ball_wall_collision_system(
    mut commands: Commands,
    mut ball_destroy_ev: EventWriter<BallDestroyEvent>,

    mut ball_query: Query<(Entity, &Transform, &mut Velocity), With<Ball>>,
) {
    for (ball_entity, ball_transform, mut velocity) in ball_query.iter_mut() {
        // checking borders and flip if on wall or despawn on ground
        if ball_transform.translation.y < -CONFIG.window_height / 2.{
            commands.entity(ball_entity).despawn();
            ball_destroy_ev.send(BallDestroyEvent);
            continue;
        }
        if ball_transform.translation.y > CONFIG.window_height / 2. {
            velocity.linear *= Vec3::new(1.,-1.,1.); 
        }
        if ball_transform.translation.x < -CONFIG.window_width / 2. {
            velocity.linear *= Vec3::new(-1.,1.,1.); 
        }
        if ball_transform.translation.x > CONFIG.window_width / 2. {
            velocity.linear *= Vec3::new(-1.,1.,1.); 
        }
    }
}
pub struct BallDestroyEvent;

pub fn check_balls_system(
    ball_query: Query<&Ball>,
    mut game_state: ResMut<State<GameState>>,
    mut ball_destroy_ev: EventReader<BallDestroyEvent>,
    mut shooter_count: ResMut<Shooter>,
) {
    if ball_destroy_ev.iter().count() != 0 {
        if *game_state.current() == GameState::Shooting {
            if ball_query.iter().len() == 0 {
                shooter_count.finished = false;
                shooter_count.shooted = 0;
                let _ = game_state.set(GameState::MovingBlocks);
            }
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