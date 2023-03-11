mod map_file;
mod map;
mod mesh_builder;
mod iter_util;
mod player;

use std::f32::consts::PI;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::iter_util::sets_of_three;
use crate::mesh_builder::build_sector_floor;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_world)
        .add_system(player::update_player)
        .run();
}

fn setup_world(mut commands: Commands,
               mut meshes: ResMut<Assets<Mesh>>,
               mut materials: ResMut<Assets<StandardMaterial>>) {
    for sector in map_file::load() {
        let triangle_indices = sets_of_three(&sector.fanned_indices()).collect();
        let points = sector.all_3d_points().collect();

        commands.spawn((PbrBundle {
            mesh: meshes.add(build_sector_floor(&sector)),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(sector.color.x, sector.color.y, sector.color.z),
                perceptual_roughness: 1.0,
                cull_mode: None,
                ..default()
            }),
            ..default()
        }, Collider::trimesh(points, triangle_indices)));
    }

    commands.insert_resource(AmbientLight {
        color: Color::RED,
        brightness: 0.2,
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
            .into(),
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::PINK,
                ..default()
            }),
            transform: Transform::from_xyz(1.0, 0.5, 1.0),
            ..default()
        },
    ));

    player::build_player_entity(&mut commands);
}
