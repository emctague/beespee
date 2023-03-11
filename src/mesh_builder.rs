use bevy::prelude::*;
use bevy::render::mesh;
use crate::map::Sector;

pub fn build_sector_floor(sector: &Sector) -> Mesh {
    let points : Vec<Vec3> = sector.floor_points().collect();

    let mut sector_mesh = Mesh::new(mesh::PrimitiveTopology::TriangleList);
    sector_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);
    sector_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![Vec3::Y;points.len()]);
    sector_mesh.set_indices(Some(mesh::Indices::U32(sector.fanned_indices())));

    sector_mesh
}