use crate::util::{AreasWithMeta, MyPoint};
use pcd_rs::{DataKind, DynRecord, DynWriter, Field, Schema, ValueKind, ViewPoint, WriterInit, Error};
use std::collections::HashSet;


fn get_field_from_valuekind(value: f32, value_type: ValueKind) -> Field {
    match value_type {
        ValueKind::F32 => {Field::F32(vec![value])},
        ValueKind::F64 => {Field::F64(vec![value.into()])},
        ValueKind::I8 => {Field::I8(vec![value as i8])},
        ValueKind::I16 => {Field::I16(vec![value as i16])},
        ValueKind::I32 => {Field::I32(vec![value as i32])},
        ValueKind::U8 => {Field::U8(vec![value as u8])},
        ValueKind::U16 => {Field::U16(vec![value as u16])},
        ValueKind::U32 => {Field::U32(vec![value as u32])},
    }
    // let mut fields = [Field::F32; 3];
    // for (i, value_type) in types.iter().enumerate() {
    //     match value_type {
    //         ValueKind::F32 => {fields[i] = Field::F32},
    //         ValueKind::F64 => {fields[i] = Field::F64},
    //         ValueKind::I8 => {fields[i] = Field::I8},
    //         ValueKind::I16 => {field.to_value::<i16>().unwrap() as f32},
    //         ValueKind::I32 => {field.to_value::<i32>().unwrap() as f32},
    //         ValueKind::U8 => {field.to_value::<u8>().unwrap() as f32},
    //         ValueKind::U16 => {field.to_value::<u16>().unwrap() as f32},
    //         ValueKind::U32 => {field.to_value::<u32>().unwrap() as f32},
    //     }
    // }
}

pub fn write_into_file(path: String, areas_with_meta: AreasWithMeta) -> Result<(), Error> {
    let schema = vec![
        ("x", areas_with_meta.types[0], 1),
        ("y", areas_with_meta.types[1], 1),
        ("z", areas_with_meta.types[2], 1)
    ];
    let points = merge_areas(areas_with_meta.areas);

    let mut writer = WriterInit {
        width: points.len() as u64,
        height: 1,
        viewpoint: areas_with_meta.viewpoint,
        data_kind: areas_with_meta.data_kind,
        schema: Some(Schema::from_iter(schema))
    }.create(path)?;
    for point in points {
        let record = DynRecord(vec![
            get_field_from_valuekind(point.x, areas_with_meta.types[0]),
            get_field_from_valuekind(point.y, areas_with_meta.types[1]),
            get_field_from_valuekind(point.z, areas_with_meta.types[2])
        ]);
        writer.push(&record)?;
    }
    writer.finish()?;

    Ok(())
    
    // let mut writer = WriterInit {
    //     wid
    // }
}
// -> Vec<DynRecord>
fn merge_areas(areas: Vec<Vec<MyPoint>>) -> Vec<MyPoint> {
    let mut points = HashSet::new();
    for area in areas {
        for point in area {
            // let record = DynRecord(vec![
            //     get_field_from_valuekind(point.x, types[0]),
            //     get_field_from_valuekind(point.y, types[1]),
            //     get_field_from_valuekind(point.z, types[2])
            // ]);
            points.insert(point);
        }
    }
    Vec::from_iter(points)
}

fn main() {
    let test = Field::F32;
    let test2 = DynRecord(vec![
        test(vec![123.0])
    ]);
    // let test3 = match {
        
    // }
}