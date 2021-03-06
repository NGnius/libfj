use genmesh::{generators::Cube, Quad, MapToVertices, Vertices, Vertex};
use obj;
use cgmath::{Quaternion, Euler, Deg, Vector3};
use crate::robocraft;

const SCALE: f32 = 0.5;

const ROTATIONS: [Euler<Deg<f32>>; 24] = [
    Euler{x: Deg(0.0), y: Deg(0.0), z: Deg(0.0)}, // 0
    Euler{x: Deg(0.0), y: Deg(0.0), z: Deg(90.0)},
    Euler{x: Deg(0.0), y: Deg(0.0), z: Deg(180.0)}, // 2
    Euler{x: Deg(0.0), y: Deg(0.0), z: Deg(-90.0)},
    Euler{x: Deg(0.0), y: Deg(90.0), z: Deg(0.0)}, // 4
    Euler{x: Deg(0.0), y: Deg(90.0), z: Deg(90.0)},
    Euler{x: Deg(-90.0), y: Deg(-90.0), z: Deg(0.0)}, // 6
    Euler{x: Deg(0.0), y: Deg(90.0), z: Deg(-90.0)},
    Euler{x: Deg(0.0), y: Deg(-90.0), z: Deg(90.0)}, // 8
    Euler{x: Deg(0.0), y: Deg(-90.0), z: Deg(-90.0)},
    Euler{x: Deg(90.0), y: Deg(-90.0), z: Deg(0.0)}, // 10
    Euler{x: Deg(90.0), y: Deg(90.0), z: Deg(0.0)},
    Euler{x: Deg(-90.0), y: Deg(90.0), z: Deg(0.0)}, // 12
    Euler{x: Deg(0.0), y: Deg(90.0), z: Deg(180.0)},
    Euler{x: Deg(0.0), y: Deg(180.0), z: Deg(0.0)}, // 14
    Euler{x: Deg(0.0), y: Deg(180.0), z: Deg(90.0)},
    Euler{x: Deg(0.0), y: Deg(180.0), z: Deg(0.0)}, // 16
    Euler{x: Deg(0.0), y: Deg(180.0), z: Deg(180.0)},
    Euler{x: Deg(0.0), y: Deg(-90.0), z: Deg(-90.0)}, // 18
    Euler{x: Deg(0.0), y: Deg(-90.0), z: Deg(0.0)},
    Euler{x: Deg(90.0), y: Deg(0.0), z: Deg(180.0)}, // 20
    Euler{x: Deg(90.0), y: Deg(180.0), z: Deg(0.0)},
    Euler{x: Deg(-90.0), y: Deg(0.0), z: Deg(0.0)}, // 22
    Euler{x: Deg(-90.0), y: Deg(180.0), z: Deg(0.0)}, // 23
];

/// Convert a Robocraft robot's orientation enum into a physical rotation
pub fn cube_rotation_to_quat(orientation: u8) -> Quaternion<f32> {
    ROTATIONS[orientation as usize].into()
}

/// Convert a Robocraft robot to a 3D model in Wavefront OBJ format.
pub fn cubes_to_model(robot: robocraft::Cubes) -> obj::Obj {
    cubes_to_model_with_lut(robot, default_model_lut)
}

/// Convert a Robocraft robot to a 3D model in Wavefront OBJ format using the provided lookup table function.
pub fn cubes_to_model_with_lut<F: FnMut(u32) -> Vec<Quad<Vertex>>>(robot: robocraft::Cubes, mut lut: F) -> obj::Obj {
    let mut positions = Vec::<[f32; 3]>::new(); // vertex positions
    let mut normals = Vec::<[f32; 3]>::new(); // vertex normals
    let mut objects = Vec::<obj::Object>::new(); // blocks
    let mut last = 0;
    for cube in robot.into_iter() {
        // generate simple cube for every block
        // TODO rotate blocks
        let vertices = lut(cube.id); // Use lookup table to find correct id <-> block translation
        let rotation: Quaternion<_> = cube_rotation_to_quat(cube.orientation);
        positions.extend::<Vec::<[f32; 3]>>(
            vertices.clone().into_iter().vertex(|v|
                {
                    let rotated = rotation * Vector3{x: v.pos.x * SCALE, y: v.pos.y * SCALE, z: v.pos.z * SCALE};
                    [rotated.x + (cube.x as f32), rotated.y + (cube.y as f32), rotated.z + (cube.z as f32)]
                })
            .vertices()
            .collect()
        );
        normals.extend::<Vec::<[f32; 3]>>(
            vertices.clone().into_iter().vertex(|v|
                {
                    let rotated = rotation * Vector3{x: v.normal.x * SCALE, y: v.normal.y * SCALE, z: v.normal.z * SCALE};
                    [rotated.x + (cube.x as f32), rotated.y + (cube.y as f32), rotated.z + (cube.z as f32)]
                })
            .vertices()
            .collect()
        );
        let polys = vertices.clone().into_iter().vertex(|_| {last+=1; return last-1;})
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

pub fn default_model_lut(id: u32) -> Vec<Quad<Vertex>> {
    // TODO generate non-cube blocks properly
    match id {
        _ => Cube::new().collect(),
    }
}
