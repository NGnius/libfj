use genmesh::{generators::Cube, Quad, MapToVertices, Vertices};
use obj;
use crate::robocraft;

/// Convert a Robocraft robot to a 3D model in Wavefront OBJ format.
pub fn cubes_to_model(robot: robocraft::Cubes) -> obj::Obj {
    let mut positions = Vec::<[f32; 3]>::new();
    let mut normals = Vec::<[f32; 3]>::new();
    let mut objects = Vec::<obj::Object>::new();
    let mut last = 0;
    for cube in robot.into_iter() {
        positions.extend::<Vec::<[f32; 3]>>(
            Cube::new().vertex(|v|
                [(v.pos.x * 0.5) + (cube.x as f32), (v.pos.y * 0.5) + (cube.y as f32), (v.pos.z * 0.5) + (cube.z as f32)])
            .vertices()
            .collect()
        );
        normals.extend::<Vec::<[f32; 3]>>(
            Cube::new().vertex(|v|
                [(v.normal.x * 0.5) + (cube.x as f32), (v.normal.y * 0.5) + (cube.y as f32), (v.normal.z * 0.5) + (cube.z as f32)])
            .vertices()
            .collect()
        );
        let polys = Cube::new().vertex(|_| {last+=1; return last-1;})
            .map(|Quad{x: v0, y: v1, z: v2, w: v3}|
                obj::SimplePolygon(vec![
                obj::IndexTuple(v0, Some(0), Some(v0)),
                obj::IndexTuple(v1, Some(0), Some(v1)),
                obj::IndexTuple(v2, Some(0), Some(v2)),
                obj::IndexTuple(v3, Some(0), Some(v3))
                ])
                /*obj::SimplePolygon(vec![
                obj::IndexTuple(v0, None, None),
                obj::IndexTuple(v1, None, None),
                obj::IndexTuple(v2, None, None),
                obj::IndexTuple(v3, None, None)
                ])*/
            ).collect();
        objects.push(
            obj::Object{
                name: format!("Cube-ID{}-NUM{}", cube.id, objects.len()),
                groups: vec![
                    obj::Group {
                        name: format!("Cube-ID{}-NUM{}-0", cube.id, objects.len()),
                        index: 0,
                        material: None,
                        polys: polys
                    },
                ]
            }
        );
    }
    println!("Last (index): {}, Vertices (len): {}", last, positions.len());

    obj::Obj{
        data: obj::ObjData {
            position: positions,
            texture: vec![[0.0, 0.0]],
            normal: normals,
            objects: objects,
            material_libs: Vec::new(),
        },
        path: std::path::PathBuf::new(),
    }
}
