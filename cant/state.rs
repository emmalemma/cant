use std::num::Float;
use std::f32;
use std::num;
use std::num::FromPrimitive;
use std::num::ToPrimitive;
use std::mem;

use arduino::*;
use vector::*;
use bindings;

static kRotationMagnitudeThreshold : f32 = 3000f32;
static kCapsMagnitude : f32 = 30000f32;
static kCapsAMagnitude : f32 = 10000f32;

static LED:u32 = 13;
static MPU_ID:u8 = 0x68;

static mut keybounce : [bool; 5] = [false; 5];
static mut current : usize = 0;
static mut window : [i16; 32] = [0; 32];
static mut total : i32 = 0;
static mut avg: i32 = 0;

static mut led : bool = false;

static mut heartbeat : u32 = 0;

static mut keyDownCache : [bool; 5] = [false; 5];
static mut keyUpCache : [bool; 5] = [false; 5];

static mut keyCache : [bool; 5] = [false; 5];
static mut keys : [bool; 5] = [false; 5];

static mut calibrated : usize = 0;
static mut calibrations : [Vf32; 6] = [Vf32 {x:0f32, y:0f32, z:0f32}; 6];
static mut gyro : Vf32 = Vf32 {x:0f32, y:0f32, z:0f32};
static mut gmag : f32 = 0f32;
static mut acc : Vf32 = Vf32 {x:0f32, y:0f32, z:0f32};
static mut amag : f32 = 0f32;
static mut maxCal : Vf32 = Vf32 {x:0f32, y:0f32, z:0f32};
static mut avgCal : Vf32 = Vf32 {x:0f32, y:0f32, z:0f32};
static mut nCals : u32 = 0;
static mut maxMag : f32 = 0f32;

static mut rotating : bool = false;
static mut rotation : usize = 0;

static kROMCalibration : u32 = 0;
fn readCal () {
	unsafe {
		eeprom_read_block(calibrations.as_ptr() as *const u8, kROMCalibration, mem::size_of_val(&calibrations) as u32);
		for cal in calibrations.iter() {
			if cal.x != 0f32 {
				calibrated += 1;
			}
		}
	}
}
fn writeCal () {
	unsafe {
		eeprom_write_block(calibrations.as_ptr() as *const u8, kROMCalibration, mem::size_of_val(&calibrations) as u32);
	}
}

pub static T :usize = 0;
pub static I :usize = 1;
pub static M :usize = 2;
pub static R :usize = 3;
pub static P :usize = 4;

static mut AcX : i16 = 0;
static mut AcY : i16 = 0;
static mut AcZ : i16 = 0;
static mut Temp : i16 = 0;

struct keybind {
  rotation: i8,
	switch: u8,
	symbol: u8,
	code: u16,
	mode: u8,
	setMode: i8
}
static mut mode : u8 = 0;
static mut modeMask : [bool; 5] = [false; 5];
impl Copy for keybind {}

static mut binds : [keybind; 64] = [keybind{rotation: 0, switch: 0, code: 0, symbol: 0, mode: 0, setMode:-1}; 64];
static mut nBinds : usize = 0;

pub fn bindChar(r:i8, s:usize, c:char, m:u8) {
	unsafe {
		binds[nBinds] = keybind {rotation: r, switch: s as u8, code: 0, symbol: c as u8, mode: m, setMode:-1};
		nBinds += 1;
	}
}
pub fn bindKeycode(r:i8, s:usize, c:u16, m:u8) {
	unsafe {
		binds[nBinds] = keybind {rotation: r, switch: s as u8, code: c as u16, symbol: 0, mode: m, setMode:-1};
		nBinds += 1;
	}
}
pub fn bindMode(r:i8, s:usize, m:u8, sm:i8) {
	unsafe {
		binds[nBinds] = keybind {rotation: r, switch: s as u8, code: 0, symbol: 0, mode: m, setMode:sm};
		nBinds += 1;
	}
}
	
unsafe fn none () {}
static mut state : unsafe fn () = none;

fn setState(s:unsafe fn ()) {
	unsafe {
		state = s;
	}
}
unsafe fn idle () {
	if calibrated < calibrations.len() {
		setState(calibrate);
	} else {
		if rotating {
			ledOn();
			for i in 0..nBinds {
				let bind = binds[i];
				if mode == bind.mode && keyDown(bind.switch as usize) && rotation == bind.rotation as usize{
					if bind.setMode > -1 {
						mode = bind.setMode as u8;
						modeMask = [false; 5];
					}
					if bind.code == 0 && bind.symbol > 0 {
						let c = if gmag > kCapsMagnitude && amag > kCapsAMagnitude {
							bindings::upcase(bind.symbol as char)
						} else { bind.symbol as char };
						usb_keyboard_write(c);
					} else if bind.code > 0 {
						usb_keyboard_press_keycode(bind.code);
						usb_keyboard_release_keycode(bind.code);
					}
				}
			}
		}
	}
}
unsafe fn calibrate () {
	if calibrated < calibrations.len() {
		Print_str("Calibrating an axis. Hold T and rotate.\n");
		maxMag = 0f32;
		avgCal = Vf32::new();
		nCals = 0;
		setState(calibrate_collect);
	} else {
		Print_str("All calibrated! Storing values...\n");
		writeCal();
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
		let calMag = mag(&gyro);
		if calMag > 1000f32 {
			let mut cal = gyro;
			normalize(&mut cal);
			cumAvg(&mut avgCal, &cal, nCals);
			nCals += 1;
		}
	}
	if keyUp(T) {
		Print_str("Confirm calibration.\nM to confirm, P to cancel\n");
		maxCal = avgCal;
		setState(calibrate_confirm);
	}
}
unsafe fn calibrate_confirm () {
	let mut cal = gyro;
	normalize(&mut cal);
	sub(&mut cal, &maxCal);
	let delta = mag(&cal);
	if delta < 0.1 {
		let mut xs : [u8; 64] = [0; 64];
		strcpy(&mut xs, "hit x:     y:     z:      - x:     y:     z:     =      \n", 0);
		f32write(&mut xs, cal.x, 6); 
		f32write(&mut xs, cal.y, 13); 
		f32write(&mut xs, cal.z, 20); 
		f32write(&mut xs, maxCal.x, 30); 
		f32write(&mut xs, maxCal.y, 37); 
		f32write(&mut xs, maxCal.z, 44); 
		f32write(&mut xs, delta, 50); 
		usb_serial_write(&xs);
		ledOn();
	}
	if keyDown(M) {
		Print_str("Calibration accepted!\n");
		copy(&mut calibrations[calibrated], &maxCal);
		calibrated += 1;
		setState(calibrate);
	} else if keyDown(P) {
		Print_str("Recalibrating...\n");
		setState(calibrate);
	}
}

