use std::{any::Any, f32::{INFINITY, NAN}};

use pcd_rs::{DynReader, Field, PcdMeta, ValueKind};
use crate::util::*;

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

fn read_file(path: &str) -> Result<PCDData, String> {
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

    'next_point:
    for file_point in file_points {
        let mut point = MyPoint{x:0.0, y:0.0, z:0.0, index, chunk_x_index: 0, chunk_y_index: 0, box_index: 0};

        for (i, field) in file_point.0.iter().enumerate() {
            // let val = field.to_value::<f32>();
            // println!("{:?}", val);
            let val = field_to_value(field);
            if val.is_nan() {
                continue 'next_point;
            }
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
    Ok(PCDData { points, x_min, x_max, y_min, y_max, z_min, z_max, chunks_in_one_row: 1 })
}

pub fn read_and_process_pcd_file(path: &str) -> Vec<Vec<MyPoint>> {
    let mut pcd_data = read_file(path).unwrap();
    split_to_chunks(&mut pcd_data)


    // println!("{} {} {} {} {} {}", x_min, y_min, z_min, x_max, y_max, z_max);
    // set_points_chunks(&mut points, [x_min, x_max, y_min, y_max, z_min, z_max]);

    // println!("{:?}", points);
    // pcd_data
}

fn split_to_chunks(pcd_data: &mut PCDData) -> Vec<Vec<MyPoint>> {
    let points_num = pcd_data.points.len() as u32;
    let chunks_num = points_num / POINTS_IN_ONE_CHUNK;
    let chunks_in_one_row = (chunks_num as f32).sqrt() as u32 + 1;

    let mut chunk_splitter_value = ChunkSplitter::Try(chunks_in_one_row);
    // pcd_data.chunks_in_one_row = chunks_in_one_row;
    
    println!("{:?}", chunk_splitter_value);
    'chunk_splitter:
    while let ChunkSplitter::Try(chunks_in_one_row) = chunk_splitter_value {
        let x_divisions = divide_by_n(pcd_data.x_min, pcd_data.x_max, chunks_in_one_row);
        let y_divisions = divide_by_n(pcd_data.y_min, pcd_data.y_max, chunks_in_one_row);
    
        let mut chunks: Vec<Vec<Vec<MyPoint>>> = vec![vec![vec![]; chunks_in_one_row as usize]; chunks_in_one_row as usize];
        // println!("{}", chunks.len());
        for point in pcd_data.points.iter_mut() {
            let x_index = get_split_index(point.x, &x_divisions);
            let y_index  = get_split_index(point.y, &y_divisions);
            point.chunk_x_index = x_index;
            point.chunk_y_index = y_index;
            // println!("{} {} {}", x_index, y_index, chunks_in_one_row);
            // println!("{:?} {}", x_divisions, point.x);
            chunks[x_index as usize][y_index as usize].push(point.clone());
            // println!("321");
            // println!("{:?}", point)
        }
        
        if chunks_in_one_row == 1 {
            return vec![chunks[0][0].clone()];
        }
        
        let mut areas: Vec<Vec<MyPoint>> = Vec::new();
        for x_haha in 0..chunks_in_one_row - 1 {
            println!("213123");
            for y_haha in 0..chunks_in_one_row - 1 {
                let mut area: Vec<MyPoint> = vec![];
                area.append(&mut chunks[x_haha as usize][y_haha as usize]);
                area.append(&mut chunks[(x_haha + 1) as usize][y_haha as usize]);
                area.append(&mut chunks[x_haha as usize][(y_haha + 1) as usize]);
                area.append(&mut chunks[(x_haha + 1) as usize][(y_haha + 1) as usize]);
                // println!("{}", area.len())
                if area.len() as u32 > POINTS_IN_ONE_CHUNK * 4 {
                    chunk_splitter_value = ChunkSplitter::Try((chunks_in_one_row as f32 * 1.2) as u32 + 1);
                    continue 'chunk_splitter;
                }

                // TODO: Убрать
                if area.len() != 0 {
                    areas.push(area)
                }
            }
        }
        chunk_splitter_value = ChunkSplitter::Ok(areas);
        
        
    }
    // if let ChunkSplitter::Ok(areas) = chunk_splitter_value {
    //     println!("{}", areas.len());
    //     // for area in areas {
    //     //     println!("{}", area.len())
    //     // }
    // }
    chunk_splitter_value.unwrap()

    // println!("{:?}", chunks);
    // let temp_x = (pcd_data.x_max - pcd_data.x_min) / chunks_in_one_row as f32 * 2. + pcd_data.x_min;
    // let temp_y = (pcd_data.y_max - pcd_data.y_min) / chunks_in_one_row as f32 * 2. + pcd_data.y_min;
    // println!("{} ", chunks_num);
    // println!("{} {} {} {}", pcd_data.x_min, temp_x, pcd_data.y_min, temp_y);
    // println!("{} {}", pcd_data.z_min, pcd_data.z_max);


    // let [x_divisions, y_divisions] = get_chunk_divisions([PCDData.x_min, PCDData.x_max, PCDData.y_min, PCDData.y_max]);
    
}

fn divide_by_n(min_value: f32, max_value: f32, n: u32) -> Vec<f32> {
    if n == 1 {
        return vec![max_value]
    }
    let step = (max_value - min_value) / n as f32;
    let mut divisions: Vec<f32> = Vec::new();
    for i in 0..n-1 {
        divisions.push(min_value + step * (i + 1) as f32)
    }
    divisions.push(max_value);
    divisions
}


// fn set_points_chunks(borders: [f32; 6]) {
//     let [x_divisions, y_divisions, z_divisions] = get_divisions(borders);
//     // println!("{:?}", x_divisions);
//     for point in points.iter_mut() {
//         let chunk_x = get_xyz_index(point.x, &x_divisions) as u32;
//         let chunk_y = get_xyz_index(point.y, &y_divisions) as u32;
//         let chunk_z = get_xyz_index(point.z, &z_divisions) as u32;
//         // println!("{} {} {}", chunk_x, chunk_y, chunk_z)
//         let chunk_num = SPLIT_NUM.pow(2) * chunk_x + SPLIT_NUM * chunk_y + chunk_z;
//         point.chunk = chunk_num;
//     }
// }

fn get_split_index(point_val: f32, divisions: &Vec<f32>) -> u32 {
    let mut chunk_index = 0;
    for div in divisions {
        if point_val <= *div {
            break;
        } else {
            chunk_index += 1
        }
    }
    return chunk_index as u32;
}

// fn main() {
//     let path = "./assets/hak_big/hak_ascii.pcd";
//     let point_list = read_and_process_pcd_file(path);
//     // println!("{:?}", point_list);
// }
