#![no_std]

#![feature(lang_items)]
#![feature(core)]


#![feature(asm)]
extern crate core;
#[no_mangle] pub extern "C" fn rust_stack_exhausted() {
    unsafe { abort() }
}
#[no_mangle] pub extern "C" fn rust_begin_unwind() {
    unsafe { main() }
}

use core::intrinsics::abort;
use core::num::ToPrimitive;
use core::num::Float;
use core::num;
use core::fmt;
use core::str;
use core::cmp;
use core::str::StrExt;

use core::slice;
use core::iter::Iterator;
use core::slice::SliceExt;

mod std {
	pub mod option {
		pub use core::option::*;
	}
	pub mod iter {
		pub use core::iter::*;
	}
}

mod arduino;

static PWM:u32 = 2;
static LED:u32 = 13;

static PWM_LOW:u32 = 0;
static PWM_HIGH:u32 = 16;

static MPU_ID:u8 = 0x68;

static mut keybounce : [bool; 5] = [false; 5];
static mut current : usize = 0;
static mut window : [i16; 32] = [0; 32];
static mut total : i32 = 0;
static mut avg: i32 = 0;
struct Vi32 {
	x: i32,
	y: i32,
	z: i32
}
fn mag(v:&Vi32) -> i32 {
	let x = v.x.to_f32().unwrap();
	let y = v.y.to_f32().unwrap();
	let z = v.z.to_f32().unwrap();
	(x * x + y * y + z * z).sqrt().to_i32().unwrap_or(9)
}


static mut led : bool = false;

static mut keyDownCache : [bool; 5] = [false; 5];
static mut keyUpCache : [bool; 5] = [false; 5];

static mut keyCache : [bool; 5] = [false; 5];
static mut keys : [bool; 5] = [false; 5];

static mut calibrated : usize = 0;
static mut calibrations : [Vi32; 6] = [Vi32 {x:0,y:0,z:0},Vi32 {x:0,y:0,z:0},Vi32 {x:0,y:0,z:0},Vi32 {x:0,y:0,z:0},Vi32 {x:0,y:0,z:0},Vi32 {x:0,y:0,z:0},];
static mut cal : Vi32 = Vi32 {x:0,y:0,z:0};
static mut maxCal : Vi32 = Vi32 {x: 0, y: 0, z: 0};
static mut maxMag : i32 = 0;

static T :usize = 0;
static I :usize = 1;
static M :usize = 2;
static R :usize = 3;
static P :usize = 4;

static mut AcX : i16 = 0;
static mut AcY : i16 = 0;
static mut AcZ : i16 = 0;
static mut Temp : i16 = 0;
static mut GyX : i16 = 0;
static mut GyY : i16 = 0;
static mut GyZ : i16 = 0;
	
unsafe fn none () {}
static mut state : unsafe fn () = none;

