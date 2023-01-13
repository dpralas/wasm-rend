use std::io::{self, BufRead, BufReader, Read, Seek, Write};

use byteorder::{LittleEndian, ReadBytesExt};
use fxhash::FxHashMap;
use glam::Vec3;

use crate::model::mesh::{Mesh, MeshType};

const HEADER_SIZE: usize = 80;

pub enum StlType {
    ASCII,
    BINARY,
}

/// Check if stl is binary or ascii format by looking at first bytes
fn is_ascii_stl<F: Read + Seek>(read: &mut F) -> io::Result<bool> {
    let mut header = String::new();
    let maybe_read_error = BufReader::new(&mut *read).read_line(&mut header);
    // Try to seek back to start before evaluating potential read errors.
    read.seek(::std::io::SeekFrom::Start(0))?;

    if maybe_read_error.is_err() {
        return Ok(false);
    }
    if !header.starts_with("solid ") {
        Ok(false)
    } else {
        Ok(true)
    }
}

impl Mesh {
    pub fn from_stl<R: Read + Seek>(input: R) -> io::Result<Self> {
        let mut input = BufReader::new(input);

        let stl_type = is_ascii_stl(&mut input)?;

        if stl_type {
            return Mesh::read_stl_ascii(&mut input);
        }

        Mesh::read_stl_binary::<_>(input)
    }

    pub fn read_stl_ascii<R: Read + Seek>(mut input: R) -> io::Result<Self> {
        let mesh: stl_io::IndexedMesh = stl_io::read_stl(&mut input)?;

        let vertices: Vec<Vec3> = mesh
            .vertices
            .iter()
            .map(|v| Vec3::new(v[0], v[1], v[2]))
            .collect();

        let faces: Vec<[usize; 3]> =
            mesh.faces.iter().map(|f| f.vertices).collect();

        let face_normals: Vec<Vec3> = mesh
            .faces
            .iter()
            .map(|f| f.normal)
            .map(|n| Vec3::new(n[0], n[1], n[2]).normalize())
            .collect();

        let mut vertex_normals: Vec<Vec3> =
            vec![Vec3::new(0.0, 0.0, 0.0); vertices.len()];

        for (face_index, face) in faces.iter().enumerate() {
            for vertice_index in face {
                vertex_normals[*vertice_index] += face_normals[face_index];
            }
        }
        for vertex_normal in &mut vertex_normals {
            *vertex_normal = vertex_normal.normalize();
        }

        let mut mesh = Mesh {
            vertices,
            vertex_normals,
            face_normals,
            faces,
            convex_hull: None,
            max: Vec3::splat(0.0),
            min: Vec3::splat(0.0),
            settle_transform: None,
            mesh_type: MeshType::Model,
        };
        mesh.recalculate_min_and_max();
        Ok(mesh)
    }

    pub fn read_stl_binary<R: Read + Seek + ReadBytesExt>(
        mut input: R,
    ) -> io::Result<Self> {
        input.seek(::std::io::SeekFrom::Start(HEADER_SIZE as u64))?;

        let triangles = input.read_u32::<LittleEndian>()? as usize;
        let mut points = FxHashMap::default();
        let mut face_normals = Vec::with_capacity(triangles);
        let mut faces = Vec::with_capacity(triangles);
        let mut vertices = Vec::new();
        let mut vertex_normals: Vec<Vec3> = Vec::new();

        let mut tri_buf = [0f32; 12];
        let mut point_index = 0;
        for i in 0..triangles {
            input.read_f32_into::<LittleEndian>(&mut tri_buf)?;
            for elem in &tri_buf {
                if elem.is_nan() || elem.is_infinite() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "STL file is corrupted",
                    ));
                }
            }
            // add face normal
            let normal = Vec3::from([tri_buf[0], tri_buf[1], tri_buf[2]]);
            face_normals.push(normal);

            // push new face with indexes to points
            faces.push([usize::MAX, usize::MAX, usize::MAX]);
            for point in tri_buf[3..].chunks_exact(3) {
                // create point
                let point_bits = [
                    point[0].to_bits(),
                    point[1].to_bits(),
                    point[2].to_bits(),
                ];
                let face_point_index: usize;
                // check if point already exists
                if let Some(index) = points.get_mut(&point_bits) {
                    // val.push(Vertex::from(normal));
                    face_point_index = *index;
                    // add normal to vertex_normal
                    vertex_normals[*index] += normal;
                } else {
                    // add vertex to vertices
                    vertices.push(Vec3::from([point[0], point[1], point[2]]));
                    // add normal to vertex_normals
                    vertex_normals.push(normal);
                    // insert vertex to hashmap
                    points.insert(point_bits, point_index);
                    face_point_index = point_index;
                    point_index += 1;
                }
                // add point index to current face
                for point_i in 0..3 {
                    if faces[i][point_i] == usize::MAX {
                        faces[i][point_i] = face_point_index;
                        break;
                    }
                }
            }
            input.read_u16::<LittleEndian>()?;
        }
        for normal in &mut vertex_normals {
            *normal = normal.normalize();
        }
        let mut mesh = Mesh {
            vertices,
            vertex_normals,
            face_normals,
            faces,
            convex_hull: None,
            max: Vec3::splat(0.0),
            min: Vec3::splat(0.0),
            settle_transform: None,
            mesh_type: MeshType::Model,
        };
        mesh.recalculate_min_and_max();
        Ok(mesh)
    }
}
