ARDUINO = '/home/adrian/arduino'
RUSTC   = '/usr/local/bin/rustc'
LLC     = '/usr/bin/llc-3.6'
TOOLS   = "#{ARDUINO}/hardware/tools"

PORT = ENV['ARDUINO_DUE_PORT'] || "tty.usbmodemfd1421"

RUST_SRC = 'cant/cant.rs'

# ---------------------------------------------------------------
# Normally you shouldn't need to change anything below this line!
# ---------------------------------------------------------------

USR_C_SRCS = [ARDUINO + '/libraries/Wire/utility/twi.c']
USR_CPP_SRCS = [
'bridge/wire.cpp',
'bridge/joystick.cpp',
ARDUINO + '/libraries/Wire/Wire.cpp']
USR_INCLUDES = []

# ----------------------------------------------------------------------
# Normally you really shouldn't need to change anything below this line!
# ----------------------------------------------------------------------

AR      = "/usr/lib/gcc/arm-none-eabi/bin/arm-none-eabi-ar"
CC      = "/usr/lib/gcc/arm-none-eabi/bin/arm-none-eabi-gcc"
CXX     = "/usr/lib/gcc/arm-none-eabi/bin/arm-none-eabi-g++"
OBJCOPY = "/usr/lib/gcc/arm-none-eabi/bin/arm-none-eabi-objcopy"
SIZE = "/usr/lib/gcc/arm-none-eabi/bin/arm-none-eabi-size"

C_SRCS = [
	'hardware/teensy/cores/teensy3/analog.c',
	'hardware/teensy/cores/teensy3/eeprom.c',
	'hardware/teensy/cores/teensy3/keylayouts.c',
	'hardware/teensy/cores/teensy3/math_helper.c', 
	'hardware/teensy/cores/teensy3/mk20dx128.c',
	'hardware/teensy/cores/teensy3/nonstd.c',
	'hardware/teensy/cores/teensy3/pins_teensy.c',
	'hardware/teensy/cores/teensy3/serial1.c',
	'hardware/teensy/cores/teensy3/serial2.c',
	'hardware/teensy/cores/teensy3/serial3.c',
	'hardware/teensy/cores/teensy3/touch.c',
	'hardware/teensy/cores/teensy3/usb_desc.c',
	'hardware/teensy/cores/teensy3/usb_dev.c',
	'hardware/teensy/cores/teensy3/usb_joystick.c',
	'hardware/teensy/cores/teensy3/usb_keyboard.c',
	'hardware/teensy/cores/teensy3/usb_mem.c',
	'hardware/teensy/cores/teensy3/usb_midi.c',
	'hardware/teensy/cores/teensy3/usb_mouse.c',
	'hardware/teensy/cores/teensy3/usb_rawhid.c',
	'hardware/teensy/cores/teensy3/usb_seremu.c',
	'hardware/teensy/cores/teensy3/usb_serial.c',
].map {|s| ARDUINO + '/' +s} + USR_C_SRCS

CPP_SRCS = [
  'hardware/teensy/cores/teensy3/AudioStream.cpp',
  'hardware/teensy/cores/teensy3/avr_emulation.cpp',
  'hardware/teensy/cores/teensy3/DMAChannel.cpp',
  'hardware/teensy/cores/teensy3/HardwareSerial1.cpp',
  'hardware/teensy/cores/teensy3/HardwareSerial2.cpp',
  'hardware/teensy/cores/teensy3/HardwareSerial3.cpp',
  'hardware/teensy/cores/teensy3/IntervalTimer.cpp',
  'hardware/teensy/cores/teensy3/IPAddress.cpp',
  'hardware/teensy/cores/teensy3/new.cpp',
  'hardware/teensy/cores/teensy3/Print.cpp',
  'hardware/teensy/cores/teensy3/Stream.cpp',
  'hardware/teensy/cores/teensy3/Tone.cpp',
  'hardware/teensy/cores/teensy3/usb_flightsim.cpp',
  'hardware/teensy/cores/teensy3/usb_inst.cpp',
  'hardware/teensy/cores/teensy3/WMath.cpp',
  'hardware/teensy/cores/teensy3/WString.cpp',
  'hardware/teensy/cores/teensy3/yield.cpp',
].map {|s| ARDUINO + '/' + s} + USR_CPP_SRCS

