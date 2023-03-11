use bevy::math::Vec4Swizzles;
use bevy::prelude::*;

pub struct Sector {
    pub floor: f32,
    pub ceil: f32,
    pub color: Vec3,
    pub sides: Vec<Side>
}

pub struct Side {
    pub p1: Vec2,
    pub p2: Vec2,
    pub looks_into_sector: Option<i32>
}

impl Sector {
    pub fn unique_points(&self) -> impl DoubleEndedIterator<Item=Vec2> + '_ {
        self.sides.iter().map(|s| s.p1)
    }

    pub fn fanned_indices(&self) -> Vec<u32> {
        let len = self.sides.len() as u32;

        let range = (0..len).collect::<Vec<_>>();

        let indices_floor = range
            .windows(2)
            .skip(1)
            .flat_map(|i| vec![0, i[0], i[1]]);

        let indices_ceil = range
            .windows(2)
            .skip(1)
            .flat_map(|i| vec![i[1], i[0], 0])
            .map(|i| i + len);

        let indices_walls = self.sides.iter().enumerate()
            .filter(|(_, s)| s.looks_into_sector.is_none())
            .map(|(i, _)| i as u32)
            .flat_map(|i| vec![((i+1)%len)+len, ((i+1)%len), i, i, i+len, ((i+1)%len)+len]);

        indices_floor.chain(indices_ceil).chain(indices_walls).collect()
    }

    pub fn all_3d_points(&self) -> impl Iterator<Item=Vec3> + '_ {
        let floor_points = self.unique_points().map(|p| Vec3::new(p.x, self.floor, p.y));
        let ceil_points = self.unique_points().map(|p| Vec3::new(p.x, self.ceil, p.y));

        floor_points.chain(ceil_points)
    }

    pub fn all_3d_normals(&self) -> impl Iterator<Item=Vec3> + '_ {
        let floor_points = self.unique_points().map(|_| Vec3::Y);
        let ceil_points = self.unique_points().map(|_| Vec3::NEG_Y);

        floor_points.chain(ceil_points)
    }

    pub fn neighbour_portal_points(&self) -> impl Iterator<Item=Vec3> + '_ {
        self.sides.iter()
            // Only sides that link to another sector
            .filter(|s| s.looks_into_sector.is_some())
            .flat_map(|s| [])
    }
}
