use std::num::Float;
use std::num::ToPrimitive;

pub struct Vf32 {
	pub x: f32,
	pub y: f32,
	pub z: f32
}
impl Copy for Vf32 {}
impl Vf32 {
	pub fn new () -> Vf32 {
		Vf32 {x: 0f32, y: 0f32, z: 0f32}
	}
}

pub fn mag2(v:&Vf32) -> f32 {
	(v.x * v.x + v.y * v.y + v.z * v.z)
}

pub fn mag(v:&Vf32) -> f32 {
	mag2(v).sqrt()
}

pub fn copy(tgt:&mut Vf32, src:&Vf32) {
	tgt.x = src.x;
	tgt.y = src.y;
	tgt.z = src.z;
}

pub fn sub(a:&mut Vf32, b:&Vf32) {
	a.x -= b.x;
	a.y -= b.y;
	a.z -= b.z;
}

pub fn normalize(v:&mut Vf32) {
	let mag = mag(v);
	v.x /= mag;
	v.y /= mag;
	v.z /= mag;
}

pub fn cumAvg(cum:&mut Vf32, data:&Vf32, n:u32) {
	let nf32 = n.to_f32().unwrap();
	cum.x = (data.x + (nf32 * cum.x)) / (nf32 + 1f32);
	cum.y = (data.y + (nf32 * cum.y)) / (nf32 + 1f32);
	cum.z = (data.z + (nf32 * cum.z)) / (nf32 + 1f32);
}
