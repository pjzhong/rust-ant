use std::f32::consts::PI;

use bevy::{
    math::vec2,
    prelude::{Vec2, Vec3},
};
use rand::{thread_rng, Rng};

use crate::*;

pub fn get_rand_unit_vec2() -> Vec2 {
    let mut rng = thread_rng();
    vec2(rng.gen_range(-W..W), rng.gen_range(-H..H))
}

pub fn calc_rotatio_angle(v1: &Vec3, v2: &Vec3) -> f32 {
    let dx = v1.x - v2.x;
    let dy = v1.y - v2.y;

    // Calculate the angle using arctangent (atan2) function
    let angle_rad = dy.atan2(dx);

    if angle_rad < 0.0 {
        angle_rad + 2.0 * PI
    } else {
        angle_rad
    }
}
