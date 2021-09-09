use std::fs::File;

use ron::de::from_reader;

pub const CONFIGPATH: &str = "assets/config/config.ron";

lazy_static! {
    pub static ref CONFIG: ConstantsManager =
        from_reader(File::open(&CONFIGPATH).expect("Failed opening file")).unwrap();
}
#[derive(serde::Deserialize, Default)]
pub struct ConstantsManager {
    pub block_size: f32,
    pub ball_size: f32,
    pub window_width: f32,
    pub window_height: f32,
    pub ballspeed: f32,
    
}
