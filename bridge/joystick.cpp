#import "joystick.h"

extern "C" void Joystick_button(uint8_t button, uint8_t state) {
	Joystick.button(button, state);
}

extern "C" void Joystick_X(uint16_t v) {
	Joystick.X(v);
}
extern "C" void Joystick_Y(uint16_t v) {
	Joystick.Y(v);
}
extern "C" void Joystick_Z(uint16_t v) {
	Joystick.Z(v);
}
extern "C" void Joystick_R(uint16_t v) {
	Joystick.Zrotate(v);
}
extern "C" void Joystick_G(uint16_t v) {
	Joystick.sliderLeft(v);
}
extern "C" void Joystick_B(uint16_t v) {
	Joystick.sliderRight(v);
}
