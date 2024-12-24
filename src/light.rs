use std::f32::consts::PI;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy::utils::default;

pub struct AppLightPlugin;

impl Plugin for AppLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
) {
    // commands.spawn(PointLightBundle {
    //     transform: Transform::from_xyz(0.0, 1.0, -5.0),
    //     point_light: PointLight {
    //         color: Color::srgb(1.0, 0.0, 0.5),
    //         intensity: 500_000.0,
    //         shadows_enabled: true,
    //         range: 1000.0,
    //         ..default()
    //     },
    //     ..default()
    // });
    // Quat::from_rotation_x(-PI / 4.)

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 2000.0,
            shadows_enabled: true,
            ..default()
        },
        transform:Transform::from_xyz(1.0, 1.0, -1.0).looking_at(Vec3::ZERO, Vec3::Y),
        // transform: Transform {
        //     translation: Vec3::new(0.0, 2.0, 0.0),
        //     rotation: Quat::from_rotation_x(-PI / 4.),
        //     ..default()
        // },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 40.0,
            maximum_distance: 100.0,
            ..default()
        }
        .into(),
        ..default()
    });
    
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 100.0,
    });
}