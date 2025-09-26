use hakaton::util::*;

fn main() {
    let my_point_1 = MyPoint{x:0.0, y:0.0, z:0.0, index: 0, chunk_x_index: 0, chunk_y_index: 0, box_index: 0};
    let my_point_2 = MyPoint{x:1.0, y:1.0, z:1.0, index: 1, chunk_x_index: 1, chunk_y_index: 1, box_index: 1};

    let points = vec![my_point_1, my_point_2];
    let pcd_data = PCDData{points, x_min: 0., x_max: 0., y_min: 0., y_max: 0., z_min: 0., z_max: 0., chunks_in_one_row: 1};
    
    let mut chunk_1:Vec<&MyPoint> = vec![];
    let mut chunk_2: Vec<&MyPoint> = vec![];
    let mut chunk_3: Vec<&MyPoint> = vec![];
    for point in & pcd_data.points {
        if point.x == 0. {
            chunk_1.push(point)
        } else {
            chunk_2.push(point)
        }
        chunk_3.push(point)
    }
    // println!("{:?}", chunk_1);
    // println!("{:?}", chunk_2);
    // println!("{:?}", chunk_3);
    // pcd_data.points.retain(|point| point.index != 0);

    println!("{:?}", chunk_1);
    println!("{:?}", chunk_2);
    println!("{:?}", chunk_3);
}