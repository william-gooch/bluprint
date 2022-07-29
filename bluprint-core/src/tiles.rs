use crate::utils::IntoIterator3D;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    North,
    East,
    South,
    West,
}

#[derive(Serialize, Deserialize)]
pub struct Facing(Direction, Direction);

pub mod face {
    use super::Direction::*;
    use super::Facing;

    pub const UP_NORTH: Facing = Facing(Up, North);
    pub const UP_EAST: Facing = Facing(Up, East);
    pub const UP_SOUTH: Facing = Facing(Up, South);
    pub const UP_WEST: Facing = Facing(Up, West);

    pub const DOWN_NORTH: Facing = Facing(Down, North);
    pub const DOWN_EAST: Facing = Facing(Down, East);
    pub const DOWN_SOUTH: Facing = Facing(Down, South);
    pub const DOWN_WEST: Facing = Facing(Down, West);

    pub const NORTH_EAST: Facing = Facing(North, East);
    pub const NORTH_WEST: Facing = Facing(North, West);
    pub const SOUTH_EAST: Facing = Facing(South, East);
    pub const SOUTH_WEST: Facing = Facing(South, West);
}

#[derive(Serialize, Deserialize)]
pub struct TileType {
    pub id: usize,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TileSet {
    tiles: HashMap<usize, TileType>,
}

impl TileSet {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }

    pub fn get_tile(&self, id: usize) -> Option<&TileType> {
        self.tiles.get(&id)
    }

    pub fn add_tile(&mut self, id: usize, tile: TileType) {
        self.tiles.insert(id, tile);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    pub tile_id: usize,
    // facing: Facing,
}

pub const CHUNK_SIZE_X: usize = 16;
pub const CHUNK_SIZE_Y: usize = 64;
pub const CHUNK_SIZE_Z: usize = 16;

#[derive(Serialize, Deserialize)]
pub struct TileChunk {
    #[serde(flatten)]
    tiles: Vec<Vec<Vec<Option<Tile>>>>,
}

impl TileChunk {
    pub fn new() -> Self {
        Self {
            tiles: vec![vec![vec![None; CHUNK_SIZE_X]; CHUNK_SIZE_Y]; CHUNK_SIZE_Z],
        }
    }

    pub fn get_tile(&self, x: usize, y: usize, z: usize) -> &Option<Tile> {
        &self.tiles[z][y][x]
    }

    pub fn get_tile_mut(&mut self, x: usize, y: usize, z: usize) -> &mut Option<Tile> {
        &mut self.tiles[z][y][x]
    }
}

#[derive(Serialize, Deserialize)]
pub struct TileMap {
    #[serde(skip)]
    #[serde(default = "crate::example::tileset")]
    tile_set: TileSet,
    chunks: HashMap<(i32, i32, i32), TileChunk>,
}

impl TileMap {
    pub fn new(tile_set: TileSet) -> Self {
        Self {
            tile_set,
            chunks: HashMap::new(),
        }
    }

    pub fn get_chunk(&self, cx: i32, cy: i32, cz: i32) -> Option<&TileChunk> {
        self.chunks.get(&(cx, cy, cz))
    }

    pub fn get_tile_at(&self, x: i32, y: i32, z: i32) -> Option<&Option<Tile>> {
        let cpos = {
            let cx = x / CHUNK_SIZE_X as i32;
            let cy = y / CHUNK_SIZE_Y as i32;
            let cz = z / CHUNK_SIZE_Z as i32;

            (cx, cy, cz)
        };

        let chunk = self.chunks.get(&cpos)?;

        let x = (x % CHUNK_SIZE_X as i32) as usize;
        let y = (y % CHUNK_SIZE_Y as i32) as usize;
        let z = (z % CHUNK_SIZE_Z as i32) as usize;

        Some(chunk.get_tile(x, y, z))
    }

