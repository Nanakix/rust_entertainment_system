use std::array;

pub static REFRESH_RATE: f32  = 1.0/60.0;

static CHIP8_FONTSET: [u8;80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Chip8 {
    pub memory: Vec<u8>,
    pub opcode: u16,
    pub i: u16, // index register
    pub v: Vec<u8>, // CPU registers from V0 to VE
    pub pc: u16, // program counter
    pub gfx: Vec<bool>, // the screen : 64*32 pixels
    pub delay_timer: u8, // timer registers that count down at 60Hz
    pub sound_timer: u8,
    pub stack: Vec<u8>,
    pub sp: u8,
    pub key: Vec<u8>, // possible keys of the CHIP-8
}

impl Chip8 {

    pub fn initialize(&mut self) {
        self.pc     = 0x200;  // Program counter starts at 0x200
        self.opcode = 0;      // Reset current opcode	
        self.i      = 0;      // Reset index register
        self.sp     = 0;      // Reset stack pointer

        // Clear display	
        // Clear stack
        // Clear registers V0-VF
        // Clear memory
 
        // Load fontset
        for n in 0..79 {
            self.memory[n + 0x50] = CHIP8_FONTSET[n];		
        }
        

// Reset timers
    }

    pub fn emulate_cycle(&mut self) {
        // Fetch Opcode

        // Decode Opcode
        // Execute Opcode
        // Update timers
    }
}