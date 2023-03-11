use std::collections::HashMap;
use std::iter;
use bevy::math::{Vec2, Vec3};
use ordered_float::OrderedFloat;
use serde::Deserialize;
use crate::map::{Sector, Side};

#[derive(PartialEq, Eq, Hash)]
struct OrdPoint(OrderedFloat<f32>, OrderedFloat<f32>);

impl From<Vec2> for OrdPoint {
    fn from(value: Vec2) -> Self {
        OrdPoint(value.x.into(), value.y.into())
    }
}


#[derive(Deserialize, Debug)]
struct FileSector {
    points: Vec<Vec2>,
    color: Vec3,
    floor: f32,
    ceil: f32
}

#[derive(Deserialize, Debug)]
struct MapFile {
    sectors: Vec<FileSector>
}

pub fn load() -> Vec<Sector> {
    let map_file : MapFile = toml::from_str(include_str!("map.toml")).expect("Cannot load map");

    let mut unmatched_sides = HashMap::<(OrdPoint, OrdPoint), (usize, usize)>::new();
    let mut sectors = Vec::<Sector>::new();

    for (i, sector) in map_file.sectors.iter().enumerate() {
        // Generate a list of sides from the sector's points
        let sides = sector.points.iter().map(|x| *x)
            .zip(sector.points.iter().map(|x| *x).skip(1).chain(iter::once(*sector.points.first().unwrap())));

        sectors.push(Sector {
            floor: sector.floor,
            ceil: sector.ceil,
            color: sector.color,
            sides: sides.clone().map(|(p1, p2)| Side {
                p1, p2,
                looks_into_sector: None
            }).collect()
        });

        // Find matches between the sides of this and other sectors
        for (j, (p1, p2)) in sides.enumerate() {
            if let Some((i_sector, i_side)) = unmatched_sides.get(&(p2.into(), p1.into())) {
                sectors[i].sides[j].looks_into_sector = Some(*i_sector as i32);
                sectors[*i_sector].sides[*i_side].looks_into_sector = Some(i as i32);
                unmatched_sides.remove(&(p2.into(), p1.into()));
            } else {
                unmatched_sides.insert((p1.into(), p2.into()), (i, j));
            }
        }
    }

    sectors
}

