//! # Mole
//!
//! A stupid 3D model format that is literally a zstandard messagepack struct. Useful for ingest directly from a game or engine without 5000 dependencies and a 6 hour build. GLTF is too hard :(
//! It is fully self contained, so textures are stored in the file. Hey, it's easy.
//! Pardon the puns. (Note: Moletex = Vertex, Molehill = top level file format, Moledel = one model in the file, Moleterial = material)
use std::fmt::Debug;

use glam::{Vec2, Vec3, Vec4};
use serde::{Deserialize, Serialize};

/// Main container, contains a list of models.
#[derive(Serialize, Deserialize, Debug)]
pub struct Molehill {
    pub models: Vec<Moledel>,
    pub materials: Vec<Moleterial>,
    pub images: Vec<Molemage>,
}

#[derive(Serialize, Deserialize)]
pub struct Molemage {
    pub color_space: ColorSpace,
    pub width: u32,
    pub height: u32,
    /// RGBA
    pub pixels: Vec<u8>,
}

impl From<image::RgbaImage> for Molemage {
    fn from(img: image::RgbaImage) -> Self {
        let w = img.width();
        let h = img.height();

        let pixels = img.into_vec();
        assert_eq!(pixels.len(), w as usize * h as usize * 4);
        Self {
            // Whatever
            color_space: ColorSpace::SRGB,
            width: w,
            height: h,
            pixels,
        }
    }
}

impl Debug for Molemage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Molemage")
            .field("color_space", &self.color_space)
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

/// Whatever. I can add linear later
#[derive(Serialize, Deserialize, Debug)]
pub enum ColorSpace {
    SRGB,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Moledel {
    pub vertices: Vec<Moletex>,
    pub indices: Vec<u32>,
    /// index into materials[]
    pub material: u32,
}

/// Colors all in SRGB
/// Textures are index into textures[]
#[derive(Serialize, Deserialize, Debug)]
pub struct Moleterial {
    pub albedo: Vec4,
    /// If None, uses just albedo value, otherwise albedo * albedo_texture
    pub albedo_texture: Option<u32>,
    pub alpha_clip_threshold: Option<f32>,
    pub tex_coord_scale: Vec2,
    pub tex_coord_offset: Vec2,
    pub double_sided: bool,
    pub roughness: f32,
    /// If None, uses just roughness value, otherwise roughness * roughness_texture
    pub roughness_texture: Option<u32>,
    pub metalness: f32,
    /// If None, uses just metalness value, otherwise metalness * metalness_texture
    pub metalness_texture: Option<u32>,

    pub normal_map: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Moletex {
    pub position: Vec3,
    pub tex_coord: Vec2,
    pub normal: Vec3,
}

pub fn from_bytes(bytes: Vec<u8>) -> Option<Molehill> {
    zstd::decode_all(std::io::Cursor::new(bytes))
        .ok()
        .and_then(|v| rmp_serde::from_slice(&v).ok())
}

pub fn to_bytes(hill: &Molehill) -> Option<Vec<u8>> {
    rmp_serde::to_vec(hill)
        .ok()
        .and_then(|v| zstd::encode_all(std::io::Cursor::new(v), 22).ok())
}