#[no_mangle]
pub fn main() {
    delay(10);
    pinMode(LED, OUTPUT);
    digitalWrite(LED, HIGH);
		delay(100);
    digitalWrite(LED, LOW);
		delay(100);
    digitalWrite(LED, HIGH);
		delay(100);
    Wire_begin();
    Wire_beginTransmission(MPU_ID);
    Wire_write(0x6B);
    Wire_write(0);
    Wire_endTransmission(true);
    // digitalWrite(LED, LOW);
		Print_str("Booting up\n");

	pinMode(0, INPUT_PULLUP);	
	pinMode(1, INPUT_PULLUP);	
	pinMode(2, INPUT_PULLUP);	
	pinMode(3, INPUT_PULLUP);	
	pinMode(4, INPUT_PULLUP);	
    
    let mut xs : [u8; 36] = [0; 36];
		/*
			 let mut slice = unsafe {
			 slice::from_raw_mut_buf(&ptr, ct) 
			 };

			 slice[0] = 'a' as u8;
		 */
		fn setState(s:unsafe fn ()) {
			unsafe {
				state = s;
			}
		}
		unsafe fn idle () {
			if calibrated < calibrations.len() {
				setState(calibrate);
			}
		}
		unsafe fn calibrate () {
			if calibrated < calibrations.len() {
				Print_str("Calibrating an axis. Hold T and rotate.\n");
				setState(calibrate_collect);
			} else {
				Print_str("All calibrated!\n");
				setState(idle);
			}
		}
		unsafe fn keyDown(key:usize) -> bool {
			if keys[key] {
			  if !keyCache[key] {
					return true
				}
			}
			false
		}
		unsafe fn keyUp(key:usize) -> bool {
			if !keys[key] {
			  if keyCache[key] {
					return true
				}
			}
			false
		}
		unsafe fn ledOn () {
			led = true;
		}
		unsafe fn calibrate_collect () {
			if keys[T]{
				ledOn();
				cal.x = GyX as i32;
				cal.y = GyY as i32;
				cal.z = GyZ as i32;
				let calMag = mag(&cal);
				if calMag > maxMag {
					maxCal.x = cal.x;
					maxCal.y = cal.y;
					maxCal.z = cal.z;
					maxMag = calMag;
				}
			}
			if keyUp(T) {
				Print_str("Confirm calibration.\nM to confirm, P to cancel\n");
				setState(calibrate_confirm);
			}
		}
		unsafe fn calibrate_confirm () {
			cal.x = GyX as i32 - maxCal.x;
			cal.y = GyY as i32 - maxCal.y;
			cal.z = GyZ as i32 - maxCal.z;
			let delta = mag(&cal);
			if delta < 1000 {
				let mut xs : [u8; 64] = [0; 64];
				strcpy(&mut xs, "hit x:     y:     z:      - x:     y:     z:     =      \n", 0);
				intwrite(&mut xs, GyX as i32, 6); 
				intwrite(&mut xs, GyY as i32, 13); 
				intwrite(&mut xs, GyZ as i32, 20); 
				intwrite(&mut xs, maxCal.x, 30); 
				intwrite(&mut xs, maxCal.y, 37); 
				intwrite(&mut xs, maxCal.z, 44); 
				intwrite(&mut xs, delta, 50); 
				usb_serial_write(&xs);
				ledOn();
			}
			if keyDown(M) {
				Print_str("Calibration accepted!\n");
				calibrations[calibrated].x = maxCal.x;
				calibrations[calibrated].y = maxCal.y;
				calibrations[calibrated].z = maxCal.z;
				calibrated += 1;
				setState(calibrate);
			} else if keyDown(P) {
				Print_str("Recalibrating...\n");
				maxMag = 0;
				setState(calibrate_collect);
			}
		}
		setState(idle);
		let mut heartbeat = 0;
		loop {
			Wire_beginTransmission(MPU_ID);
			Wire_write(0x3B);
			Wire_endTransmission(false);
			Wire_requestFrom(MPU_ID, 14, true);
		unsafe {
			for k in 0..5 {
				keys[k] = if digitalRead(k as u32) as u8 == LOW {true} else {false};
			}
			AcX = Wire_read().to_i16().unwrap_or(0) << 8 | Wire_read().to_i16().unwrap_or(0);
			AcY = Wire_read().to_i16().unwrap_or(0) << 8 | Wire_read().to_i16().unwrap_or(0);
			AcZ = Wire_read().to_i16().unwrap_or(0) << 8 | Wire_read().to_i16().unwrap_or(0);
			Temp = Wire_read().to_i16().unwrap_or(0) << 8 | Wire_read().to_i16().unwrap_or(0);
			GyX = Wire_read().to_i16().unwrap_or(0) << 8 | Wire_read().to_i16().unwrap_or(0);
			GyY = Wire_read().to_i16().unwrap_or(0) << 8 | Wire_read().to_i16().unwrap_or(0);
			GyZ = Wire_read().to_i16().unwrap_or(0) << 8 | Wire_read().to_i16().unwrap_or(0);
			led =	if heartbeat < 10 {
				true
			} else {
				false
			};
			heartbeat = (heartbeat + 1) % 5000;
			state();
			for k in 0..keys.len() {
				keyCache[k] = keys[k];
			}
			digitalWrite(13, (if led {HIGH} else {LOW}));
		}
	}
}

fn strcpy (buf:&mut [u8], s:&str,mut  n:usize) {
	for b in s.bytes() {
		buf[n] = b;
		n+=1;
	}
}
fn intwrite (buf:&mut [u8], i:i32, mut n:usize) {
	let mut tot = i;
	let f : f32 = Float::abs(num::from_i32(tot).unwrap_or(1.0f32));
	let mut len: usize = Float::ceil(Float::log10(f)).to_int().unwrap_or(7) as usize;
	if tot < 0 {
		buf[n] = '-' as u8;
		len += 1;
		tot = -tot;
	}

	if tot == 0 {
		buf[n] = '0' as u8;
	}
	while tot > 0 {
		len -= 1;
		buf[n + len] = '0' as u8 + (tot % 10) as u8;
		tot = (tot / 10);
	}
}
