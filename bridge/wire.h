#import <Wire.h>

extern "C" void Wire_begin();
extern "C" void Wire_beginTransmission(uint8_t);
extern "C" void Wire_write(uint8_t);
extern "C" uint8_t  Wire_read();
extern "C" void Wire_endTransmission(uint8_t);
extern "C" void Wire_requestFrom(uint8_t, uint8_t, uint8_t); 

