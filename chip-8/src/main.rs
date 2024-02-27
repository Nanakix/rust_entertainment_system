use chip_8::chip8::Chip8;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use std::time::Duration;

extern crate glium;
extern crate winit;


fn main() -> Result<(), String> {
    // init CPU
    let mut chip_8 = Chip8::default();
    chip_8.initialize();


    // let c8_rom = PathBuf::from("res/chip8-roms/games/Pong (1 player).ch8");
    //let c8_rom = PathBuf::from("res/chip8-roms/tests/1-chip8-logo.ch8");
    let c8_rom = PathBuf::from("res/chip8-roms/games/Airplane.ch8");
    let mut file = File::open(c8_rom);
    let mut buffer = Vec::new();
    file.expect("could not open file").read_to_end(&mut buffer).expect("could not read to end");
    // assert_eq!(buffer.len(), 246);
    // load ROM at 0x200 (512)
    for n in 0..buffer.len() {
        chip_8.memory[n + 512] = buffer[n];
    }
        let mut cpt = 0;

    // 1. The **winit::EventLoop** for handling events.
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
    // 2. Create a glutin context and glium Display
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);
    let _ = event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => window_target.exit(),
            _ => (),
            },
            _ => (),
        };
    });
    

    // 'running: loop {
    //         chip_8.emulate_cycle();
    //         ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    // }


//   // If the draw flag is set, update the screen
//   if(myChip8.drawFlag)
//     drawGraphics();

//   // Store key press state (Press and Release)
//   myChip8.setKeys();	
    loop {
        cpt += 1 ;
        chip_8.emulate_cycle();
    }

    Ok(())
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
