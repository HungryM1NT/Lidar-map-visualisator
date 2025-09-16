use pcd_rs::{DynReader};

fn main() {
    let path = "./assets/office1.pcd";
    let reader = DynReader::open(path).unwrap();
    let meta = reader.meta().clone();
    // println!("{:?}", points);
    for (i, field) in meta.field_defs.iter().enumerate() {
        println!(
            "  Field {}: {} ({:?}, count={})",
            i, field.name, field.kind, field.count
        );
    }
    let points: Vec<_> = reader.collect::<Result<_, _>>().unwrap();
    for point in points {
        for j in point.0 {
            let val = match j.kind() {
                pcd_rs::ValueKind::U32 => {j.to_value::<u32>().unwrap() as f32},
                pcd_rs::ValueKind::F32 => {j.to_value::<f32>().unwrap() as f32},
                _ => {0.0}
            };
            println!("{}", val)
        }
        // println!("{:?}", point);
    }
}
