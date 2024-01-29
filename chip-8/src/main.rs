use chip_8::chip8::Chip8;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    // init CPU
    let mut chip_8 = Chip8::default();
    chip_8.initialize();
    // let c8_rom = PathBuf::from("res/chip8-roms/games/Pong (1 player).ch8");
    let c8_rom = PathBuf::from("res/chip8-roms/tests/1-chip8-logo.ch8");
    let mut file = File::open(c8_rom)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    // assert_eq!(buffer.len(), 246);
    // load ROM at 0x200 (512)
    for n in 0..buffer.len() {
        chip_8.memory[n + 512] = buffer[n];
    }
    let mut cpt = 0;
    loop {
        cpt += 1 ;
        chip_8.emulate_cycle();
        
        if cpt == 40 { // test rom is 39 opcodes long
            break;
        }
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
