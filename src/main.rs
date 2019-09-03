use sdl2::{event::Event, keyboard::Keycode};

fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let video_subsystem = sdl.video()?;
    let _window = video_subsystem
        .window("sol_tutorial_imgui", 640, 480)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut events = sdl.event_pump()?;

    loop {
        let mut quit = false;

        for event in events.poll_iter() {
            match event {
                Event::KeyUp {
                    keycode: Some(Keycode::Escape),
                    ..
                } => quit = true,
                Event::Quit { .. } => quit = true,
                _ => {}
            }
        }

        if quit {
            break;
        }
    }

    Ok(())
}
