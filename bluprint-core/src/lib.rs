#![feature(int_roundings)]
pub mod tiles;
pub mod utils;

pub mod example {
    use crate::tiles::*;

    pub fn tileset() -> TileSet {
        let mut tileset = TileSet::new();

        tileset.add_tile(
            0,
            TileType {
                id: 0,
                name: "Floor".to_owned(),
            },
        );

        tileset
    }

    pub fn tilemap() -> TileMap {
        let mut tilemap = TileMap::new(tileset());

        tilemap.set_tile_range((0, 0, 0)..=(15, 0, 15), Some(Tile { tile_id: 0 }));
        tilemap.set_tile_range((1, 1, 1)..=(15, 1, 15), Some(Tile { tile_id: 0 }));
        tilemap.set_tile_range((2, 2, 2)..=(15, 2, 15), Some(Tile { tile_id: 0 }));
        tilemap.set_tile_range((3, 3, 3)..=(15, 3, 15), Some(Tile { tile_id: 0 }));

        tilemap
    }
}
