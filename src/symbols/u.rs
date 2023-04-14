use crate::types::{
    Position,
    FaceType,
};

const SU_VERT_A: Position = [-0.3,  0.5,  0.0,  1.0];
const SU_VERT_B: Position = [-0.1,  0.5,  0.0,  1.0];
const SU_VERT_C: Position = [-0.3, -0.4,  0.0,  1.0];
const SU_VERT_D: Position = [-0.1, -0.3,  0.0,  1.0];
const SU_VERT_E: Position = [-0.2, -0.5,  0.0,  1.0];
const SU_VERT_F: Position = [ 0.3,  0.5,  0.0,  1.0];
const SU_VERT_G: Position = [ 0.1,  0.5,  0.0,  1.0];
const SU_VERT_H: Position = [ 0.3, -0.4,  0.0,  1.0];
const SU_VERT_I: Position = [ 0.1, -0.3,  0.0,  1.0];
const SU_VERT_J: Position = [ 0.2, -0.5,  0.0,  1.0];

pub const SU_FACE_A: FaceType = (SU_VERT_A, SU_VERT_B, SU_VERT_D);
pub const SU_FACE_B: FaceType = (SU_VERT_A, SU_VERT_C, SU_VERT_D);
pub const SU_FACE_C: FaceType = (SU_VERT_C, SU_VERT_D, SU_VERT_E);
pub const SU_FACE_D: FaceType = (SU_VERT_D, SU_VERT_E, SU_VERT_I);
pub const SU_FACE_E: FaceType = (SU_VERT_E, SU_VERT_I, SU_VERT_J);
pub const SU_FACE_F: FaceType = (SU_VERT_F, SU_VERT_G, SU_VERT_I);
pub const SU_FACE_G: FaceType = (SU_VERT_F, SU_VERT_H, SU_VERT_I);
pub const SU_FACE_H: FaceType = (SU_VERT_H, SU_VERT_I, SU_VERT_J);

pub const SU: [FaceType; 8] = [
    SU_FACE_A,
    SU_FACE_B,
    SU_FACE_C,
    SU_FACE_D,
    SU_FACE_E,
    SU_FACE_F,
    SU_FACE_G,
    SU_FACE_H,
];