pub fn init () {
	readCal();
	bindings::load();
	pinMode(0, INPUT_PULLUP);
	pinMode(1, INPUT_PULLUP);
	pinMode(2, INPUT_PULLUP);
	pinMode(3, INPUT_PULLUP);
	pinMode(4, INPUT_PULLUP);
	Wire_begin();
	Wire_beginTransmission(MPU_ID);
	Wire_write(0x6B);
	Wire_write(0);
	Wire_endTransmission(true);
	digitalWrite(LED, LOW);
	setState(idle);
}
static mut ticks : u32 = 0;
static mut kCalibrationTriggerTime : u32 = 3000;
static mut calibrationTriggerFrames : u32 = 0;
pub fn read () {
	unsafe {
		Wire_beginTransmission(MPU_ID);
		Wire_write(0x3B);
		Wire_endTransmission(false);
		Wire_requestFrom(MPU_ID, 14, true);
		for k in 0..5 {
			keys[k] = if digitalRead(k as u32) as u8 == LOW {true} else {false};
		}
		acc.x = num::from_i16(Wire_read().to_i16().unwrap_or(0) << 8 | Wire_read().to_i16().unwrap_or(0)).unwrap_or(0f32);
		acc.y = num::from_i16(Wire_read().to_i16().unwrap_or(0) << 8 | Wire_read().to_i16().unwrap_or(0)).unwrap_or(0f32);
		acc.z = num::from_i16(Wire_read().to_i16().unwrap_or(0) << 8 | Wire_read().to_i16().unwrap_or(0)).unwrap_or(0f32);
		Temp = Wire_read().to_i16().unwrap_or(0) << 8 | Wire_read().to_i16().unwrap_or(0);
		gyro.x = num::from_i16(Wire_read().to_i16().unwrap_or(0) << 8 | Wire_read().to_i16().unwrap_or(0)).unwrap_or(0f32);
		gyro.y = num::from_i16(Wire_read().to_i16().unwrap_or(0) << 8 | Wire_read().to_i16().unwrap_or(0)).unwrap_or(0f32);
		gyro.z = num::from_i16(Wire_read().to_i16().unwrap_or(0) << 8 | Wire_read().to_i16().unwrap_or(0)).unwrap_or(0f32);

		gmag = mag(&gyro);
		amag = mag(&acc);

		digitalWrite(LED, LOW);

		Joystick_X((gyro.x / 8f32) as u16);
		Joystick_Y((gyro.y / 8f32) as u16);
		Joystick_Z((gyro.z / 8f32) as u16);


		led =	if heartbeat < 10 {
			true
		} else {
			false
		};
		heartbeat = (heartbeat + 1) % 5000;

		let mut minMag = f32::MAX_VALUE;
		let mut mindex = -1;
		let mut i = 0;
		for cal in calibrations.iter() {
			let mut delta = gyro;
			normalize(&mut delta);
			sub(&mut delta, &cal);
			let mag = mag2(&delta);
			if mag < minMag {
				minMag = mag;
				mindex = i;
			}
			i += 1;
		}

		if gmag > kRotationMagnitudeThreshold {
			rotating = true;
			rotation = mindex;
		} else {
			rotating = false;
			rotation = 0;
		}

		if keys[T] && keys[I] && keys[M] && keys[R] && keys[P] {
			calibrationTriggerFrames += 1;
		} else {
			calibrationTriggerFrames = 0;
		}
		if calibrationTriggerFrames > kCalibrationTriggerTime {
			keyboard_write("cal");
			calibrated = 0;
			setState(calibrate);
		}	
	}
}
pub fn exec () {
	unsafe { state() }
}
pub fn post () {
	unsafe {
		if keyCache[T] && !keys[T] {
			if mode == 1 {
				if !modeMask[I] && !modeMask[M] && !modeMask[R] && !modeMask[P] {
					usb_keyboard_write(' ');
				}
			}
			mode = 0;
		}	
		for k in 0..keys.len() {
			keyCache[k] = keys[k];
			modeMask[k] = modeMask[k] | keys[k];
		}
		digitalWrite(13, (if led {HIGH} else {LOW}));
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
fn f32write (buf:&mut [u8], i:f32, mut n:usize) {
	let c : i32 = i.to_i32().unwrap_or(0);
	intwrite(buf, c, n);
}
fn keyboard_write (s:&str) {
	for c in s.bytes() {
		usb_keyboard_write(bindings::dvorak(c as char));
	}
}
