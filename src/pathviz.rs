use std::{collections::HashMap, time::Duration};

use bevy::{
    prelude::{
        Assets, Commands, Component, Handle, Image, IntoSystemConfigs, Plugin, Query, Res, ResMut,
        Resource, Startup, Transform, Update, Vec3, With,
    },
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    sprite::SpriteBundle,
    time::common_conditions::on_timer,
};

use crate::{
    ant::{Ant, CurrentTask},
    grids::{add_map_to_grid_img, DecayGrid},
    utils::window_to_grid,
    H, PH_UNIT_GRID_SIZE, VIZ_COLOR_STRENGTH, VIZ_COLOR_TO_FOOD, VIZ_COLOR_TO_HOME, VIZ_DECAY_RATE,
    VIZ_MAX_COLOR_STRENGTH, W,
};

#[derive(Component)]
struct PathVizImageRender;

#[derive(Resource)]
pub struct PathVizGrid {
    pub dg_home: DecayGrid,
    pub dg_food: DecayGrid,
}

impl PathVizGrid {
    fn new() -> Self {
        Self {
            dg_home: DecayGrid::new(HashMap::new(), VIZ_MAX_COLOR_STRENGTH),
            dg_food: DecayGrid::new(HashMap::new(), VIZ_MAX_COLOR_STRENGTH),
        }
    }
}

pub struct PathVizPlugin;

impl Plugin for PathVizPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup)
            .insert_resource(PathVizGrid::new())
            .add_systems(Update, update_grid_values)
            .add_systems(
                Update,
                update_path_viz_image.run_if(on_timer(Duration::from_secs_f32(0.1))),
            );
    }
}

///这里更新路径数值
fn update_grid_values(
    ant_query: Query<(&Transform, &CurrentTask), With<Ant>>,
    mut viz_grid: ResMut<PathVizGrid>,
) {
    for (transform, current_task) in ant_query.iter() {
        let x = transform.translation.x as i32;
        let y = transform.translation.y as i32;
        let key = window_to_grid(x, y);

        match current_task.0 {
            crate::ant::AntTask::FindFood => {
                viz_grid.dg_food.add_value(&key, VIZ_COLOR_STRENGTH, 5.0);
            }
            crate::ant::AntTask::FindHome => {
                viz_grid.dg_home.add_value(&key, VIZ_COLOR_STRENGTH, 5.0);
            }
        }
    }

    viz_grid.dg_food.decay_values(VIZ_DECAY_RATE);
    viz_grid.dg_food.drop_zero_values();
    viz_grid.dg_home.decay_values(VIZ_DECAY_RATE);
    viz_grid.dg_home.drop_zero_values();
}

///这里根据路径数值开始渲染蚂蚁路径
fn update_path_viz_image(
    mut textures: ResMut<Assets<Image>>,
    viz_grid: Res<PathVizGrid>,
    mut query: Query<&mut Handle<Image>, With<PathVizImageRender>>,
) {
    let mut image_handle = query.single_mut();
    let (w, h) = (
        W as usize / PH_UNIT_GRID_SIZE,
        H as usize / PH_UNIT_GRID_SIZE,
    );

    let mut bytes = vec![0; w * h * 4];
    add_map_to_grid_img(
        viz_grid.dg_food.get_values(),
        &VIZ_COLOR_TO_FOOD,
        &mut bytes,
        false,
    );
    add_map_to_grid_img(
        viz_grid.dg_home.get_values(),
        &VIZ_COLOR_TO_HOME,
        &mut bytes,
        false,
    );

    let path_img = Image::new(
        Extent3d {
            width: w as u32,
            height: h as u32,
            ..Default::default()
        },
        TextureDimension::D2,
        bytes,
        TextureFormat::Rgba8Unorm,
    );
    *image_handle = textures.add(path_img);
}

fn setup(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1.0)
                .with_scale(Vec3::splat(PH_UNIT_GRID_SIZE as f32)),
            ..Default::default()
        },
        PathVizImageRender,
    ));
}
