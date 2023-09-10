// Global
pub const W: f32 = 1920.0;
pub const H: f32 = 1080.0;

pub const BG_COLOR: (u8, u8, u8) = (0, 0, 0);

// Ant Colony
pub const HOME_LOCATION: (f32, f32) = (759.0, -350.0);
// pub const HOME_LOCATION: (f32, f32) = (300.0, -250.0);
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
