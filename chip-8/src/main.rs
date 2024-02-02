use chip_8::chip8::Chip8;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;


fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn main() -> Result<(), String> {
    // init CPU
    let mut chip_8 = Chip8::default();
    chip_8.initialize();


    // let c8_rom = PathBuf::from("res/chip8-roms/games/Pong (1 player).ch8");
    let c8_rom = PathBuf::from("res/chip8-roms/tests/1-chip8-logo.ch8");
    let mut file = File::open(c8_rom);
    let mut buffer = Vec::new();
    file.expect("could not open file").read_to_end(&mut buffer).expect("could not read to end");
    // assert_eq!(buffer.len(), 246);
    // load ROM at 0x200 (512)
    for n in 0..buffer.len() {
        chip_8.memory[n + 512] = buffer[n];
    }
        let mut cpt = 0;


    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

        'running: loop {
            chip_8.emulate_cycle();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
    
            canvas.clear();
            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
            // The rest of the game loop goes here...
        }


  // Emulate one cycle
//   myChip8.emulateCycle();

//   // If the draw flag is set, update the screen
//   if(myChip8.drawFlag)
//     drawGraphics();

//   // Store key press state (Press and Release)
//   myChip8.setKeys();	
    loop {
        cpt += 1 ;
        
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
