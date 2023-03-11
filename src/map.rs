use bevy::math::{Vec2, Vec3};

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
        (0..(self.sides.len() as u32)).collect::<Vec<_>>()
            .windows(2)
            .flat_map(|i| vec![0, i[0], i[1]])
            .collect()
    }

    pub fn floor_points(&self) -> impl Iterator<Item=Vec3> + '_ {
        self.unique_points().map(|p| Vec3::new(p.x, self.floor, p.y))
    }

    pub fn ceil_points(&self) -> Vec<Vec3> {
        self.unique_points().rev().map(|p| Vec3::new(p.x, self.ceil, p.y)).collect()
    }
}
