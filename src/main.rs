mod map_file;
mod map;
mod mesh_builder;
mod iter_util;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::render::{mesh, RenderPlugin};
use bevy::render::mesh::PrimitiveTopology;
use bevy::pbr::wireframe::Wireframe;
use bevy::render::settings::{WgpuFeatures, WgpuSettings};
use crate::iter_util::sets_of_three;
use crate::mesh_builder::build_sector_floor;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            wgpu_settings: WgpuSettings {
                features: WgpuFeatures::POLYGON_MODE_LINE,
                ..default()
            }
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_world)
        .run();
}

fn setup_world(mut commands: Commands,
               mut meshes: ResMut<Assets<Mesh>>,
               mut materials: ResMut<Assets<StandardMaterial>>) {
    for sector in map_file::load() {
        let triangle_indices = sets_of_three(&sector.fanned_indices()).collect();
        let points = sector.floor_points().collect();

        commands.spawn((PbrBundle {
            mesh: meshes.add(build_sector_floor(&sector)),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(sector.color.x, sector.color.y, sector.color.z),
                unlit: true,
                ..default()
            }),
            ..default()
        }, Wireframe, Collider::trimesh(points, triangle_indices)));
    }
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(8.0, 9.0, 25.0).looking_to(Vec3::new(0.0, -1.0, -1.2), Vec3::Y),
        ..default()
    });
}