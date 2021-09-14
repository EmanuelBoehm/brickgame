use heron::*;

#[derive(PhysicsLayer)]
pub enum CollisionLayer {
    Ball,
    BlockStandard,
    BlockAddBall,
}