use iyes_loopless::prelude::*;
use bevy::pbr::wireframe::{WireframePlugin, Wireframe};
use bevy::utils::HashSet;
use bluprint_core::utils::IntoIterator3D;
use bluprint_core::tiles::{CHUNK_SIZE_X, CHUNK_SIZE_Y, CHUNK_SIZE_Z};
use bluprint_core::tiles::TileMap;
use bluprint_core::tiles::TileChunk;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::prelude::*;
use itertools::MultiUnzip;

#[derive(Clone, Copy)]
pub enum FaceSide {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

const X_SCALE: f32 = 1.0;
const Y_SCALE: f32 = 0.25;
const Z_SCALE: f32 = 1.0;

pub const TOP_FACE_VERTS: [([f32; 3], [f32; 3], [f32; 2]); 4] = [
    ([0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0]),
    ([0.0, 1.0, 1.0], [0.0, 1.0, 0.0], [0.0, 1.0]),
    ([1.0, 1.0, 1.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
    ([1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0]),
];

pub const BOTTOM_FACE_VERTS: [([f32; 3], [f32; 3], [f32; 2]); 4] = [
    ([0.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0]),
    ([0.0, 0.0, 1.0], [0.0, -1.0, 0.0], [0.0, 1.0]),
    ([1.0, 0.0, 1.0], [0.0, -1.0, 0.0], [1.0, 1.0]),
    ([1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [1.0, 0.0]),
];

pub const LEFT_FACE_VERTS: [([f32; 3], [f32; 3], [f32; 2]); 4] = [
    ([0.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0]),
    ([0.0, 0.0, 1.0], [-1.0, 0.0, 0.0], [0.0, 1.0]),
    ([0.0, 1.0, 1.0], [-1.0, 0.0, 0.0], [1.0, 1.0]),
    ([0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [1.0, 0.0]),
];

pub const RIGHT_FACE_VERTS: [([f32; 3], [f32; 3], [f32; 2]); 4] = [
    ([1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0]),
    ([1.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0]),
    ([1.0, 1.0, 1.0], [1.0, 0.0, 0.0], [1.0, 1.0]),
    ([1.0, 1.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0]),
];

pub const FRONT_FACE_VERTS: [([f32; 3], [f32; 3], [f32; 2]); 4] = [
    ([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0]),
    ([1.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 1.0]),
    ([1.0, 1.0, 1.0], [0.0, 0.0, 1.0], [1.0, 1.0]),
    ([0.0, 1.0, 1.0], [0.0, 0.0, 1.0], [1.0, 0.0]),
];

pub const BACK_FACE_VERTS: [([f32; 3], [f32; 3], [f32; 2]); 4] = [
    ([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], [0.0, 0.0]),
    ([1.0, 0.0, 0.0], [0.0, 0.0, -1.0], [0.0, 1.0]),
    ([1.0, 1.0, 0.0], [0.0, 0.0, -1.0], [1.0, 1.0]),
    ([0.0, 1.0, 0.0], [0.0, 0.0, -1.0], [1.0, 0.0]),
];

impl FaceSide {
    pub fn get_verts(self) -> [([f32; 3], [f32; 3], [f32; 2]); 4] {
        match self {
            FaceSide::Top => TOP_FACE_VERTS,
            FaceSide::Bottom => BOTTOM_FACE_VERTS,
            FaceSide::Left => LEFT_FACE_VERTS,
            FaceSide::Right => RIGHT_FACE_VERTS,
            FaceSide::Front => FRONT_FACE_VERTS,
            FaceSide::Back => BACK_FACE_VERTS,
        }
    }
}

struct Face {
    side: FaceSide,
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
}

impl Face {
    pub fn generate(
        side: FaceSide,
        pos_in_chunk: (f32, f32, f32),
    ) -> Face {
        let (positions, normals, uvs): (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>) =
            side.get_verts()
                .into_iter()
                .map(|(position, normal, uv)| {
                    let position = [
                        (position[0] + pos_in_chunk.0) * X_SCALE,
                        (position[1] + pos_in_chunk.1) * Y_SCALE,
                        (position[2] + pos_in_chunk.2) * Z_SCALE,
                    ];
                    (position, normal, uv)
                })
                .multiunzip();


        Face {
            side,
            positions,
            normals,
            uvs,
        }
    }
}

struct Faces {
    indices: Vec<u32>,
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
}

impl From<Vec<Face>> for Faces {
    fn from(vec: Vec<Face>) -> Self {
        let (indices, positions, normals, uvs): (Vec<[u32; 6]>, Vec<Vec<[f32; 3]>>, Vec<Vec<[f32; 3]>>, Vec<Vec<[f32; 2]>>) = vec
            .into_iter()
            .enumerate()
            .map(|(idx, face)| {
                let indices = match face.side {
                    FaceSide::Top | FaceSide::Left | FaceSide::Front => {
                        [
                            (0 + idx * 4) as u32,
                            (1 + idx * 4) as u32,
                            (2 + idx * 4) as u32,
                            (2 + idx * 4) as u32,
                            (3 + idx * 4) as u32,
                            (0 + idx * 4) as u32
                        ]
                    }
                    FaceSide::Bottom | FaceSide::Right | FaceSide::Back => {
                        [
                            (0 + idx * 4) as u32,
                            (3 + idx * 4) as u32,
                            (2 + idx * 4) as u32,
                            (2 + idx * 4) as u32,
                            (1 + idx * 4) as u32,
                            (0 + idx * 4) as u32
                        ]
                    }
                };

                (indices, face.positions, face.normals, face.uvs)
            })
            .multiunzip();

        let indices = indices
            .into_iter()
            .flatten()
            .collect();

        let positions = positions
            .into_iter()
            .flatten()
            .collect();

        let normals = normals
            .into_iter()
            .flatten()
            .collect();

        let uvs = uvs
            .into_iter()
            .flatten()
            .collect();

        Self {
            indices,
            positions,
            normals,
            uvs,
        }
    }
}

fn gen_chunk_mesh(
    chunk: &TileChunk,
    chunk_pos: (i32, i32, i32),
    tile_map: &TileMap,
) -> Mesh {
    let mut faces = Vec::new();

    for (x, y, z) in ((0, 0, 0)..(CHUNK_SIZE_X, CHUNK_SIZE_Y, CHUNK_SIZE_Z)).into_3d_iter() {
        let (fx, fy, fz) = (x as f32, y as f32, z as f32);
        if let Some(_tile) = chunk.get_tile(x, y, z) {
            println!("Generating tile");
            if x > 0 && let None = chunk.get_tile(x - 1, y, z) {
                faces.push(Face::generate(FaceSide::Left, (fx, fy, fz)));
            }
            if x < CHUNK_SIZE_X - 1 && let None = chunk.get_tile(x + 1, y, z) {
                faces.push(Face::generate(FaceSide::Right, (fx, fy, fz)));
            }
            if y > 0 && let None = chunk.get_tile(x, y - 1, z) {
                faces.push(Face::generate(FaceSide::Bottom, (fx, fy, fz)));
            }
            if y < CHUNK_SIZE_Y - 1 && let None = chunk.get_tile(x, y + 1, z) {
                faces.push(Face::generate(FaceSide::Top, (fx, fy, fz)));
            }
            if z > 0 && let None = chunk.get_tile(x, y, z - 1) {
                faces.push(Face::generate(FaceSide::Front, (fx, fy, fz)));
            }
            if z < CHUNK_SIZE_Z - 1 && let None = chunk.get_tile(x, y, z + 1) {
                faces.push(Face::generate(FaceSide::Back, (fx, fy, fz)));
            }
        }
    }

    let faces: Faces = faces.into();

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let inds = Indices::U32(faces.indices);
    mesh.set_indices(Some(inds));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, faces.positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, faces.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, faces.uvs);

    mesh
}

#[derive(Clone, Default)]
pub struct LoadedChunks(HashSet<(i32, i32, i32)>);

fn world_render(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    map: Res<TileMap>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut loaded: ResMut<LoadedChunks>,
) {
    let chunks_to_load = HashSet::from_iter(vec![(0, 0, 0)].into_iter());

    let material_handle = materials.add(StandardMaterial {
        base_color: Color::PURPLE,
        ..Default::default()
    });

    for (cx, cy, cz) in chunks_to_load.difference(&loaded.clone().0) {
        if let Some(chunk) = map.get_chunk(*cx, *cy, *cz) {
            let mesh = gen_chunk_mesh(chunk, (*cx, *cy, *cz), &*map);
            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(mesh),
                material: material_handle.clone(),
                transform: Transform::from_xyz(*cx as f32 * X_SCALE, *cy as f32 * Y_SCALE, *cz as f32 * Z_SCALE),
                ..Default::default()
            })
            .insert(Wireframe);
            loaded.0.insert((*cx, *cy, *cz));
        }
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        });
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(WireframePlugin)
            .add_startup_system(setup)
            .add_system(world_render.run_if_resource_exists::<TileMap>());
    }
}
