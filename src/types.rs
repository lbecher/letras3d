use nalgebra::SMatrix;

pub type Position = [f64; 4];
pub type Scale = [f64; 3];
pub type Rotation = [f64; 3];

pub type Matrix4x1 = SMatrix<f64, 4, 1>;
pub type Matrix4x4 = SMatrix<f64, 4, 4>;

pub type FaceType = (Position, Position, Position);

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ShaderVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}