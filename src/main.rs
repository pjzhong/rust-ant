use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

use ants::{ant::AntPlugin, pathviz::PathVizPlugin, pheromone::PheromonePlugin, *};
use bevy_pancam::{PanCam, PanCamPlugin};

#[derive(Component)]
struct FollowCamera;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        /// TODO support resizable
                        resizable: false,
                        focused: true,
                        resolution: (W, H).into(),
                        title: "Ants".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(PanCamPlugin)
        .add_plugins(PheromonePlugin)
        .add_plugins(PathVizPlugin)
        .insert_resource(ClearColor(Color::rgba_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2, 0,
        )))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .add_plugins(AntPlugin)
        .run();
}

fn setup(mut commands: Commands, assert_server: Res<AssetServer>) {
    commands
        .spawn((
            Camera2dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                tonemapping: Tonemapping::TonyMcMapface,
                ..default()
            },
            FollowCamera,
            BloomSettings::default(),
        ))
        .insert(PanCam::default());

    commands.spawn(SpriteBundle {
        texture: assert_server.load(SPRITE_ANT_COLONY),
        sprite: Sprite {
            color: Color::rgb(1.5, 1.5, 1.5),
            ..default()
        },
        transform: Transform::from_xyz(HOME_LOCATION.0, HOME_LOCATION.1, 2.0)
            .with_scale(Vec3::splat(HOME_SPRITE_SCALE)),
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: assert_server.load(SPRITE_FOOD),
        sprite: Sprite {
            color: Color::rgb(1.5, 1.5, 1.5),
            ..default()
        },
        transform: Transform::from_xyz(FOOD_LOCATION.0, FOOD_LOCATION.1, 2.0)
            .with_scale(Vec3::splat(FOOD_SPRITE_SCALE)),
        ..default()
    });
}
