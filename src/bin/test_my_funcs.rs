use hakaton::point_reader::*;
use hakaton::main_file_splitter::*;

fn main() {
    let path = "./assets/hak_big/hak_ascii.pcd";
    // let path = "./assets/bunny.pcd";
    let pcd_data = read_and_process_pcd_file(path);
    // println!("{:?}", pcd_data.points)
    // let areas = get_file_areas(&pcd_data);
    // println!("{:?}", areas);
    // let points = get_area_points(pcd_data, &areas[20]);
    // println!("{:?}", get_area_center(&points));
    // println!("{}", points.len());
    // for point in pcd_data.points {
    //     println!("{:?}", point)
    // }
    // println!("{:?}", areas);
}