    pub fn get_tile_at_or_create_chunk(&mut self, x: i32, y: i32, z: i32) -> &Option<Tile> {
        let cpos = {
            let cx = x / CHUNK_SIZE_X as i32;
            let cy = y / CHUNK_SIZE_Y as i32;
            let cz = z / CHUNK_SIZE_Z as i32;

            (cx, cy, cz)
        };

        let chunk = self.chunks.entry(cpos).or_insert(TileChunk::new());

        let x = (x % CHUNK_SIZE_X as i32) as usize;
        let y = (y % CHUNK_SIZE_Y as i32) as usize;
        let z = (z % CHUNK_SIZE_Z as i32) as usize;

        chunk.get_tile(x, y, z)
    }

    pub fn set_tile_at(&mut self, pos: (i32, i32, i32), tile: Option<Tile>) {
        let cpos = {
            let cx = pos.0 / CHUNK_SIZE_X as i32;
            let cy = pos.1 / CHUNK_SIZE_Y as i32;
            let cz = pos.2 / CHUNK_SIZE_Z as i32;

            (cx, cy, cz)
        };

        let chunk = self.chunks.entry(cpos).or_insert(TileChunk::new());

        let x = (pos.0 % CHUNK_SIZE_X as i32) as usize;
        let y = (pos.1 % CHUNK_SIZE_Y as i32) as usize;
        let z = (pos.2 % CHUNK_SIZE_Z as i32) as usize;

        *chunk.get_tile_mut(x, y, z) = tile;
    }

    pub fn set_tile_range(
        &mut self,
        range: std::ops::RangeInclusive<(i32, i32, i32)>,
        tile: Option<Tile>,
    ) {
        let cpos_start = {
            let cx = range.start().0.div_floor(CHUNK_SIZE_X as i32);
            let cy = range.start().1.div_floor(CHUNK_SIZE_Y as i32);
            let cz = range.start().2.div_floor(CHUNK_SIZE_Z as i32);

            (cx, cy, cz)
        };

        let cpos_end = {
            let cx = range.end().0.div_floor(CHUNK_SIZE_X as i32);
            let cy = range.end().1.div_floor(CHUNK_SIZE_Y as i32);
            let cz = range.end().2.div_floor(CHUNK_SIZE_Z as i32);

            (cx, cz, cy)
        };

        for (cx, cy, cz) in (cpos_start..=cpos_end).into_3d_iter() {
            let chunk = self.chunks.entry((cx, cy, cz)).or_insert(TileChunk::new());

            let pos_start = {
                let x = if range.start().0 < (cx * CHUNK_SIZE_X as i32) {
                    0
                } else {
                    range.start().0.rem_euclid(CHUNK_SIZE_X as i32)
                } as usize;
                let y = if range.start().1 < (cy * CHUNK_SIZE_Y as i32) {
                    0
                } else {
                    range.start().1.rem_euclid(CHUNK_SIZE_Y as i32)
                } as usize;
                let z = if range.start().2 < (cz * CHUNK_SIZE_Z as i32) {
                    0
                } else {
                    range.start().2.rem_euclid(CHUNK_SIZE_Z as i32)
                } as usize;

                (x, y, z)
            };

            let pos_end = {
                let x = if range.end().0 >= ((cx + 1) * CHUNK_SIZE_X as i32) {
                    CHUNK_SIZE_X as i32 - 1
                } else {
                    range.end().0.rem_euclid(CHUNK_SIZE_X as i32)
                } as usize;
                let y = if range.end().1 >= ((cy + 1) * CHUNK_SIZE_Y as i32) {
                    CHUNK_SIZE_Y as i32 - 1
                } else {
                    range.end().1.rem_euclid(CHUNK_SIZE_Y as i32)
                } as usize;
                let z = if range.end().2 >= ((cz + 1) * CHUNK_SIZE_Z as i32) {
                    CHUNK_SIZE_Z as i32 - 1
                } else {
                    range.end().2.rem_euclid(CHUNK_SIZE_Z as i32)
                } as usize;

                (x, y, z)
            };

            for (x, y, z) in (pos_start..=pos_end).into_3d_iter() {
                *chunk.get_tile_mut(x, y, z) = tile;
            }
        }
    }
}
