use std::f32::consts::PI;

use bevy::{
    math::{vec2, vec3},
    prelude::{Vec2, Vec3},
};
use rand::{thread_rng, Rng};

pub fn get_rand_unit_vec2() -> Vec2 {
    let rand_vec3 = get_rand_unit_vec3();
    vec2(rand_vec3.x, rand_vec3.y)
}

pub fn get_rand_unit_vec3() -> Vec3 {
    let mut rng = thread_rng();
    vec3(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0).normalize()
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

/// 转向力
pub fn get_steering_force(target: Vec2, current: Vec2, velocity: Vec2) -> Vec2 {
    let desired = target - current;
    let steering = desired - velocity;
    steering * 0.5
}

/// 计算中值
pub fn calc_weighted_midpoint(points: &[(i32, i32, f32)]) -> Vec2 {
    let total_weight: f32 = points.iter().map(|point| point.2).sum();

    let weighted_sum_x: f32 = points.iter().map(|point| point.0 as f32 * point.2).sum();
    let weighted_sum_y: f32 = points.iter().map(|point| point.1 as f32 * point.2).sum();

    let weighted_midpoint_x = weighted_sum_x / total_weight;
    let weighted_midpoint_y = weighted_sum_y / total_weight;

    vec2(weighted_midpoint_x, weighted_midpoint_y)
}
