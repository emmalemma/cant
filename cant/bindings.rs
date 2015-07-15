use state::{bindChar, bindMode, bindKeycode, T, I, R, M, P};

static KEY_ENTER : u16 = 0x4000 | 40;
static KEY_ESC : u16 = 0x4000 | 41;
static KEY_BACKSPACE : u16 = 0x4000 | 42;
static KEY_TAB : u16 = 0x4000 | 43;
pub static KEY_A : u16 = 0x4000 | 4;

static Over : i8 = 4;
static Under : i8 = 5;
static Left : i8 = 0;
static Right : i8 = 1;
static Up : i8 = 2;
static Down : i8 = 3;
static mut current_mode :u8 = 0;
fn bind_mode(i:u8) {
	unsafe { current_mode = i }
}
fn currentMode() -> u8 {
	unsafe { current_mode }
}
pub fn load() {
	bind_mode(0);
	bindDv(Under, T, 'A');
	bindDv(Under, I, 'E');
	bindDv(Under, M, 'I');
	bindDv(Under, R, 'O');
	bindDv(Under, P, 'U');

	bindDv(Left, T, 'R');
	bindDv(Left, I, 'S');
	bindDv(Left, M, 'T');
	bindDv(Left, R, 'H');
	bindDv(Left, P, 'C');

	bindDv(Right, T, 'B');
	bindDv(Right, I, 'F');
	bindDv(Right, M, 'G');
	bindDv(Right, R, 'Y');
	bindDv(Right, P, ' ');

	bindMode(Over, T, currentMode(), 1);
	bindDv(Over, I, 'D');
	bindDv(Over, M, 'M');
	bindDv(Over, R, 'N');
	bindDv(Over, P, 'L');

	bindDv(Up, T, 'P');
	bindDv(Up, I, 'V');
	bindDv(Up, M, 'W');
	bindDv(Up, R, 'K');
	bindDv(Up, P, ' ');

	bindDv(Down, T, ' ');
	bindDv(Down, I, ' ');
	bindDv(Down, M, ' ');
	bindDv(Down, R, ' ');
	bindDv(Down, P, ' ');
	
	bind_mode(1);

	bindDv(Over, I, '.');
	bindDv(Over, M, ',');
	bindDv(Over, R, '!');
	bindDv(Over, P, '?');

	bindDv(Left, I, 'J');
	bindDv(Left, M, 'Q');
	bindDv(Left, R, 'X');
	bindDv(Left, P, 'Z');

	bindDv(Up, I, '1');
	bindDv(Up, M, '2');
	bindDv(Up, R, '4');
	bindDv(Up, P, '8');

	bindKc(Under, I, KEY_TAB);
	bindKc(Under, M, KEY_ENTER);
	bindKc(Under, R, KEY_BACKSPACE);
	bindKc(Under, P, KEY_ESC);
}

fn bindKc(r:i8, s:usize, c:u16) {
	bindKeycode(r,s,c,currentMode());
}

fn bindDv(r:i8, s:usize, c:char) {
	bindChar(r,s,dvorak(c),currentMode());
}
pub fn dvorak(c:char) -> char {
	match c {
		'A' => 'a',
		'B' => 'n',
		'C' => 'i',
		'D' => 'h',
		'E' => 'd',
		'F' => 'y',
		'G' => 'u',
		'H' => 'j',
		'I' => 'g',
		'J' => 'c',
		'K' => 'v',
		'L' => 'p',
		'M' => 'm',
		'N' => 'l',
		'O' => 's',
		'P' => 'r',
		'Q' => 'x',
		'R' => 'o',
		'S' => ';',
		'T' => 'k',
		'U' => 'f',
		'V' => '.',
		'W' => ',',
		'X' => 'b',
		'Y' => 't',
		'Z' => '/',
		',' => 'w',
		'.' => 'e',
		'?' => '{',
		_ => c
	}
}

pub fn upcase(c:char) -> char {
	match c {
		'a' => 'A',
		'b' => 'B',
		'c' => 'C',
		'd' => 'D',
		// 'e' => 'E',
		'f' => 'F',
		'g' => 'G',
		'h' => 'H',
		'i' => 'I',
		'j' => 'J',
		'k' => 'K',
		'l' => 'L',
		'm' => 'M',
		'n' => 'N',
		'o' => 'O',
		'p' => 'P',
		'q' => 'Q',
		'r' => 'R',
		's' => 'S',
		't' => 'T',
		'u' => 'U',
		'v' => 'V',
		'x' => 'X',
		'y' => 'Y',
		'z' => 'Z',
		';' => ':',
		',' => '<',
		'.' => '>',
		'/' => '?',
		'e' => 'w', // , -> .
		_ => c
	}   
}

