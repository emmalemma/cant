#![feature(asm)]

pub static INPUT:u32        = 0x00;
pub static OUTPUT:u32       = 0x01;
pub static INPUT_PULLUP:u32 = 0x02;

pub static LOW:u8           = 0x00;
pub static HIGH:u8          = 0x01;
pub static CHANGE:u8        = 0x02;
pub static FALLING:u8       = 0x03;
pub static RISING:u8        = 0x04;

pub static EXTERNAL:u32     = 0x00;
pub static DEFAULT:u32      = 0x01;

pub static KEY_BACKSPACE:u16 = 0x4000 | 42;

pub mod c {
	extern {
		//pub fn init();
		pub fn malloc(src:u32, size:u32);
		pub fn usb_serial_write(buffer:*const u8, size:usize);
		pub fn usb_keyboard_press(key:u8, modifier:u8);
		pub fn usb_keyboard_press_keycode(code:u16);
		pub fn usb_keyboard_release_keycode(code:u16);
		pub fn usb_keyboard_write_key(code:u16);
		pub fn unicode_to_keycode(cp:u16) -> u16;
		pub fn usb_keyboard_write(c:u8);
		pub fn usb_keyboard_write_unicode(cp:u16);
		pub fn pinMode(pin:u32, mode:u32);

		pub fn eeprom_read_block(buffer:*const u8, addr:u32, len:u32);
		pub fn eeprom_write_block(buffer:*const u8, addr:u32, len:u32);

		pub fn digitalWrite(pin:u32, value:u8);
		pub fn digitalRead(pin:u32) -> i32;

		pub fn analogReference(mode:u8); // TODO: This is an enum, and I _think_ the size is u8 on Cortex Mx
		pub fn analogRead(pin:u32) -> i32;
		pub fn analogWrite(pin:u32, value:u32);

		// pub fn analogReadResolution(res:i32);
		// pub fn analogWriteResolution(res:i32);

		pub fn tone(pin:u32, frequency:u32, duration:u32);
		pub fn noTone(pin:u32);

		pub fn shiftOut(data_pin:u32, clock_pin:u32, bit_order:u32, value:u32);
		pub fn shiftIn(data_pin:u32, clock_pin:u32, bit_order:u32) -> u32;
		pub fn pulseIn(pin:u32, state:u32, timeout:u32) -> u32;

		pub fn millis() -> u32;
		pub fn micros() -> u32;
		pub fn delay(ms:u32);
		pub fn delayMicroseconds(us:u32);

		pub fn attachInterrupt(pin:u32, callback:extern "C" fn(), mode:u32);
		pub fn detachInterrupt(pin:u32);

		pub fn Wire_begin();
		pub fn Wire_beginTransmission(n:u8);
		pub fn Wire_write(n:u8);
		pub fn Wire_read() -> u8;
		pub fn Wire_endTransmission(n:u8);
		pub fn Wire_requestFrom(id:u8, n:u8, f:u8);

		pub fn Joystick_button(button:u8, state:u8);
		pub fn Joystick_X(value:u16);
		pub fn Joystick_Y(value:u16);
		pub fn Joystick_Z(value:u16);
	}
}

// pub fn init() { unsafe { c::init() } }

pub fn Print_str(message:&str) { 
	unsafe { c::usb_serial_write(message.as_ptr(), message.len()) }
}

pub fn usb_serial_write(message:&[u8]) { 
	unsafe { c::usb_serial_write(message.as_ptr(), message.len()) }
 }

pub fn eeprom_write_block(buf:*const u8, addr:u32, len:u32) {
	unsafe {
		c::eeprom_write_block(buf, addr, len);
	}
}
pub fn eeprom_read_block(buf:*const u8, addr:u32, len:u32) {
	unsafe {
		c::eeprom_read_block(buf, addr, len);
	}
}

