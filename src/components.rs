use bevy::math::Vec2;

#[derive(Clone, Debug)]
pub struct Movement{
    direction: Vec2,
    pub speed: f32,
}
impl Movement {
    pub fn new(direction: Vec2, speed: f32) -> Self {
        Movement {direction: direction.normalize(), speed}
    }
    #[allow(unused)]
    pub fn set_x(&mut self, x: f32){
        self.direction.x = x;
        self.normalize();
    }
    #[allow(unused)]
    pub fn set_y(&mut self, y: f32){
        self.direction.y = y;
        self.normalize();
    }
    pub fn flip_x(&mut self){
        self.direction.x = -self.direction.x;
    }

    pub fn flip_y(&mut self){
        self.direction.y = -self.direction.y;
    }

    pub fn x(&self) -> f32 {
        self.direction.x
    }
    pub fn y(&self) -> f32 {
        self.direction.y
    }
    fn normalize(&mut self){
        self.direction.normalize();
    }
}