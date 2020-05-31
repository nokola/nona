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
    ) -> NonaResult<ImageId>;

    fn delete_texture(&mut self, img: ImageId) -> NonaResult<()>;

    fn update_texture(
        &mut self,
        img: ImageId,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        data: &[u8],
    ) -> NonaResult<()>;

    fn texture_size(&self, img: ImageId) -> NonaResult<(usize, usize)>;

    fn viewport(&mut self, extent: Extent, device_pixel_ratio: f32) -> NonaResult<()>;

    fn flush(&mut self) -> NonaResult<()>;

    fn fill(
        &mut self,
        paint: &Paint,
        composite_operation: CompositeOperationState,
        scissor: &Scissor,
        fringe: f32,
        bounds: Bounds,
        paths: &[Path],
    ) -> NonaResult<()>;

    fn stroke(
        &mut self,
        paint: &Paint,
        composite_operation: CompositeOperationState,
        scissor: &Scissor,
        fringe: f32,
        stroke_width: f32,
        paths: &[Path],
    ) -> NonaResult<()>;

    fn triangles(
        &mut self,
        paint: &Paint,
        composite_operation: CompositeOperationState,
        scissor: &Scissor,
        vertexes: &[Vertex],
    ) -> NonaResult<()>;
}
