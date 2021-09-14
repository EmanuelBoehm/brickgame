
use bevy::prelude::*;
use bevy::render::render_graph::base::MainPass;
use bevy::text::Text2dSize;
use heron::{CollisionLayers, CollisionShape, PhysicMaterial, RigidBody, Velocity};
use crate::direction_ball_to_mouse;
use crate::components::{CollisionLayer};
use crate::constants::CONFIG;
use crate::entity::{Ball, Block};
use crate::resource::MousePos;

type FieldPos = (usize, usize);

pub fn construct_block_standard(
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
                transform: Transform::from_xyz(CONFIG.block_size/10., -CONFIG.block_size / 3., 0.1),
                global_transform: Default::default(),
                main_pass: MainPass {},
                text_2d_size: Text2dSize {
                    size: Size::default(),
                },
            });
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(CONFIG.block_size / 2., CONFIG.block_size / 2.,0.),
            border_radius: Some(1.),
        })  
        .insert(PhysicMaterial {
            restitution: 1.,
            ..Default::default()
        })
        .insert(
            CollisionLayers::none()
                .with_group(CollisionLayer::BlockStandard)
                .with_mask(CollisionLayer::Ball),
        )
        .insert(Block::Standard(health));
        
}

pub fn construct_block_add_ball(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
    field_pos: FieldPos,
) {
    let asset: Handle<Texture> = asset_server.load("pic/upgrade_live.png");

    let xy = field_pos_to_transform(field_pos);
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(asset.into()),
            transform: Transform::from_xyz(xy.0, xy.1, 0.),
            sprite: Sprite::new(Vec2::new(CONFIG.block_size, CONFIG.block_size)),
            ..Default::default()
        }).with_children(|parent| {
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
                    "+1".to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/Uroob-Regular.ttf"),
                        font_size: CONFIG.block_size / 3. + 10.,
                        color: Color::BLACK,
                    },
                    Default::default(),
                ),
                transform: Transform::from_xyz(CONFIG.block_size/10., -CONFIG.block_size / 3., 0.1),
                global_transform: Default::default(),
                main_pass: MainPass {},
                text_2d_size: Text2dSize {
                    size: Size::default(),
                },
            });
        })
        .insert(RigidBody::Sensor)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(CONFIG.block_size / 2., CONFIG.block_size / 2.,0.),
            border_radius: None,
        })  
        .insert(PhysicMaterial {
            restitution: 1.,
            ..Default::default()
        })
        .insert(
            CollisionLayers::none()
                .with_group(CollisionLayer::BlockAddBall)
                .with_mask(CollisionLayer::Ball),
        )
        .insert(Block::AddBall);
        
}
pub fn construct_ball(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &mut ResMut<AssetServer>,

    mouse_pos: Res<MousePos>,
) {
    let asset: Handle<Texture> = asset_server.load("pic/ball.png");
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(asset.into()),
            transform: Transform::from_xyz(0.0, -CONFIG.window_height / 2., 1.0),
            sprite: Sprite::new(Vec2::new(CONFIG.ball_size, CONFIG.ball_size)),
            ..Default::default()    
        })
        .insert(Ball)
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Sphere { radius: CONFIG.ball_size })
        .insert(PhysicMaterial {
            restitution: 1.,
            ..Default::default()
        })
        .insert(
            CollisionLayers::none()
                .with_group(CollisionLayer::Ball)
                .with_masks(vec![CollisionLayer::BlockStandard, CollisionLayer::BlockAddBall]),
        )
        .insert(Velocity::from(direction_ball_to_mouse(*mouse_pos) * CONFIG.ballspeed));
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