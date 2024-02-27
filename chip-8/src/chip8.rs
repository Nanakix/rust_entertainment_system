use std::thread;
use std::time::Duration;
use std::time::Instant;
use crate::utils::shift_idiomatic_split_u16;

pub static REFRESH_RATE: f32 = 1.0 / 60.0;

static CHIP8_FONTSET: [u8; 80] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[derive(Debug)]
pub struct Chip8 {
    pub memory: [u8;4096],
    pub opcode: u16,
    pub i: u16,          // index register
    pub v: [u8;16],      // CPU registers from V0 to VE
    pub pc: u16,         // program counter
    pub gfx: [bool; 2048],  // the screen : 64*32 pixels
    pub delay_timer: u8, // timer registers that count down at 60Hz
    pub sound_timer: u8,
    pub stack: [u8;16],
    pub sp: u8,
    pub key: [u8;16], // possible keys of the CHIP-8
}


impl Chip8 {
    pub fn default() -> Self{
        Self {
            memory: [0;4096],
            opcode: 0,
            i: 0,
            v: [0;16],
            pc: 0,
            gfx: [false;2048],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0;16],
            sp: 0,
            key: [0;16],
        }
    }
    pub fn initialize(&mut self) {
        self.pc = 0x200; // Program counter starts at 0x200
        self.opcode = 0; // Reset current opcode
        self.i = 0; // Reset index register
        self.sp = 0; // Reset stack pointer
        self.memory = [0; 4096]; // clear Memory
        self.v = [0;16]; // CPU registers from V0 to VE
        self.delay_timer = 0; // timer registers that count down at 60Hz
        self.sound_timer = 0;
        self.stack = [0; 16]; // Clear stack
        self.key = [0; 16]; // possible keys of the CHIP-8
        self.gfx = [false; 2048]; // the screen : 64*32 pixels

        // Load fontset
        for n in 0..79 {
            self.memory[n + 0x50] = CHIP8_FONTSET[n];
        }
    }

    pub fn emulate_cycle(&mut self) {

        let now = Instant::now();
        {
            // Fetch Opcode
            self.opcode = (self.memory[self.pc as usize] as u16) << 8 | self.memory[(self.pc + 1) as usize] as u16;
            self.pc += 2;   

            // Decode Opcode
            println!("{}", format!("{:#06x}", self.opcode));
            self.decode_opcode();
            
            // Update timers
            self.update_timers();
        }
    
        let elapsed = now.elapsed();
        // println!("Elapsed: {:.2?}", elapsed);

        thread::sleep(Duration::from_secs_f32(REFRESH_RATE) - elapsed); //TODO: Handle case where elapsed > REFRESH_RATE
        
    }

    fn update_timers(&mut self){
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer == 1 {
            println!("BEEP!");
            self.sound_timer -=1;
        }
    }
    /* opcodes reminder

        NNN: address
        NN: 8-bit constant
        N: 4-bit constant
        X and Y: 4-bit register identifier
        PC : Program Counter
        I : 16bit register (For memory address) (Similar to void pointer);
        VN: One of the 16 available variables. N may be 0 to F (hexadecimal);

    */
    fn decode_opcode(&mut self) {
        match self.opcode & 0xF000 { // mask for 4 bits long opcodes

            0x0000 => { 
                match self.opcode & 0x00FF {
                    0x00E0 => { // Clear the screen
                        self.gfx = [false; 2048];
                     },
                    0x00EE => { // Returns from a subroutine
                        let low  = self.stack[self.sp as usize];
                        self.sp -=1;
                        let high  = self.stack[self.sp as usize];
                        self.pc = (high as u16)  << 8 | low as u16;
                    },
                    _ => {println!("Not implemented");}
                }
            },
            0x1000 => {
                // Jumps to address NNN.
                self.pc = self.opcode & 0x0FFF;
            },
            0x2000 => {
                // Calls subroutine at NNN.
                // context save
                let [hi, lo] = shift_idiomatic_split_u16(self.pc);
                self.stack[self.sp as usize] = hi;
                self.sp += 1;
                self.stack[self.sp as usize] = lo;
                self.pc = self.opcode & 0x0FFF;
            },
            0x3000 => {
                // Skips the next instruction if VX equals NN (usually the next instruction is a jump to skip a code block).
                let [reg, nn] = shift_idiomatic_split_u16(self.opcode);
                let reg = reg & 0x0F; 
                if self.v[reg as usize] == nn {
                    self.pc += 2;
                }
                self.pc += 2;
            },
            0x4000 => {
                // Skips the next instruction if VX does not equal NN (usually the next instruction is a jump to skip a code block).
                let [reg, nn] = shift_idiomatic_split_u16(self.opcode);
                let reg = reg & 0x0F; 
                if self.v[reg as usize] != nn {
                    self.pc += 2;
                }
                self.pc += 2;
            },
            0x5000 => {
                // Skips the next instruction if VX equals VY (usually the next instruction is a jump to skip a code block).
                let [vx, vy] = shift_idiomatic_split_u16(self.opcode);
                let vx = vx & 0x0F; 
                let vy = vy & 0xF0;
                if self.v[vx as usize] == self.v[vy as usize] {
                    self.pc += 2;
                }
                self.pc += 2;
            },
            0x6000 => { //6XNN
                // Sets VX to NN.
                let [high, low] = shift_idiomatic_split_u16(self.opcode);
                let high = high & 0x0F; // get the register X
                self.v[high as usize] = low;
                self.pc +=2;
            },
            0x7000 => {
                // Adds NN to VX (carry flag is not changed).
            },
            0x8000 => { // inter registry operations: 0x8XYN
                let [vx, vy] = shift_idiomatic_split_u16(self.opcode);
                let vx = vx & 0x0F;
                let vy = vy & 0xF0; 
                match self.opcode & 0xF00F {
                    
                    0x8000 => { 
                        // Sets VX to the value of VY
                        self.v[vx as usize] = self.v[vy as usize];
                        self.pc+=2;
                    },
                    0x8001 => {
                        // Sets VX to VX OR VY. 
                        self.v[vx as usize] = self.v[vx as usize] | self.v[vy as usize];
                    },
                    0x8002 => {
                        // Sets VX to VX AND VY.
                        self.v[vx as usize] = self.v[vx as usize] & self.v[vy as usize];
                    },
                    0x8003 => {
                        // Sets VX to VX xor VY
                        self.v[vx as usize] = self.v[vx as usize] ^ self.v[vy as usize];
                    },
                    0x8004 => {  
                        // Adds VY to VX. VF is set to 1 when there's an overflow, and to 0 when there is not.
                        let (vz, carry) = vx.overflowing_add(vy);
                        self.v[vx as usize] = vz;
                        self.v[15] = carry as u8;
                        self.pc+=2;
                    },
                    0x8005 => {},
                    0x8006 => {},
                    0x8007 => {},
                    0x800E => {},
                    _ => eprintln!("Unknown opcode:{:#06x}", self.opcode),
                };
            },
            0x9000 => { // 9XY0
                // Skips the next instruction if VX does not equal VY. (Usually the next instruction is a jump to skip a code block).
                let [vx, vy] = shift_idiomatic_split_u16(self.opcode);
                let vx = vx & 0x0F;
                let vy = vy & 0xF0;
                if vx != vy {
                    self.pc += 4;
                } 
                else {
                    self.pc +=2;
                }
            },
            0xA000 => { // set I to the address NNN
                self.i = self.opcode & 0x0FFF;
                self.pc +=2;
            },
            0xB000 => { // BNNN
                // Jumps to the address NNN + V0
                let [high, low] = shift_idiomatic_split_u16(self.opcode);
                let high = high & 0x0F;
                let jump = (high as u16) << 8 | low as u16;

                self.pc += self.v[0] as u16 + jump; // need testing, not sure lol
            },
            0xC000 => { // CXNN
                // Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
                let [vx, nn] = shift_idiomatic_split_u16(self.opcode);
                self.v[vx as usize] = self.v[vx as usize] & nn;
                self.pc +=2;
            },
            0xD000 => { // DXYN
                // Draws a sprite at coordinate (VX, VY) of width 8 bits and height N
                let x = self.v[self.opcode &0x0F00 >> 8 as usize];
                let y = self.v[self.opcode &0x00F0 >> 4 as usize];
                let height = self.opcode & 0x000F;


                self.v[0xF] = 0;

                for h in 0..height - 1 {
                    let pixel = self.memory[self.i + h];
                    for xline in 0..7 {
                        if pixel & (0x80 >> xline) {

                        }
                    }
                }

                self.pc +=2;
            },
            0xE000 => { // Key opcodes
                let [high, low] = shift_idiomatic_split_u16(self.opcode);
                let high = high & 0x0F;
                match self.opcode & 0xF0FF {
                    0xE09E => { // Skips the next instruction if the key stored in VX is pressed
                        println!("awaiting key handling");
                        // self.pc += 2;
                    },
                    0xE0A1 => { // Skips the next instruction if the key stored in VX is not pressed 
                        println!("awaiting key handling");
                        // self.pc += 2;
                    },
                    _ => {
                        println!("Unknown opcode: {:#06x}", self.opcode);
                    }
                }
                // Draws a sprite at coordinate (VX, VY)
                self.pc +=2;
            },
            0xF000 => { // FXNN
                let [reg, nn] = shift_idiomatic_split_u16(self.opcode);
                let reg = reg & 0x0F;
                match self.opcode & 0xF0FF {
                    0xF007 => {
                        // Sets VX to the value of the delay timer
                        self.v[reg as usize] = self.delay_timer;
                        self.pc += 2;
                    },
                    0xF00A => {
                        // A key press is awaited, and then stored in VX (blocking operation, all instruction halted until next key event)
                        self.v[reg as usize] = 0;
                        self.pc += 2;
                    },
                    0xF015 => {
                        // Sets the delay timer to VX
                        self.delay_timer = self.v[reg as usize];
                        self.pc += 2;
                    },
                    0xF018 => {
                        // Sets the sound timer to VX
                        self.sound_timer = self.v[reg as usize];
                        self.pc += 2;
                    },
                    0xF01E => {
                        // Adds VX to I. VF is not affected
                        self.v[reg as usize] = 0;
                        self.pc += 2;
                    },
                    0xF029 => {
                        // Sets I to the location of the sprite for the character in VX. 
                        // Characters 0-F (in hexadecimal) are represented by a 4x5 font.
                        self.v[reg as usize] = 0;
                        self.pc += 2;
                    },
                    0xF033 => {
                        // Stores the binary-coded decimal representation of VX,
                        // with the hundreds digit in memory at location in I, 
                        // the tens digit at location I+1,
                        // and the ones digit at location I+2
                        self.v[reg as usize] = 0;
                        self.pc += 2;
                    },
                    0xF055 => {
                        // Stores from V0 to VX (including VX) in memory, starting at address I.
                        // The offset from I is increased by 1 for each value written, but I itself is left unmodified
                        self.v[reg as usize] = 0;
                        self.pc += 2;
                    },
                    0xF065 => {
                        // Fills from V0 to VX (including VX) with values from memory, starting at address I. 
                        // The offset from I is increased by 1 for each value read, but I itself is left unmodified
                        self.v[reg as usize] = 0;
                        self.pc += 2;
                    },
                    _ => println!("Unknown opcode: {:#06x}", self.opcode)
                }
                self.pc +=2;
            },
            _ => eprintln!("Unknown opcode: {:#06x}", self.opcode),
        };
        self.update_timers();
    }
}


