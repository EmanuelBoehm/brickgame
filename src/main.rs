use bevy::core::FixedTimestep;
use bevy::prelude::*;
mod constants;
mod entity;
mod builder;
mod resource;
use builder::{construct_ball, construct_block_standard};
use brickgame_mapgen::{map::BrickType, voronoi};
use entity::*;
mod components;
use constants::CONFIG;
use heron::PhysicsPlugin;
use resource::{HasWon, MousePos, Shooter};
use system::{GameEvents, ball_wall_collision_system, button_system, check_blocks_system, collision_events, despawn_balls_system, despawn_blocks_system, despawn_button_system, mouse_listener_system, move_blocks_system, read_game_events, update_block_text};

use crate::builder::construct_block_add_ball;
mod system;

#[macro_use]
extern crate lazy_static;


fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
                title: "Brickgame".to_string(),
                width: CONFIG.window_width,
                height: CONFIG.window_height,
                ..Default::default()
        })
        .add_plugin(PhysicsPlugin::default()) // Add the plugin

        .add_plugins(DefaultPlugins)
        .add_event::<GameEvents>()
        .insert_resource(HasWon::default())
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(Shooter::default())
        .insert_resource(MousePos::ZERO)
        .add_system(collision_events.system())
        .add_system(update_block_text.system())

        // startup
        .add_startup_system(camera_init_system.system())
        //.add_startup_system(physic_init_system.system())
        .add_state(GameState::Init)
        .add_system(mouse_listener_system.system())
        // Gamestate Init
        .add_system_set(
            SystemSet::on_enter(GameState::Init)
                .with_system(despawn_blocks_system.system())
                .with_system(button_setup_system.system())
        )
        .add_system_set(
            SystemSet::on_update(GameState::Init)
                .with_system(button_system.system())
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Init)
                .with_system(despawn_button_system.system())
                .with_system(block_setup.system())
        )
        // Gamestate Shooting
        
        .add_system_set(
            SystemSet::on_update(GameState::Shooting)
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(ball_setup.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Shooting)
                .with_system(read_game_events.system())
                .with_system(check_blocks_system.system())
                .with_system(ball_wall_collision_system.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Shooting)
                .with_system(despawn_balls_system.system())
        )
        // Gamestate MovingBlocks
        .add_system_set(
            SystemSet::on_enter(GameState::MovingBlocks)
            .with_system(move_blocks_system.system()),
        )
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Init,
    Shooting,
    Aiming,
    MovingBlocks,
}


fn block_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let map = voronoi::voronoi_map_gen(
        (CONFIG.window_width as usize / CONFIG.block_size as usize, 
            CONFIG.window_height as usize / CONFIG.block_size as usize)
        );
    for brick in &map.bricks {
        match brick.brick_type {
            BrickType::Standard(health) => {
                construct_block_standard(
                    &mut commands,
                    &mut materials,
                    &asset_server,
                    brick.position,
                    health,
                );
            },
            BrickType::AddBall => {
                construct_block_add_ball(&mut commands, &mut materials, &asset_server,brick.position)
            },
            BrickType::None => {},
        }
        
    }

}

fn ball_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut shooter_count: ResMut<Shooter>,
    mouse_pos: Res<MousePos>,
    game_state: Res<State<GameState>>,
    mut asset_server: ResMut<AssetServer>,

) {
    if *game_state.current() == GameState::Shooting {
        if !shooter_count.finished {
            construct_ball(&mut commands, &mut materials, &mut asset_server, mouse_pos);
            shooter_count.shooted += 1;
            if shooter_count.shooted == shooter_count.count {
                shooter_count.shooted = 0;
                shooter_count.finished = true;
            }
        }
    }
}

fn camera_init_system(mut commands: Commands) {
    // spawn camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}


fn direction_ball_to_mouse(mouse_pos: Vec2) -> Vec2 {
    let mut position = mouse_pos.clone();
    position.x -= CONFIG.window_width / 2.;
    Vec2::new(position.x, position.y)
}

fn button_setup_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    has_won: Res<HasWon>,
){
    let size = Size::new(Val::Px(CONFIG.window_width / 2.), Val::Px(CONFIG.window_height / 16.));
    match *has_won {
        None => {
            let message = "Init new Game with space or click!";
                println!("{}", message);
                commands
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size,
                        // center button
                        margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: materials.add(Color::rgb(0.1, 0.5, 0.3).into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            message,
                            TextStyle {
                                font: asset_server.load("fonts/Uroob-Regular.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
        },
        Some(won) => {
            if won {
                let message = "you won. Init new Game with space or click!";
                println!("{}", message);
                commands
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size,
                        // center button
                        margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: materials.add(Color::rgb(0.1, 0.5, 0.3).into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            message,
                            TextStyle {
                                font: asset_server.load("fonts/Uroob-Regular.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
            } else {
                let message = "you lost. Init new Game with space or click!";
                println!("{}", message);
                commands
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size,
                        // center button
                        margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: materials.add(Color::rgb(0.1, 0.5, 0.3).into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            message,
                            TextStyle {
                                font: asset_server.load("fonts/Uroob-Regular.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
            }
        },
    }
}
