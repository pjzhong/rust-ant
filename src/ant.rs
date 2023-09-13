use std::{f32::consts::PI, time::Duration, slice::Windows};

use crate::{
    utils::{calc_rotatio_angle, get_rand_unit_vec2, get_steering_force},
    *,
};
use bevy::{
    math::{vec2, vec3},
    prelude::{
        AssetServer, Color, Commands, Component, IntoSystemConfigs, Plugin, Quat, Query, Res,
        Startup, Transform, Update, Vec2, Vec3, With,
    },
    sprite::{Sprite, SpriteBundle},
    time::common_conditions::on_timer, window::Window,
};
use rand::{thread_rng, Rng};
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
            .add_systems(
                Update,
                check_wall_collision.run_if(on_timer(Duration::from_secs_f32(0.1))),
            )
            .add_systems(Update, update_position.after(check_wall_collision));
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
    window: Query<&Window>,
) {
    let window = window.single();
    let (w, h) = (window.width(), window.height());

    for (transform, velocity, mut acceleration) in ant_query.iter_mut() {
        let border = 20.0;
        let top_left = (-w / 2.0, h / 2.0);
        let bottom_right = (w / 2.0, -h / 2.0);
        let x_bound = transform.translation.x < top_left.0 + border
            || transform.translation.x >= bottom_right.0 - border;
        let y_bound = transform.translation.y >= top_left.1 - border
            || transform.translation.y < bottom_right.1 + border;
        if x_bound || y_bound {
            let mut rng = thread_rng();
            let target = vec2(rng.gen_range(-200.0..200.0), rng.gen_range(-200.0..200.0));
            acceleration.0 +=
                get_steering_force(target, transform.translation.truncate(), velocity.0);
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
        //调整图像旋转角度
        transform.rotation =
            Quat::from_rotation_z(calc_rotatio_angle(&old_pos, &transform.translation) + PI / 2.0)
    }
}
