use std::collections::HashMap;

use bevy::{
    prelude::{Commands, Component, Plugin, Resource, Startup, Transform, Vec3},
    sprite::SpriteBundle,
};

use crate::{grids::DecayGrid, PH_UNIT_GRID_SIZE, VIZ_MAX_COLOR_STRENGTH};

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
            .insert_resource(PathVizGrid::new());
    }
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
