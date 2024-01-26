use chip_8::chip8::Chip8;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() -> std::io::Result<()>{


    // init CPU
    let mut chip_8: Chip8 = Chip8 {
        memory: vec![0;4096],
        opcode: 0,
        i: 0, // index register
        v: Vec::with_capacity(16), // CPU registers from V0 to VE
        pc: 0x200, // program counter
        gfx: vec![false;2048], // the screen : 64*32 pixels
        delay_timer: 0, // timer registers that count down at 60Hz
        sound_timer: 0,
        stack: Vec::with_capacity(16),
        sp: 0,
        key: Vec::with_capacity(16), // possible keys of the CHIP-8
    
    };

    let c8_rom = PathBuf::from("res/chip8-roms/games/Pong (1 player).ch8");
    let mut file = File::open(c8_rom)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    assert_eq!(buffer.len(), 246);
    // load ROM at 0x200 (512)
    for n in 0..buffer.len() {
      chip_8.memory[n + 512] = buffer[n];
    }
    loop {
        
        chip_8.emulate_cycle();
        
        //simulate timer countdown speed
       
    }

    Ok(())
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

