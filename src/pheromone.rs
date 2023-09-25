use std::{collections::HashMap, time::Duration};

use bevy::{
    prelude::{
        Assets, Commands, Component, Handle, Image, IntoSystemConfigs, Plugin, Query, Res, ResMut,
        Resource, Startup, Transform, Update, Vec3, With,
    },
    sprite::SpriteBundle,
    time::common_conditions::on_timer,
};

use crate::{
    grids::WorldGrid, FOOD_LOCATION, HOME_LOCATION, PH_COLOR_TO_FOOD, PH_COLOR_TO_HOME,
    PH_DECAY_INTERVAL, PH_IMG_UPDATE_SEC, PH_KD_TREE_UPDATE_INTERVAL,
};

use crate::PH_UNIT_GRID_SIZE;

#[derive(Default)]
pub struct PheromonePlugin;

#[derive(Component)]
struct PheromoneImageRender;

impl Plugin for PheromonePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup)
            .add_systems(
                Update,
                pheronone_decay.run_if(on_timer(Duration::from_secs_f32(PH_DECAY_INTERVAL))),
            )
            .add_systems(
                Update,
                clear_zero_signals.run_if(on_timer(Duration::from_secs_f32(2.0))),
            )
            .add_systems(
                Update,
                update_kd_tree.run_if(on_timer(Duration::from_secs_f32(
                    PH_KD_TREE_UPDATE_INTERVAL,
                ))),
            )
            .add_systems(
                Update,
                pheromone_image_update.run_if(on_timer(Duration::from_secs_f32(PH_IMG_UPDATE_SEC))),
            )
            .insert_resource(Pheromones::new());
    }
}

#[derive(Resource)]
pub struct Pheromones {
    pub to_home: WorldGrid,
    pub to_food: WorldGrid,
}

fn pheronone_decay(mut pheronones: ResMut<Pheromones>) {
    pheronones.to_food.decay_signals();
    pheronones.to_home.decay_signals();
}

fn clear_zero_signals(mut pheromones: ResMut<Pheromones>) {
    pheromones.to_food.drop_zero_signals();
    pheromones.to_home.drop_zero_signals();
}

fn update_kd_tree(mut pheromones: ResMut<Pheromones>) {
    pheromones.update_tree()
}

impl Pheromones {
    fn new() -> Self {
        let mut to_food_map = HashMap::new();
        let mut to_home_map = HashMap::new();

        to_food_map.insert((FOOD_LOCATION.0 as i32, FOOD_LOCATION.1 as i32), 100000.0);
        to_home_map.insert((HOME_LOCATION.0 as i32, HOME_LOCATION.1 as i32), 100000.0);

        Self {
            to_food: WorldGrid::new(PH_COLOR_TO_FOOD, to_food_map),
            to_home: WorldGrid::new(PH_COLOR_TO_HOME, to_home_map),
        }
    }

    fn update_tree(&mut self) {
        self.to_food.update_tree();
        self.to_home.update_tree();
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .with_scale(Vec3::splat(PH_UNIT_GRID_SIZE as f32)),
            ..Default::default()
        },
        PheromoneImageRender,
    ));
}

fn pheromone_image_update(
    mut textures: ResMut<Assets<Image>>,
    pheromone: Res<Pheromones>,
    image_handle_query: Query<&mut Handle<Image>, With<PheromoneImageRender>>,
) {
}
