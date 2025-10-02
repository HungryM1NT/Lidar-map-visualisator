use std::hash::{Hash, Hasher};
use std::f32::INFINITY;
use bevy::math::Vec3;
use pcd_rs::{DataKind, ValueKind, ViewPoint, Field};

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

pub fn get_area_center(area_points: &Vec<MyPoint>) -> Vec3 {
    let mut x_min: f32 = INFINITY;
    let mut x_max: f32 = -INFINITY;
    let mut y_min: f32 = INFINITY;
    let mut y_max: f32 = -INFINITY;
    let mut z_min: f32 = INFINITY;
    let mut z_max: f32 = -INFINITY;
    for point in area_points {
        x_min = x_min.min(point.x);
        x_max = x_max.max(point.x);
        y_min = y_min.min(point.y);
        y_max = y_max.max(point.y);
        z_min = z_min.min(point.z);
        z_max = z_max.max(point.z);
    }
    let x_mid = (x_max + x_min) / 2.;
    let y_mid = (y_max + y_min) / 2.;
    let z_mid = (z_max + z_min) / 2.;
    Vec3{x: x_mid, y: y_mid, z: z_mid}
}

pub fn field_to_value(field: &Field) -> f32 {
    match field.kind() {
        ValueKind::F32 => {field.to_value::<f32>().unwrap() as f32},
        ValueKind::F64 => {field.to_value::<f64>().unwrap() as f32},
        ValueKind::I8 => {field.to_value::<i8>().unwrap() as f32},
        ValueKind::I16 => {field.to_value::<i16>().unwrap() as f32},
        ValueKind::I32 => {field.to_value::<i32>().unwrap() as f32},
        ValueKind::U8 => {field.to_value::<u8>().unwrap() as f32},
        ValueKind::U16 => {field.to_value::<u16>().unwrap() as f32},
        ValueKind::U32 => {field.to_value::<u32>().unwrap() as f32},
    }
}

pub fn get_field_from_valuekind(value: f32, value_type: ValueKind) -> Field {
    match value_type {
        ValueKind::F32 => {Field::F32(vec![value])},
        ValueKind::F64 => {Field::F64(vec![value.into()])},
        ValueKind::I8 => {Field::I8(vec![value as i8])},
        ValueKind::I16 => {Field::I16(vec![value as i16])},
        ValueKind::I32 => {Field::I32(vec![value as i32])},
        ValueKind::U8 => {Field::U8(vec![value as u8])},
        ValueKind::U16 => {Field::U16(vec![value as u16])},
        ValueKind::U32 => {Field::U32(vec![value as u32])},
    }
}
