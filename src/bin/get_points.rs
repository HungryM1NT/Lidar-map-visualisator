use std::{cmp::min, f32::INFINITY};

use pcd_rs::{DynReader, Field, PcdMeta, ValueKind};


const SPLIT_NUM: u32 = 10;
struct PCDField {
    x: i8,
    y: i8,
    z: i8
}

#[derive(Debug)]
pub struct MyPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub chunk: u32,
    pub index: u32,
}

fn get_xyz_indexes(meta: PcdMeta) -> Result<PCDField, String> {
    let mut pcd_field = PCDField{x:-1, y:-1, z:-1};
    for (i, field) in meta.field_defs.iter().enumerate() {
        match field.name.as_str() {
            "x" => {pcd_field.x = i.try_into().unwrap()},
            "y" => {pcd_field.y = i.try_into().unwrap()},
            "z" => {pcd_field.z = i.try_into().unwrap()},
            _ => {}
        }
    }
    if pcd_field.x < 0 || pcd_field.y < 0 || pcd_field.z < 0 {
        return Err(String::from("Invalid field type"))
    }
    Ok(pcd_field)
}

pub fn get_points(path: &str) -> Result<Vec<MyPoint>, String> {
    let reader = DynReader::open(path).unwrap();

    let meta = reader.meta().clone();
    let xyz_indexes = get_xyz_indexes(meta).unwrap();
    
    let mut points: Vec<MyPoint> = Vec::new();
    let file_points: Vec<_> = reader.collect::<Result<_, _>>().unwrap();

    let mut index = 0;
    let mut x_min: f32 = INFINITY;
    let mut x_max: f32 = -INFINITY;
    let mut y_min: f32 = INFINITY;
    let mut y_max: f32 = -INFINITY;
    let mut z_min: f32 = INFINITY;
    let mut z_max: f32 = -INFINITY;

    for file_point in file_points {
        let mut point = MyPoint{x:0.0, y:0.0, z:0.0, index, chunk: 0};

        for (i, field) in file_point.0.iter().enumerate() {
            // let val = field.to_value::<f32>();
            // println!("{:?}", val);
            let val = field_to_value(field);
            if i == xyz_indexes.x as usize {
                point.x = val;
                x_min = x_min.min(val);
                x_max = x_max.max(val);
            } else if i == xyz_indexes.y as usize {
                point.y = val;
                y_min = y_min.min(val);
                y_max = y_max.max(val);
            } else if i == xyz_indexes.z as usize {
                point.z = val;
                z_min = z_min.min(val);
                z_max = z_max.max(val);
            }
        }
        // println!("{:?}", point);
        points.push(point);
        index += 1;
    }

    println!("{} {} {} {} {} {}", x_min, y_min, z_min, x_max, y_max, z_max);
    set_points_chunks(&mut points, [x_min, x_max, y_min, y_max, z_min, z_max]);

    println!("{:?}", points);
    Ok(points)
}

fn field_to_value(field: &Field) -> f32 {
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

fn set_points_chunks(points: &mut Vec<MyPoint>, borders: [f32; 6]) {
    let [x_divisions, y_divisions, z_divisions] = get_divisions(borders);
    // println!("{:?}", x_divisions);
    for point in points.iter_mut() {
        let chunk_x = get_xyz_index(point.x, &x_divisions) as u32;
        let chunk_y = get_xyz_index(point.y, &y_divisions) as u32;
        let chunk_z = get_xyz_index(point.z, &z_divisions) as u32;
        // println!("{} {} {}", chunk_x, chunk_y, chunk_z)
        let chunk_num = SPLIT_NUM.pow(2) * chunk_x + SPLIT_NUM * chunk_y + chunk_z;
        point.chunk = chunk_num;
    }
}

fn get_xyz_index(point_val: f32, divisions: &Vec<f32>) -> usize {
    let mut chunk_index = 0;
    for div in divisions {
        if point_val <= *div {
            break;
        } else if point_val > *div {
            chunk_index += 1
        }
    }
    return chunk_index;
}

fn get_divisions(borders: [f32; 6]) -> [Vec<f32>; 3] {
    let x_step = (borders[1] - borders[0]) / SPLIT_NUM as f32;
    let y_step = (borders[3] - borders[2]) / SPLIT_NUM as f32;
    let z_step = (borders[5] - borders[4]) / SPLIT_NUM as f32;
    let mut x_divisions: Vec<f32> = Vec::new();
    let mut y_divisions: Vec<f32> = Vec::new();
    let mut z_divisions: Vec<f32> = Vec::new();
    for i in 0..SPLIT_NUM as i32 {
        x_divisions.push(borders[0] + x_step * (i + 1) as f32);
        y_divisions.push(borders[2] + y_step * (i + 1) as f32);
        z_divisions.push(borders[4] + z_step * (i + 1) as f32);
    };
    [x_divisions, y_divisions, z_divisions]
}
fn main() {
    let path = "./assets/bunny.pcd";
    let point_list = get_points(path).unwrap();
    // println!("{:?}", point_list);
}