pub fn usb_keyboard_press(key:u8, modifier:u8) { unsafe { c::usb_keyboard_press(key, modifier) } } 
pub fn usb_keyboard_press_keycode(code:u16) { unsafe { c::usb_keyboard_press_keycode(code) } }
pub fn usb_keyboard_release_keycode(code:u16) { unsafe { c::usb_keyboard_release_keycode(code) } }

pub fn Joystick_button(button:u8, state:u8) { unsafe { c::Joystick_button(button, state); } }
pub fn Joystick_X(value:u16) { unsafe { c::Joystick_X(value); } }
pub fn Joystick_Y(value:u16) { unsafe { c::Joystick_Y(value); } }
pub fn Joystick_Z(value:u16) { unsafe { c::Joystick_Z(value); } }

pub fn Wire_begin() { unsafe { c::Wire_begin(); } }
pub fn Wire_beginTransmission(n:u8) { unsafe { c::Wire_beginTransmission(n) } }
pub fn Wire_write(n:u8) { unsafe { c::Wire_write(n) } }
pub fn Wire_read() -> u8 { unsafe { c::Wire_read() } }
pub fn Wire_endTransmission(n:bool) { unsafe { c::Wire_endTransmission(if n {1} else {0}) } }
pub fn Wire_requestFrom(id:u8, n:u8, f:bool) { unsafe { c::Wire_requestFrom(id, n, (if f {1} else {0})) } }

pub fn pinMode(pin:u32, mode:u32) { unsafe { c::pinMode(pin, mode) } }

pub fn usb_keyboard_write(c:char) { unsafe { c::usb_keyboard_write(c as u8) } }
pub fn usb_keyboard_write_unicode(cp:char) { unsafe { c::usb_keyboard_write_unicode(cp as u16) } }

pub fn digitalWrite(pin:u32, value:u8) { unsafe { c::digitalWrite(pin, value) } }
pub fn digitalRead(pin:u32) -> i32 { unsafe { c::digitalRead(pin) } }

pub fn analogReference(mode:u8) { unsafe { c::analogReference(mode) } }
pub fn analogRead(pin:u32) -> i32 { unsafe { c::analogRead(pin) } }
pub fn analogWrite(pin:u32, value:u32) { unsafe { c::analogWrite(pin, value) } }

// pub fn analogReadResolution(res:i32) { unsafe { c::analogReadResolution(res) } }
// pub fn analogWriteResolution(res:i32) { unsafe { c::analogWriteResolution(res) } }

pub fn tone(pin:u32, frequency:u32, duration:u32) { unsafe { c::tone(pin, frequency, duration) } }
pub fn noTone(pin:u32) { unsafe { c::noTone(pin) } }

pub fn shiftOut(data_pin:u32, clock_pin:u32, bit_order:u32, value:u32) { unsafe { c::shiftOut(data_pin, clock_pin, bit_order, value) } }
pub fn shiftIn(data_pin:u32, clock_pin:u32, bit_order:u32) -> u32 { unsafe { c::shiftIn(data_pin, clock_pin, bit_order) } }
pub fn pulseIn(pin:u32, state:u32, timeout:u32) -> u32 { unsafe { c::pulseIn(pin, state, timeout) } }

pub fn millis() -> u32 { unsafe { c::millis() } }
pub fn micros() -> u32 { unsafe { c::micros() } }
pub fn delay(ms:u32) { unsafe { c::delay(ms) } }
pub fn delayMicroseconds(us:u32) { unsafe { c::delayMicroseconds(us) } }

pub fn attachInterrupt(pin:u32, callback:extern "C" fn(), mode:u32) { unsafe { c::attachInterrupt(pin, callback, mode) } }
pub fn detachInterrupt(pin:u32) { unsafe { c::detachInterrupt(pin) } }


#[inline(always)]
pub fn interrupts() {
	unsafe {
		asm!("CPSIE i");
	}
}

#[inline(always)]
pub fn noInterrupts() {
	unsafe {
		asm!("CPSID i");
	}
}
