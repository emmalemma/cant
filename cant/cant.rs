#![feature(core)]
#![feature(asm)]

static LED : u32 = 13;
use arduino::*;	
mod arduino;
mod state;
mod vector;
mod bindings;

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
	Print_str("Booting up\n");
	
	state::init();
	loop {
		state::read();
		state::exec();
		state::post();
	}
}
