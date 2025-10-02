use crate::util::{AreasWithMeta, MyPoint, get_field_from_valuekind};
use pcd_rs::{DynRecord, Schema, WriterInit, Error};
use std::collections::HashSet;


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
}

fn merge_areas(areas: Vec<Vec<MyPoint>>) -> Vec<MyPoint> {
    let mut points = HashSet::new();
    for area in areas {
        for point in area {
            points.insert(point);
        }
    }
    Vec::from_iter(points)
}
