use bevy::math::Vec2;

pub type MousePos = Vec2;

pub type HasWon = Option<bool>;

#[derive(Debug, Clone)]
pub struct Shooter {
    pub count: u32,
    pub shooted: u32,
    pub finished: bool,
}
impl Default for Shooter {
    fn default() -> Self {
        Shooter {
            count: 40,
            shooted: 0,
            finished: false,
        }
    }
}
impl Shooter {
    pub fn reset(&mut self){
        self.shooted = 0;
        self.finished = false;
    }
}