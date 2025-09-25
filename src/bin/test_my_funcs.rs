use hakaton::point_reader::*;
use hakaton::main_file_splitter::*;

fn main() {
    // let path = "./assets/hak_big/hak_ascii.pcd";
    let path = "./assets/office_ascii.pcd";
    let pcd_data = read_and_process_pcd_file(path);
    // println!("{:?}", pcd_data.points)
    let areas = get_file_areas(&pcd_data);
    let points = get_area_points(pcd_data, &areas[0]);
    println!("{:?}", get_area_center(&points));
    // println!("{:?}", areas);
}