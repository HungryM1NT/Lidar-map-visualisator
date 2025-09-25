use hakaton::point_reader::*;
use hakaton::main_file_splitter::get_file_areas;

fn main() {
    let path = "./assets/hak_big/hak_ascii.pcd";
    let pcd_data = read_and_process_pcd_file(path);
    // println!("{:?}", pcd_data.points)
    let areas = get_file_areas(pcd_data);
    println!("{:?}", areas);
}