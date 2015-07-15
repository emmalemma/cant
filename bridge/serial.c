#include "serial.h"

void Serial_begin(int baud) {
	usb_serial_write("TEST", 4);
}
