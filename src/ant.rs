use std::f32::consts::PI;

use crate::{
    utils::{calc_rotatio_angle, get_rand_unit_vec2},
    *,
};
use bevy::{
    math::vec3,
    prelude::{
        AssetServer, Color, Commands, Component, Plugin, Quat, Query, Res, Startup, Transform,
        Update, Vec2, Vec3, With,
    },
    sprite::{Sprite, SpriteBundle},
};
pub struct AntPlugin;

#[derive(Component)]
pub struct Ant;
#[derive(Component)]
pub struct Velocity(Vec2);
#[derive(Component)]
pub struct Acceleration(Vec2);

impl Plugin for AntPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_position);
    }
}

fn setup(mut commands: Commands, assert_server: Res<AssetServer>) {
    for _ in 0..NUM_ANTS {
        commands.spawn((
            SpriteBundle {
                texture: assert_server.load(SPRITE_ANT),
                sprite: Sprite {
                    color: Color::rgb(1.1, 1.1, 1.0),
                    ..Default::default()
                },
                transform: Transform::from_xyz(HOME_LOCATION.0, HOME_LOCATION.1, ANT_Z_INDEX)
                    .with_scale(Vec3::splat(ANT_SPRITE_SCALE)),
                ..Default::default()
            },
            Ant,
            Velocity(get_rand_unit_vec2()),
            Acceleration(Vec2::ZERO),
        ));
    }
}

fn check_wall_collision(
    mut ant_query: Query<(&Transform, &Velocity, &mut Acceleration), With<Ant>>,
) {
    for (mut transform, velocity, mut acceleration) in ant_query.iter_mut() {
        let border = 20.0;
        let top_left = (-W / 2.0, H / 2.0);
        let bottom_right = (W / 2.0, -H / 2.0);
        let x_bound = transform.translation.x < top_left.0 + border
            || transform.translation.x >= bottom_right.0 - border;
        let y_bound = transform.translation.y >= top_left.1 - border
            || transform.translation.y < bottom_right.1 + border;
        if x_bound || y_bound {
            
        }
    }
}

fn update_position(
    mut ant_query: Query<(&mut Transform, &mut Velocity, &mut Acceleration), With<Ant>>,
) {
    for (mut transform, mut velocity, mut acceleration) in ant_query.iter_mut() {
        let old_pos = transform.translation;
        if !acceleration.0.is_nan() {
            velocity.0 = (velocity.0 + acceleration.0).normalize();
            let new_translation =
                transform.translation + vec3(velocity.0.x, velocity.0.y, 0.0) * ANT_SPEED;
            if !new_translation.is_nan() {
                transform.translation = new_translation;
            }
        }

        acceleration.0 = Vec2::ZERO;
        transform.rotation =
            Quat::from_rotation_z(calc_rotatio_angle(&old_pos, &transform.translation) + PI / 2.0)
    }
}
