
use bevy::prelude::*;
use heron::{CollisionEvent, Velocity};

use crate::{GameState, MousePos, Shooter, components::CollisionLayer, constants::CONFIG, entity::{Ball, Block}, resource::HasWon};

pub fn collision_events(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut game_events: EventWriter<GameEvents>,
    mut block_query: Query<(Entity, &mut Block)>,
) {
    collision_events
    .iter()
    // We care about when the entities "stopp" to collide
    .filter(|e| e.is_stopped())
    .filter_map(|event| {
        let (entity_1, entity_2) = event.rigid_body_entities();
        
        let (layers_1, layers_2) = event.collision_layers();

        if !layers_1.contains_group(CollisionLayer::Ball) {
            Some(entity_1)
        } else if !layers_2.contains_group(CollisionLayer::Ball) {
            Some(entity_2)
        } else {
            None
        }
    })
    .for_each(|block_entity| {
        let may_block = block_query.get_mut(block_entity);
        if let Ok(mut block) = may_block {
            match *block.1 {
                Block::Standard(health) => {
                    if health > 0 {
                        *block.1 = Block::Standard(health - 1);
                    }
                },
                Block::AddBall => {
                    game_events.send(GameEvents::AddBall);
                    commands.entity(block_entity).despawn_recursive();
                },
            }
        }
        
    });
}
pub fn read_game_events(
    mut game_events: EventReader<GameEvents>,
    mut shooter_count: ResMut<Shooter>,
    ball_query: Query<&Ball>,
    mut game_state: ResMut<State<GameState>>,
    

){
    for game_event in game_events.iter() {
        match *game_event {
            GameEvents::AddBall => {
                shooter_count.count += 1;

            },
            GameEvents::DestroyBall => {
                if *game_state.current() == GameState::Shooting {
                    if ball_query.iter().len() <= 1 {
                        shooter_count.finished = false;
                        shooter_count.shooted = 0;
                        let _ = game_state.set(GameState::MovingBlocks);
                    }
                }
            }
        }
    }
}

pub fn update_block_text(
    mut commands: Commands,
    block_query: Query<(Entity, &Children, &Block), Changed<Block>>,
    mut collider_text_query: Query<&mut Text>,

){
    for (entity, children,block) in block_query.iter() {
        if let &Block::Standard(health) = block {
            if health >= 1 {
                if let Ok(mut child) = collider_text_query.get_mut(children[0]) {
                    child.sections[0].value = health.to_string();
                }
            } else {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
pub fn ball_wall_collision_system(
    mut commands: Commands,
    mut game_events: EventWriter<GameEvents>,
    mut ball_query: Query<(Entity, &Transform, &mut Velocity), With<Ball>>,
) {
    for (ball_entity, ball_transform, mut velocity) in ball_query.iter_mut() {
        // checking borders and flip if on wall or despawn on ground
        if ball_transform.translation.y < -CONFIG.window_height / 2.{

            game_events.send(GameEvents::DestroyBall);
            commands.entity(ball_entity).despawn();
            continue;
        }
        if ball_transform.translation.y > CONFIG.window_height / 2. && velocity.linear.y > 0. {
            velocity.linear *= Vec3::new(1.,-1.,1.); 
        }
        if ball_transform.translation.x < -CONFIG.window_width / 2. && velocity.linear.x < 0. {
            velocity.linear *= Vec3::new(-1.,1.,1.); 
        }
        if ball_transform.translation.x > CONFIG.window_width / 2. && velocity.linear.x > 0. {
            velocity.linear *= Vec3::new(-1.,1.,1.); 
        }
    }
}
#[derive(Debug,PartialEq)]
pub enum GameEvents {
    DestroyBall,
    AddBall,
}

pub fn check_blocks_system(
    block_query: Query<&Block>,
    mut game_state: ResMut<State<GameState>>,
    mut has_won: ResMut<HasWon>,
) {
    if *game_state.current() == GameState::Shooting {
        if block_query.iter().len() == 0 {
            *has_won = Some(true);
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
    interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>),
    >,
    mut has_won: ResMut<HasWon>,
    mut game_state: ResMut<State<GameState>>,
) {
    for interaction in interaction_query.iter() {
        
        if *interaction == Interaction::Clicked {

            let _ = game_state.set(GameState::MovingBlocks);
            *has_won = None;
        }
    }
}
pub fn despawn_button_system(
    mut commands: Commands,
    button_query: Query<Entity, With<Button>>,
) {
    button_query
    .iter()
    .for_each(|e| commands.entity(e).despawn_recursive());
}

pub fn despawn_blocks_system(
    mut commands: Commands,
    block_query: Query<Entity, With<Block>>,
) {
    block_query
    .iter()
    .for_each(|e| commands.entity(e).despawn_recursive());
}
pub fn despawn_balls_system(
    mut commands: Commands,
    ball_query: Query<Entity, With<Ball>>,
    mut shooter: ResMut<Shooter>,
) {
    shooter.reset();
    ball_query
    .iter()
    .for_each(|e| commands.entity(e).despawn_recursive());
}