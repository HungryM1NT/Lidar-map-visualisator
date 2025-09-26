// pub const SPLIT_NUM: u32 = 10;
pub const POINTS_IN_ONE_CHUNK: u32 = 100_000;

pub struct PCDField {
    pub x: i8,
    pub y: i8,
    pub z: i8
}

#[derive(Debug, Clone)]
pub struct MyPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub chunk_x_index: u32,
    pub chunk_y_index: u32,
    pub box_index: u32,
    pub index: u32,
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
}

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