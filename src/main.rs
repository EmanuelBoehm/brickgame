use bevy::prelude::*;
mod entity;
use entity::*;
mod components;
use components::*;
mod system;

const WIDTH: f32 = 800.;
const HEIGHT: f32 = 800.;
const BLOCKSIZE: f32 = 10.;
const _WALLSIZE: f32 = 10.;
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(startup_system.system())
        .add_system(system::ball_movement_system.system())
        .add_system(system::ball_collision_system.system())
        .run();
}

fn startup_system(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // spawn camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    //spawn ball
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(0.0, 215.0, 1.0),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(Ball)
        .insert(Speed(100.))
        .insert(Direction(0., 1.));

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(0.0, 215.0, 1.0),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(Ball)
        .insert(Speed(100.))
        .insert(Direction(0.7, 3.));

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

    construct_blocks(&mut commands, &mut materials, (1, 1), 1);
    construct_blocks(&mut commands, &mut materials, (2, 2), 1);
    construct_blocks(&mut commands, &mut materials, (1, 3), 1);
    construct_blocks(&mut commands, &mut materials, (10, 3), 1);
}
pub struct Scoreboard {
    score: usize,
}
pub enum Collider {
    Block(u32),
    Wall,
}

type FieldPos = (i32, i32);

fn construct_blocks(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    field_pos: FieldPos,
    health: u32,
) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.8, 0.1, 1.0).into()),
            transform: field_pos_to_transform(field_pos),
            sprite: Sprite::new(Vec2::new(20.0, 20.0)),
            ..Default::default()
        })
        .insert(Collider::Block(health));
}
fn field_pos_to_transform(field_pos: FieldPos) -> Transform {
    let x = field_pos.0 as f32 * BLOCKSIZE * 2. - WIDTH / 2. + BLOCKSIZE / 2.;
    let y = HEIGHT / 2. - field_pos.1 as f32 * BLOCKSIZE * 2. - BLOCKSIZE / 2.;

    Transform::from_xyz(x, y, 1.)
}
