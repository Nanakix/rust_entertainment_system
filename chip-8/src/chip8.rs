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

#[derive(Default, Debug)]
pub struct Chip8 {
    pub memory: Vec<u8>,
    pub opcode: u16,
    pub i: u16,          // index register
    pub v: Vec<u8>,      // CPU registers from V0 to VE
    pub pc: u16,         // program counter
    pub gfx: Vec<bool>,  // the screen : 64*32 pixels
    pub delay_timer: u8, // timer registers that count down at 60Hz
    pub sound_timer: u8,
    pub stack: Vec<u8>,
    pub sp: u8,
    pub key: Vec<u8>, // possible keys of the CHIP-8
}


impl Chip8 {
    pub fn initialize(&mut self) {
        self.pc = 0x200; // Program counter starts at 0x200
        self.opcode = 0; // Reset current opcode
        self.i = 0; // Reset index register
        self.sp = 0; // Reset stack pointer
        self.memory = vec![0; 4096]; // clear Memory
        self.v = vec![0;16]; // CPU registers from V0 to VE
        self.delay_timer = 0; // timer registers that count down at 60Hz
        self.sound_timer = 0;
        self.stack = vec![0; 16]; // Clear stack
        self.key = vec![0; 16]; // possible keys of the CHIP-8
        self.gfx = vec![false; 2048]; // the screen : 64*32 pixels    

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
            
            // Execute Opcode

            // Update timers
            self.update_timers();
        }
    
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);

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
                        self.gfx = vec![false; 2048]; 
                     },
                    0x00EE => { // Returns from a subroutine
                        println!("TODO: 0x00EE");
                    },
                    _ => {println!("Not yet implemented: call to routine at NNN");}
                }
            },
            0x1000 => {
                // Jumps to address NNN.
            },
            0x2000 => {
                // Calls subroutine at NNN.
                // context save
                self.stack[self.sp as usize] = self.pc as u8;
                self.sp += 1;
                self.pc = self.opcode & 0x0FFF;
            },
            0x3000 => {
                // Skips the next instruction if VX equals NN (usually the next instruction is a jump to skip a code block).
                let [reg, nn] = shift_idiomatic_split_u16(self.opcode);
                let reg = reg & 0x0F; 
                if self.v[reg as usize] == nn {
                    self.pc += 4;
                }
                else {
                    self.pc += 2;
                }
            },
            0x4000 => {
                // Skips the next instruction if VX does not equal NN (usually the next instruction is a jump to skip a code block).
                let [reg, nn] = shift_idiomatic_split_u16(self.opcode);
                let reg = reg & 0x0F; 
                if self.v[reg as usize] != nn {
                    self.pc += 4;
                }
                else {
                    self.pc += 2;
                }
            },
            0x5000 => {
                // Skips the next instruction if VX equals VY (usually the next instruction is a jump to skip a code block).
                let [vx, vy] = shift_idiomatic_split_u16(self.opcode);
                let vx = vx & 0x0F; 
                let vy = vy & 0xF0;
                if self.v[vx as usize] == self.v[vy as usize] {
                    self.pc += 4;
                }
                else {
                    self.pc += 2;
                }
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
                match self.opcode & 0x000F {
                    0x0000 => { 
                        // Sets VX to the value of VY

                    },
                    0x0001 => {},
                    0x0002 => {},
                    0x0003 => {},
                    0x0004 => {  
                        // Adds VY to VX. VF is set to 1 when there's an overflow, and to 0 when there is not.
                        let [vx, vy] = shift_idiomatic_split_u16(self.opcode);
                        let vx = vx & 0x0F;
                        let vy = vy & 0xF0; 
                        let (vz, carry) = vx.overflowing_add(vy);
                        self.v[vx as usize] = vz;
                        self.v[15] = carry as u8;
                        self.pc+=2;

                    },
                    0x0005 => {},
                    0x0006 => {},
                    0x0007 => {},
                    0x000E => {},
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
            _ => eprintln!("Unknown opcode: {:#06x}", self.opcode),
        };
        self.update_timers();
    }
}


