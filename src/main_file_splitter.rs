use std::f32::INFINITY;

use bevy::math::Vec3;

use crate::util::*;

// All areas contain 2x2 chunks
#[derive(Debug)]
pub struct Area {
    start_x: u32, 
    start_y: u32,
    index: u32
}

pub fn get_file_areas(pcd_data: &PCDData) -> Vec<Area> {
    match pcd_data.chunks_in_one_row {
        1..=2 => {vec![Area{ start_x: 0, start_y: 0, index: 0}]},
        _ => {
            let mut area_index = 0;
            let n = pcd_data.chunks_in_one_row - 1;
            let mut areas = Vec::new();

            for x in 0..n {
                for y in 0..n {
                    if is_area_usefull(&pcd_data, x, y) {
                        areas.push(Area{
                            start_x: x,
                            start_y: y,
                            index: area_index
                        });
                        area_index += 1;
                    }
                }
            }
            
            areas
        }
    }
}

fn is_area_usefull(pcd_data: &PCDData, area_start_x: u32, area_start_y: u32) -> bool {
    for point in pcd_data.points.iter() {
        if ((point.chunk_x_index == area_start_x) || (point.chunk_x_index - 1 == area_start_x)) &&
            ((point.chunk_y_index == area_start_y) || (point.chunk_y_index - 1 == area_start_y)) {
                return true;
            }
    }
    return false;
}

pub fn get_area_points(pcd_data: PCDData, area: &Area) -> Vec<MyPoint> {
    let mut area_points: Vec<MyPoint> = Vec::new();

    for point in pcd_data.points {
        if ((point.chunk_x_index == area.start_x) || (point.chunk_x_index - 1 == area.start_x)) &&
            ((point.chunk_y_index == area.start_y) || (point.chunk_y_index - 1 == area.start_y)) {

                area_points.push(point);
            }
    }
    area_points
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
    // [x_sum / n, y_sum / n, z_sum / n]
    Vec3{x: x_mid, y: y_mid, z: z_mid}
}

// fn main() {
    
// }