use bevy::{
    prelude::{Commands, Plugin, Resource, Startup, Transform, Vec3},
    sprite::SpriteBundle,
};

use crate::{grids::WorldGrid, PH_UNIT_GRID_SIZE};

pub struct PheromonePlugin;

impl Plugin for PheromonePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Resource)]
pub struct Pheromones {
    pub to_home: WorldGrid,
    pub to_food: WorldGrid,
}

fn setup(mut commands: Commands) {
    commands.spawn((SpriteBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .with_scale(Vec3::splat(PH_UNIT_GRID_SIZE as f32)),
        ..Default::default()
    },));
}
