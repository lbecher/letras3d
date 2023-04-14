use crate::types::{
    Position,
    FaceType,
};

const SZ_VERT_A: Position = [-0.3,  0.5,  0.0,  1.0];
const SZ_VERT_B: Position = [ 0.1,  0.5,  0.0,  1.0];
const SZ_VERT_C: Position = [ 0.3,  0.5,  0.0,  1.0];
const SZ_VERT_D: Position = [-0.3,  0.3,  0.0,  1.0];
const SZ_VERT_E: Position = [ 0.1,  0.3,  0.0,  1.0];
const SZ_VERT_F: Position = [ 0.3,  0.3,  0.0,  1.0];
const SZ_VERT_G: Position = [-0.3, -0.3,  0.0,  1.0];
const SZ_VERT_H: Position = [-0.1, -0.3,  0.0,  1.0];
const SZ_VERT_I: Position = [ 0.3, -0.3,  0.0,  1.0];
const SZ_VERT_J: Position = [-0.3, -0.5,  0.0,  1.0];
const SZ_VERT_K: Position = [-0.1, -0.5,  0.0,  1.0];
const SZ_VERT_L: Position = [ 0.3, -0.5,  0.0,  1.0];

pub const SZ_FACE_A: FaceType = (SZ_VERT_A, SZ_VERT_B, SZ_VERT_E);
pub const SZ_FACE_B: FaceType = (SZ_VERT_A, SZ_VERT_D, SZ_VERT_E);
pub const SZ_FACE_C: FaceType = (SZ_VERT_B, SZ_VERT_C, SZ_VERT_E);
pub const SZ_FACE_D: FaceType = (SZ_VERT_C, SZ_VERT_E, SZ_VERT_F);
pub const SZ_FACE_E: FaceType = (SZ_VERT_E, SZ_VERT_F, SZ_VERT_H);
pub const SZ_FACE_F: FaceType = (SZ_VERT_E, SZ_VERT_G, SZ_VERT_H);
pub const SZ_FACE_G: FaceType = (SZ_VERT_G, SZ_VERT_H, SZ_VERT_J);
pub const SZ_FACE_H: FaceType = (SZ_VERT_H, SZ_VERT_J, SZ_VERT_K);
pub const SZ_FACE_I: FaceType = (SZ_VERT_H, SZ_VERT_K, SZ_VERT_L);
pub const SZ_FACE_J: FaceType = (SZ_VERT_H, SZ_VERT_I, SZ_VERT_L);

pub const SZ: [FaceType; 10] = [
    SZ_FACE_A,
    SZ_FACE_B,
    SZ_FACE_C,
    SZ_FACE_D,
    SZ_FACE_E,
    SZ_FACE_F,
    SZ_FACE_G,
    SZ_FACE_H,
    SZ_FACE_I,
    SZ_FACE_J,
];