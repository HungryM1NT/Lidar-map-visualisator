use std::hash::{Hash, Hasher};

use pcd_rs::{DataKind, ValueKind, ViewPoint};

// pub const SPLIT_NUM: u32 = 10;
pub const POINTS_IN_ONE_CHUNK: u32 = 100_000;

pub struct PCDField {
    pub x: i8,
    pub y: i8,
    pub z: i8,
    pub x_type: Option<ValueKind>,
    pub y_type: Option<ValueKind>,
    pub z_type: Option<ValueKind>,
}

#[derive(Debug, Clone, Copy)]
pub struct MyPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub chunk_x_index: u32,
    pub chunk_y_index: u32,
    pub box_index: u32,
    pub index: u32,
}

impl PartialEq for MyPoint {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for MyPoint {}

impl Hash for MyPoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

pub struct PCDData {
    pub points: Vec<MyPoint>,
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
    pub z_min: f32,
    pub z_max: f32,
    pub chunks_in_one_row: u32,
    pub types: [ValueKind; 3],
    pub viewpoint: ViewPoint,
    pub data_kind: DataKind
}

#[derive(Clone)]
pub struct AreasWithMeta{
    pub areas: Vec<Vec<MyPoint>>,
    pub types: [ValueKind; 3],
    pub viewpoint: ViewPoint,
    pub data_kind: DataKind
}

#[derive(Debug)]
pub enum ChunkSplitter {
    Ok(Vec<Vec<MyPoint>>),
    Try(u32)
}

impl ChunkSplitter {
    pub fn unwrap(self) -> Vec<Vec<MyPoint>> {
        match self {
            ChunkSplitter::Ok(areas) => areas,
            ChunkSplitter::Try(_) => vec![vec![]]
        }
    }
}