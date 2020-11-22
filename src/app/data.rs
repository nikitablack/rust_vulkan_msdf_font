pub struct MemImage {
    pub image: ash::vk::Image,
    pub view: ash::vk::ImageView,
    pub allocation: vk_mem::Allocation,
    pub allocation_info: vk_mem::AllocationInfo,
    pub extent: ash::vk::Extent3D,
}

pub struct MemBuffer {
    pub buffer: ash::vk::Buffer,
    pub allocation: vk_mem::Allocation,
    pub allocation_info: vk_mem::AllocationInfo,
}

pub struct DebugUtilsData<'a> {
    pub debug_utils_loader: &'a ash::extensions::ext::DebugUtils,
    pub device: ash::vk::Device,
    pub name: &'static str,
}

#[derive(serde::Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct FontAtlas {
    pub atlas_type: String,
    pub size: u32,
    pub width: u32,
    pub height: u32,
    pub yOrigin: String,
}

#[derive(serde::Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct FontMetrics {
    pub lineHeight: f32,
    pub ascender: f32,
    pub descender: f32,
    pub underlineY: f32,
    pub underlineThickness: f32,
}

#[derive(serde::Deserialize, Debug)]
pub struct Bounds {
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
    pub top: f32,
}

#[derive(serde::Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct GlyphData {
    pub unicode: u32,
    pub advance: f32,
    pub planeBounds: Option<Bounds>,
    pub atlasBounds: Option<Bounds>,
}

#[derive(serde::Deserialize, Debug)]
pub struct KerningData {
    pub unicode1: u32,
    pub unicode2: u32,
    pub advance: f32,
}

#[derive(serde::Deserialize, Debug)]
pub struct FontData {
    pub atlas: FontAtlas,
    pub metrics: FontMetrics,
    pub glyphs: Vec<GlyphData>,
    pub kerning: Vec<KerningData>,
}
