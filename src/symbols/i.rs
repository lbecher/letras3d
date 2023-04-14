use crate::types::{
    Position,
    FaceType,
};

const SI_VERT_A: Position = [-0.3,  0.3,  0.0,  1.0];
const SI_VERT_B: Position = [-0.1,  0.5,  0.0,  1.0];
const SI_VERT_C: Position = [-0.3,  0.5,  0.0,  1.0];
const SI_VERT_D: Position = [-0.1,  0.3,  0.0,  1.0];
const SI_VERT_E: Position = [ 0.1,  0.3,  0.0,  1.0];
const SI_VERT_F: Position = [ 0.1,  0.5,  0.0,  1.0];
const SI_VERT_G: Position = [ 0.3,  0.5,  0.0,  1.0];
const SI_VERT_H: Position = [ 0.3,  0.3,  0.0,  1.0];
const SI_VERT_I: Position = [-0.1, -0.3,  0.0,  1.0];
const SI_VERT_J: Position = [ 0.1, -0.3,  0.0,  1.0];
const SI_VERT_K: Position = [-0.3, -0.3,  0.0,  1.0];
const SI_VERT_L: Position = [ 0.3, -0.3,  0.0,  1.0];
const SI_VERT_M: Position = [-0.3, -0.5,  0.0,  1.0];
const SI_VERT_N: Position = [-0.1, -0.5,  0.0,  1.0];
const SI_VERT_O: Position = [ 0.1, -0.5,  0.0,  1.0];
const SI_VERT_P: Position = [ 0.3, -0.5,  0.0,  1.0];

pub const SI_FACE_A: FaceType = (SI_VERT_A, SI_VERT_B, SI_VERT_C);
pub const SI_FACE_B: FaceType = (SI_VERT_A, SI_VERT_B, SI_VERT_D);
pub const SI_FACE_C: FaceType = (SI_VERT_B, SI_VERT_D, SI_VERT_E);
pub const SI_FACE_D: FaceType = (SI_VERT_B, SI_VERT_E, SI_VERT_F);
pub const SI_FACE_E: FaceType = (SI_VERT_E, SI_VERT_F, SI_VERT_G);
pub const SI_FACE_F: FaceType = (SI_VERT_E, SI_VERT_G, SI_VERT_H);
pub const SI_FACE_G: FaceType = (SI_VERT_E, SI_VERT_D, SI_VERT_I);
pub const SI_FACE_H: FaceType = (SI_VERT_E, SI_VERT_I, SI_VERT_J);
pub const SI_FACE_I: FaceType = (SI_VERT_I, SI_VERT_K, SI_VERT_M);
pub const SI_FACE_J: FaceType = (SI_VERT_I, SI_VERT_M, SI_VERT_N);
pub const SI_FACE_K: FaceType = (SI_VERT_I, SI_VERT_N, SI_VERT_O);
pub const SI_FACE_L: FaceType = (SI_VERT_I, SI_VERT_J, SI_VERT_O);
pub const SI_FACE_M: FaceType = (SI_VERT_J, SI_VERT_L, SI_VERT_O);
pub const SI_FACE_N: FaceType = (SI_VERT_L, SI_VERT_O, SI_VERT_P);

pub const SI: [FaceType; 14] = [
    SI_FACE_A,
    SI_FACE_B,
    SI_FACE_C,
    SI_FACE_D,
    SI_FACE_E,
    SI_FACE_F,
    SI_FACE_G,
    SI_FACE_H,
    SI_FACE_I,
    SI_FACE_J,
    SI_FACE_K,
    SI_FACE_L,
    SI_FACE_M,
    SI_FACE_N,
];