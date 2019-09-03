use std::{thread, time::Duration};

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, RenderTarget},
};

fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let video_subsystem = sdl.video()?;
    let window = video_subsystem
        .window("sol_tutorial_imgui", 640, 480)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let mut events = sdl.event_pump()?;

    loop {
        let mut quit = false;

        render(&mut canvas)?;

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

        thread::sleep(Duration::from_millis(10));

        if quit {
            break;
        }
    }

    Ok(())
}

fn render<T: RenderTarget>(canvas: &mut Canvas<T>) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.fill_rect(Rect::new(0, 0, 640, 480))?;

    canvas.set_draw_color(Color::RGB(0, 0, 255));
    canvas.fill_rect(Rect::new(64, 48, 64, 48))?;

    canvas.present();

    Ok(())
}
