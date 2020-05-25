pub use crate::context::{CompositeOperationState, ImageId, Path, Vertex};
pub use crate::*;

#[derive(Debug, Copy, Clone)]
pub enum TextureType {
    RGBA,
    Alpha,
}

#[derive(Debug, Copy, Clone)]
pub struct Scissor {
    pub xform: Transform,
    pub extent: Extent,
}

pub trait Renderer {
    fn edge_antialias(&self) -> bool;

    fn create_texture(
        &mut self,
        texture_type: TextureType,
        width: usize,
        height: usize,
        flags: ImageFlags,
        data: Option<&[u8]>,
    ) -> anyhow::Result<ImageId>;

    fn delete_texture(&mut self, img: ImageId) -> anyhow::Result<()>;

    fn update_texture(
        &mut self,
        img: ImageId,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        data: &[u8],
    ) -> anyhow::Result<()>;

    fn texture_size(&self, img: ImageId) -> anyhow::Result<(usize, usize)>;

    fn viewport(&mut self, extent: Extent, device_pixel_ratio: f32) -> anyhow::Result<()>;

    fn cancel(&mut self) -> anyhow::Result<()>;

    fn flush(&mut self) -> anyhow::Result<()>;

    fn fill(
        &mut self,
        paint: &Paint,
        composite_operation: CompositeOperationState,
        scissor: &Scissor,
        fringe: f32,
        bounds: Bounds,
        paths: &[Path],
    ) -> anyhow::Result<()>;

    fn stroke(
        &mut self,
        paint: &Paint,
        composite_operation: CompositeOperationState,
        scissor: &Scissor,
        fringe: f32,
        stroke_width: f32,
        paths: &[Path],
    ) -> anyhow::Result<()>;

    fn triangles(
        &mut self,
        paint: &Paint,
        composite_operation: CompositeOperationState,
        scissor: &Scissor,
        vertexes: &[Vertex],
    ) -> anyhow::Result<()>;
}
