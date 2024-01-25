use chip_8::chip8::Chip8;
use std::fs::File;
use std::io::{Read, BufReader};
use std::time::Duration;
use std::thread;
use std::path::PathBuf;

fn main() -> std::io::Result<()>{


    // init CPU
    let mut chip_8: Chip8 = Chip8 {
        memory: Vec::with_capacity(4096),
        opcode: 0,
        i: 0, // index register
        v: Vec::with_capacity(16), // CPU registers from V0 to VE
        pc: 0, // program counter
        gfx: vec![false;2048], // the screen : 64*32 pixels
        delay_timer: 0, // timer registers that count down at 60Hz
        sound_timer: 0,
        stack: Vec::with_capacity(16),
        sp: 0,
        key: Vec::with_capacity(16), // possible keys of the CHIP-8
    
    };

    let c8_rom = PathBuf::from("res/chip8-roms/games/Pong (1 player).ch8");
    let file = File::open(c8_rom)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    assert_eq!(contents.len(), 246);
    


    // }
    Ok(while true {
        
        chip_8.emulate_cycle();
        
        //simulate timer countdown speed
        thread::sleep(Duration::from_secs_f32(chip_8::chip8::REFRESH_RATE));
        chip_8.delay_timer -=1;
        chip_8.sound_timer -=1;
    })
  // Set up render system and register input callbacks
//   setupGraphics();
//   setupInput();

  // Initialize the Chip8 system and load the game into the memory  
//   myChip8.initialize();
//   myChip8.loadGame("pong");

  // Emulation loop
//   for(;;)
//   {
//     // Emulate one cycle
//     myChip8.emulateCycle();

//     // If the draw flag is set, update the screen
//     if(myChip8.drawFlag)
//       drawGraphics();

//     // Store key press state (Press and Release)
//     myChip8.setKeys();	
//   }


}

