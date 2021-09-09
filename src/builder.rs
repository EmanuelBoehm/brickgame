use bevy::prelude::*;
use bevy::render::render_graph::base::MainPass;
use bevy::text::Text2dSize;
use crate::{Collider, direction_ball_to_mouse};
use crate::components::Movement;
use crate::constants::CONFIG;
use crate::entity::{Ball, Block};
use crate::resource::MousePos;

type FieldPos = (usize, usize);

pub fn construct_block(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
    field_pos: FieldPos,
    health: u32,
) {
    let xy = field_pos_to_transform(field_pos);
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.2, 0.8, 0.6).into()),
            transform: Transform::from_xyz(xy.0, xy.1, 0.),
            sprite: Sprite::new(Vec2::new(CONFIG.block_size, CONFIG.block_size)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(Text2dBundle {
                draw: Draw {
                    ..Default::default()
                },
                visible: Visible {
                    is_transparent: true,
                    ..Default::default()
                },
                text: Text::with_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    health.to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/Uroob-Regular.ttf"),
                        font_size: CONFIG.block_size / 3. + 10.,
                        color: Color::BLACK,
                    },
                    Default::default(),
                ),
                transform: Transform::from_xyz(0., -CONFIG.block_size / 3., 0.1),
                global_transform: Default::default(),
                main_pass: MainPass {},
                text_2d_size: Text2dSize {
                    size: Size::default(),
                },
            });
        })
        .insert(Collider::Block(health))
        .insert(Block);
        
}

pub fn construct_ball(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    mouse_pos: Res<MousePos>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(0.0, -CONFIG.window_height / 2., 1.0),
            sprite: Sprite::new(Vec2::new(CONFIG.ball_size, CONFIG.ball_size)),
            ..Default::default()
        })
        .insert(Ball)
        .insert(Movement::new(direction_ball_to_mouse(*mouse_pos), CONFIG.ballspeed));        
}

fn field_pos_to_transform(field_pos: FieldPos) -> (f32, f32) {
    //offset in blocks
    let offset = 5;
    let x =
        field_pos.0 as f32 * CONFIG.block_size - CONFIG.window_width / 2. + CONFIG.block_size / 2.;
    let y =
        field_pos.1 as f32 * CONFIG.block_size - CONFIG.window_height / 2. - CONFIG.block_size / 2. + CONFIG.block_size * offset as f32;
    (x, y)
}