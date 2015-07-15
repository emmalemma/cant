#import "wire.h"

extern "C" void Wire_begin() {
	Wire.begin();
}
extern "C" void Wire_beginTransmission(uint8_t n) {
	Wire.beginTransmission(n);
}
extern "C" void Wire_write(uint8_t n) {
	Wire.write(n);
}
extern "C" uint8_t Wire_read() {
	return Wire.read();
}
extern "C" void Wire_endTransmission(uint8_t n) {
	Wire.endTransmission(n);
}
extern "C" void Wire_requestFrom(uint8_t id, uint8_t n, uint8_t f) {
	Wire.requestFrom(id, n, f);
}

