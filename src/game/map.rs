
use {
    crate::{
        alg::{V2, Shape},
        gfx::{load_atlas_texture},
    },
    std::{
        error::Error
    },
    gl::types::*,
};

#[derive(Debug)]
pub enum LoadMapError {
    MainLayerMissing,
    TooManyTilesets,
    TooManyImages,
    Nested(Box<dyn Error>),
}

impl LoadMapError {
    fn nest(inner: impl Into<Box<dyn Error>>) -> LoadMapError {
        LoadMapError::Nested(inner.into())
    }
}

impl std::fmt::Display for LoadMapError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for LoadMapError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LoadMapError::Nested(e) => Some(e.as_ref() as _),
            _                       => None
        }
    }
}

pub struct Tile {
    collider: Shape
}

pub struct Tileset {
    atlas_texture: GLuint,
    base_gid:      u32,
    max_id:        u32,
    tiles:         Vec<(u32, Tile)>
}

impl std::ops::Index<u32> for Tileset {
    type Output = Tile;
    fn index(&self, id: u32) -> &Tile {
        self.tiles.iter()
            .find(|(tid, _)| *tid == id)
            .map(|(_, tile)| tile)
            .unwrap()
    }
}

impl Tileset {
    pub fn texture(&self) -> GLuint {
        self.atlas_texture
    }

    fn load(ts: &tiled::Tileset) -> Result<Tileset, LoadMapError> {
        let (atlas_texture, base_gid) = {
            if ts.images.len() != 1 {
                return Err(LoadMapError::TooManyImages);
            }

            let image = &ts.images[0];
            let atlas = load_atlas_texture(&image.source, 16, 16)
                .map_err(|e| LoadMapError::nest(e))?;

            (atlas, ts.first_gid)
        };

        let mut tiles = Vec::with_capacity(ts.tiles.len());
        let mut max_id = 0;

        for (tile_index, in_tile) in ts.tiles.iter().enumerate() {
            let obj_group = match &in_tile.objectgroup {
                Some(og) => og,
                None     => { continue; }
            };

            let mut collider_verts = Vec::new();

            for obj in &obj_group.objects {
                use tiled::ObjectShape::*;
                match &obj.shape {
                    Rect { width, height } => {
                        collider_verts.extend_from_slice(&[
                            V2::new(obj.x,         obj.y),
                            V2::new(obj.x + width, obj.y),
                            V2::new(obj.x + width, obj.y + height),
                            V2::new(obj.x,         obj.y + height),
                        ]);
                    }

                    Polygon { points } => {
                        for (x, y) in points.iter() {
                            collider_verts.push(V2::new(obj.x + x, obj.y + y));
                        }
                    }

                    _ => {
                        eprintln!("bad collider shape in tile {}", tile_index);
                    }
                }
            }

            let out_tile = Tile { collider: Shape::new_from_vec(collider_verts) };
            tiles.push((in_tile.id, out_tile));

            max_id = max_id.max(in_tile.id);
        }

        Ok(Tileset { atlas_texture, base_gid, max_id, tiles })
    }

    fn gid_to_index(&self, gid: u32) -> Option<u32> {
        if gid < self.base_gid { return None; }
        let index = gid - self.base_gid;
        if index > self.max_id { None }
        else { Some(index) }
    }
}

pub struct Map {
    tileset: Tileset,
    tiles:   Vec<Option<u32>>,
    columns: u32,
    rows:    u32,
}

impl Map {
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Map, LoadMapError> {
        let map = tiled::parse_file(path.as_ref())
            .map_err(|e| LoadMapError::nest(e))?;

        let tileset = if map.tilesets.len() != 1 {
            return Err(LoadMapError::TooManyTilesets);
        }
        else {
            Tileset::load(&map.tilesets[0])?
        };

        let main_layer = map.layers.iter()
            .find(|layer| layer.name == "main")
            .ok_or(LoadMapError::MainLayerMissing)?;

        let tiles: Vec<Option<u32>> = main_layer.tiles.iter()
            .flatten()
            .map(|gid| if *gid == 0 { None } else { Some(*gid) })
            .collect();

        let columns = map.width;
        let rows    = map.height;

        Ok(Map{tileset, tiles, columns, rows})
    }

    pub fn tile_at(&self, x: i32, y: i32) -> Option<(&Tileset, &Tile, u32)> {
        let x = x as u32;
        let y = y as u32;
        if x >= self.columns || y >= self.rows {
            None
        }
        else {
            let y = self.rows - y - 1;
            self.tiles[(y * self.columns + x) as usize].map(|gid| {
                let tileset = &self.tileset;
                let index = tileset.gid_to_index(gid).unwrap();
                let tile = &tileset[index];
                (tileset, tile, index)
            })
        }
    }
}

