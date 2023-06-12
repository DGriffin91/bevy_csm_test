use bevy::math::{vec3, vec4};
use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;
use bevy_basic_camera::{CameraController, CameraControllerPlugin};

use std::f32::consts::PI;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugin(CameraControllerPlugin)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform {
                translation: vec3(-4.5723248, 1.9728147, 2.7140265),
                rotation: Quat::from_vec4(vec4(-0.13991262, -0.45213655, -0.07205334, 0.8779552)),
                ..default()
            },
            ..default()
        })
        .insert(CameraController::default());

    commands.spawn(SceneBundle {
        scene: asset_server.load("test_box.glb#Scene0"),
        ..default()
    });

    const HALF_SIZE: f32 = 50.0;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            ..default()
        },

        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 3.) * Quat::from_rotation_y(-0.3),
            ..default()
        },

        ..default()
    });
}
