
#[derive(Debug)]
pub struct Ball;
pub struct Scoreboard {
    pub score: usize,
}
#[derive(Debug)]
pub enum Block {
    Standard(u32),
    AddBall,
}