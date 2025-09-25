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

pub fn get_area_center(area_points: &Vec<MyPoint>) -> [f32; 3] {
    let mut x_sum = 0.;
    let mut y_sum = 0.;
    let mut z_sum = 0.;
    let n = area_points.len() as f32;
    for point in area_points {
        x_sum += point.x;
        y_sum += point.y;
        z_sum += point.z;
    }
    [x_sum / n, y_sum / n, z_sum / n]
}

// fn main() {
    
// }