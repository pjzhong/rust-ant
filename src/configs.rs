// Global
pub const W: f32 = 1920.0;
pub const H: f32 = 1080.0;

pub const BG_COLOR: (u8, u8, u8) = (0, 0, 0);

// Ant Colony
pub const HOME_LOCATION: (f32, f32) = (759.0, -350.0);
pub const HOME_SPRITE_SCALE: f32 = 2.5;
pub const HOME_RADIUS: f32 = 30.0;

// Food
// pub const FOOD_LOCATION: (f32, f32) = (-400.0, 300.0);
pub const FOOD_LOCATION: (f32, f32) = (-750.0, 400.0);
pub const FOOD_PICKUP_RADIUS: f32 = 30.0;
pub const FOOD_SPRITE_SCALE: f32 = 2.0;

// Sprites
pub const SPRITE_ANT: &str = "ant.png";
pub const SPRITE_ANT_WITH_FOOD: &str = "ant_with_food.png";
pub const SPRITE_ANT_COLONY: &str = "nest.png";
pub const SPRITE_FOOD: &str = "food.png";

// Ants
pub const NUM_ANTS: u32 = 5000;
pub const ANT_SPEED: f32 = 1.5;
pub const ANT_DIRECTION_RANDOMNESS_DEG: f32 = 300.0;
pub const ANT_DIRECTION_UPDATE_INTERVAL: f32 = 0.5;
pub const ANT_SPRITE_SCALE: f32 = 0.3;
pub const ANT_Z_INDEX: f32 = 3.0;
pub const ANT_INITIAL_PH_STRENGTH: f32 = 32.0;
pub const ANT_PH_STRENGTH_DECAY_RATE: f32 = 0.7;
pub const ANT_PH_STRENGTH_DECAY_INTERVAL: f32 = 0.5;
pub const ANT_PH_DROP_INTERVAL: f32 = 0.7;
pub const INITIAL_ANT_PH_SCAN_RADIUS: f32 = 15.0;
pub const ANT_PH_SCAN_RADIUS_INCREMENT: f32 = 0.1;
pub const ANT_PH_SCAN_RADIUS_SCALE: f32 = 1.8;
pub const ANT_STEERING_FORCE_FACTOR: f32 = 0.7;
pub const ANT_TARGET_AUTO_PULL_RADIUS: f32 = 100.0;
