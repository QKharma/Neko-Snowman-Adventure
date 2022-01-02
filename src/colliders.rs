use ncollide2d::shape::{Ball, Cuboid};

pub struct Collider;
pub struct BallCollider(pub Ball<f32>);
pub struct RectCollider(pub Cuboid<f32>);