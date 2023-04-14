use crate::types::{
    Position,
    FaceType,
};

const SL_VERT_A: Position = [-0.3,  0.5,  0.0,  1.0];
const SL_VERT_B: Position = [-0.3, -0.3,  0.0,  1.0];
const SL_VERT_C: Position = [-0.1, -0.3,  0.0,  1.0];
const SL_VERT_D: Position = [-0.1,  0.5,  0.0,  1.0];
const SL_VERT_E: Position = [-0.3, -0.5,  0.0,  1.0];
const SL_VERT_F: Position = [-0.1, -0.5,  0.0,  1.0];
const SL_VERT_G: Position = [ 0.3, -0.3,  0.0,  1.0];
const SL_VERT_H: Position = [ 0.3, -0.5,  0.0,  1.0];

pub const SL_FACE_A: FaceType = (SL_VERT_A, SL_VERT_B, SL_VERT_C);
pub const SL_FACE_B: FaceType = (SL_VERT_A, SL_VERT_C, SL_VERT_D);
pub const SL_FACE_C: FaceType = (SL_VERT_C, SL_VERT_E, SL_VERT_B);
pub const SL_FACE_D: FaceType = (SL_VERT_C, SL_VERT_E, SL_VERT_F);
pub const SL_FACE_E: FaceType = (SL_VERT_C, SL_VERT_F, SL_VERT_H);
pub const SL_FACE_F: FaceType = (SL_VERT_C, SL_VERT_H, SL_VERT_G);

pub const SL: [FaceType; 6] = [
    SL_FACE_A,
    SL_FACE_B,
    SL_FACE_C,
    SL_FACE_D,
    SL_FACE_E,
    SL_FACE_F,
];