INCLUDES = [
	'/usr/lib/gcc/arm-none-eabi/arm-none-eabi/include',
	"#{ARDUINO}/hardware/teensy/cores/teensy3",
	"#{ARDUINO}/libraries/Wire",
	"#{ARDUINO}/libraries/Wire/utility",
] + USR_INCLUDES

DEFINE = "-D__MK20DX256__ -DF_CPU=72000000 -DLAYOUT_US_ENGLISH -DUSB_SERIAL_HID -DARDUIO=105 -DTEENYDUINO=118"

@cflags = '-c -g -Os -w -mfloat-abi=soft -ffunction-sections -msoft-float -fdata-sections -nostdlib -mcpu=cortex-m4 -mthumb ' + DEFINE + ' ' + INCLUDES.map { |x| "-I#{x}" }.join(' ')

@cxxflags = "#{@cflags} -fno-rtti -fno-exceptions"

@ldflags = '-Os -Wl,--gc-sections -mcpu=cortex-m4 -mfloat-abi=soft -msoft-float'
@ldflags += " -T#{ARDUINO}/hardware/teensy/cores/teensy3/mk20dx256.ld"
@ldflags += ' -Loutput'
@ldflags += ' -mthumb' 
@ldflags += ' -Wl,--check-sections'
@ldflags += ' -Wl,--gc-sections'
@ldflags += ' -Wl,--unresolved-symbols=report-all'
@ldflags += ' -Wl,--warn-common'
@ldflags += ' -Wl,--warn-section-align'
@ldflags += ' -Wl,--warn-unresolved-symbols'
@ldflags += ' -Wl,--start-group'
@ldflags += ' output/arduino.a'
@ldflags += ' output/core.o'
@ldflags += " /usr/lib/gcc/arm-none-eabi/arm-none-eabi/lib/thumb/libm.a" 
@ldflags += ' -Wl,--end-group'

task :all => :core
task :core => 'output/core.bin'
task :hex => 'output/core.hex'
task :clean do
  sh 'rm -r output/'
end

task :flash => :hex do
  # Upload to Programming Port
	sh "teensy_loader_cli -w -v -mmcu=mk20dx128 output/core.hex"
end

directory 'output'
directory 'Cant'

file 'output/core.s' => ['cant/state.rs', 'output'] do
  sh "#{RUSTC} --target arm-unknown-linux-gnueabihf --crate-type=lib --emit=llvm-ir -C soft-float -C no-stack-check -o output/main.ll -A non-upper-case-globals -A unused-imports -A dead-code -A non-snake-case -v #{RUST_SRC} -L cant -L lib -L ." 
  sh "sed -i.1 's/arm-unknown-linux-gnueabihf/arm-none-eabi/g' output/main.ll"
  # This prevents an error: invalid use of function-only attribute
  sh "sed -i.1 's/nocapture readonly/nocapture/g' output/main.ll"
  sh "sed -i.1 's/= distinct !/= !/g' output/main.ll"
  sh "#{LLC} -soft-float -march=thumb -mcpu=cortex-m4 --float-abi=soft -asm-verbose output/main.ll -o=output/core.s"
end

file 'output/core.o' => 'output/core.s' do
  sh "#{CC} #{@cflags} output/core.s -o output/core.o"
end

file 'output/core.elf' => ['output/core.o', 'output/arduino.a'] do
  sh "#{CC} #{@ldflags} -o output/core.elf"
end

file 'output/core.S' => ['output/core.o', 'output/arduino.a'] do
  sh "#{CC} #{@ldflags} -S -o output/core.S"
end

file 'output/core.bin' => 'output/core.elf' do
  sh "#{OBJCOPY} -O binary output/core.elf output/core.bin"
end

file 'output/core.hex' => 'output/core.elf' do
  sh "#{OBJCOPY} -O ihex -R .eeprom output/core.elf output/core.hex"
end


file 'output/arduino.a' => ['output'] do
  C_SRCS.each do |src|
    output = "output/#{File.basename(src, '.c')}.o"
    sh "#{CC} #{@cflags} #{src} -o '#{output}'"
    sh "#{AR} rcs output/arduino.a '#{output}'"
  end

  CPP_SRCS.each do |src|
    output = "output/#{File.basename(src, '.cpp')}.o"
    sh "#{CXX} #{@cxxflags} #{src} -o '#{output}'"
    sh "#{AR} rcs output/arduino.a '#{output}'"
  end
end
