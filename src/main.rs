use bevy::prelude::*;
use bevy::core::FixedTimestep;
mod entity;
use bevy_asset_ron::RonAssetPlugin;
use entity::*;
mod components;
use components::*;
mod system;

const WIDTH: f32 = 800.;
const HEIGHT: f32 = 800.;
const BLOCKSIZE: f32 = 40.;
const _WALLSIZE: f32 = 10.;
const BALLSPEED: f32 = 500.;

pub type MousePos = Vec2;
fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
        title: "Tetris".to_string(),width: WIDTH, height: HEIGHT,..Default::default()})    
        .add_plugins(DefaultPlugins)
        // .add_plugin(
        //     // load `*.item` files
        //     RonAssetPlugin::<Map>::new("item")
        // )
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(Shooter::default())
        .insert_resource(MousePos::ZERO)
        .add_state(GameState::Aiming)
        .add_startup_system(startup_system.system())
        .add_system_set(
            SystemSet::on_enter(GameState::Aiming)
            .with_system(block_setup.system())
        ).add_system_set(
            SystemSet::on_update(GameState::Aiming)
                .with_system(system::mouse_listener_system.system())
        ).add_system_set(
            SystemSet::on_update(GameState::Shooting)
                .with_run_criteria(FixedTimestep::step(0.2))
                .with_system(ball_setup.system())
        )
        .add_system_set(
            SystemSet::on_update(GameState::Shooting)
                .with_system(system::ball_movement_system.system())
                .with_system(system::ball_collision_system.system())
        )
        .run();
 }
// #[derive(serde::Deserialize)]
// #[derive(TypeUuid)]
// #[uuid = "1df82c01-9c71-4fa8-adc4-78c5822268f8"]
// struct Map {
//     blocks: Vec<Block>,
// }
// #[derive(serde::Deserialize)]
// struct Block {
//     health: u32,
//     position: Vec2,
// }

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Shooting,
    Aiming,
}

fn block_setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    construct_block(&mut commands, &mut materials, (1, 1), 1);
    construct_block(&mut commands, &mut materials, (2, 2), 1);
    construct_block(&mut commands, &mut materials, (1, 3), 1);
    construct_block(&mut commands, &mut materials, (10, 3), 1);
}

#[derive(Debug,Clone)]
struct Shooter{
    pub count: u32,
    pub shooted: u32,
    pub finished: bool,
}
impl Default for Shooter {
    fn default() -> Self {
        Shooter { count: 10, shooted: 0, finished: false }
    }
}

fn ball_setup(
    mut commands: Commands, 
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut shooter_count: ResMut<Shooter>,
    mouse_pos: Res<MousePos>,
    game_state: Res<State<GameState>>,

) {
    if *game_state.current() == GameState::Shooting {
        if !shooter_count.finished {
            construct_ball(&mut commands, &mut materials, mouse_pos);
            shooter_count.shooted += 1;
            if shooter_count.shooted == shooter_count.count {
                shooter_count.shooted = 0;
                shooter_count.finished = true;
            }
        }
    }
}

fn startup_system(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // spawn camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    rand::Rng
    //spawn outer walls
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.1, 0.5, 1.0).into()),
            transform: Transform::from_xyz(0.0, HEIGHT / 2., 1.0),
            sprite: Sprite::new(Vec2::new(WIDTH + 10., 10.0)),
            ..Default::default()
        })
        .insert(Collider::Wall);

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.1, 0.5, 1.0).into()),
            transform: Transform::from_xyz(WIDTH / 2., 0.0, 1.0),
            sprite: Sprite::new(Vec2::new(10.0, HEIGHT)),
            ..Default::default()
        })
        .insert(Collider::Wall);

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.1, 0.5, 1.0).into()),
            transform: Transform::from_xyz(-WIDTH / 2., 0.0, 1.0),
            sprite: Sprite::new(Vec2::new(10.0, HEIGHT)),
            ..Default::default()
        })
        .insert(Collider::Wall);
}
pub struct Scoreboard {
    score: usize,
}
pub enum Collider {
    Block(u32),
    Wall,
}

type FieldPos = (i32, i32);

fn construct_block(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    field_pos: FieldPos,
    health: u32,
) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.8, 0.1, 1.0).into()),
            transform: field_pos_to_transform(field_pos),
            sprite: Sprite::new(Vec2::new(BLOCKSIZE, BLOCKSIZE)),
            ..Default::default()
        })
        .insert(Collider::Block(health));
}

fn field_pos_to_transform(field_pos: FieldPos) -> Transform {
    let x = field_pos.0 as f32 * BLOCKSIZE - WIDTH / 2. + BLOCKSIZE / 2.;
    let y = HEIGHT / 2. - field_pos.1 as f32 * BLOCKSIZE - BLOCKSIZE / 2.;
    Transform::from_xyz(x, y, 1.)
}

fn construct_ball(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    mouse_pos: Res<MousePos>,
) {
    commands
    .spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
        transform: Transform::from_xyz(0.0, -HEIGHT / 2., 1.0),
        sprite: Sprite::new(Vec2::new(10.0, 10.0)),
        ..Default::default()
    })
    .insert(Ball)
    .insert(Speed(BALLSPEED))
    .insert(direction_ball_to_mouse(*mouse_pos));
}

fn direction_ball_to_mouse(mouse_pos: Vec2) -> MoveDirection {
    let mut position = mouse_pos.clone();
    position.x -= WIDTH / 2.;
    MoveDirection(position.normalize().x, position.normalize().y)
}