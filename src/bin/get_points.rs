use pcd_rs::{DynReader, Field, PcdMeta, ValueKind};


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

    for file_point in file_points {
        let mut point = MyPoint{x:0.0, y:0.0, z:0.0};

        for (i, field) in file_point.0.iter().enumerate() {
            // let val = field.to_value::<f32>();
            // println!("{:?}", val);
            let val = field_to_value(field);
            if i == xyz_indexes.x as usize {
                point.x = val;
            } else if i == xyz_indexes.y as usize {
                point.y = val;
            } else if i == xyz_indexes.z as usize {
                point.z = val;
            }
        }
        // println!("{:?}", point);
        points.push(point);
    }
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

fn main() {
    let path = "./assets/office1.pcd";
    let point_list = get_points(path).unwrap();
    println!("{:?}", point_list);
}
