use std::{thread, time::Duration};

use sdl2::{
    event::Event,
    keyboard::Keycode,
    mouse::MouseButton,
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

    let mut ui_state = UiState::default();

    loop {
        let mut quit = false;

        render(&mut canvas, &ui_state)?;

        for event in events.poll_iter() {
            match event {
                Event::MouseMotion { x, y, .. } => {
                    ui_state.mouse_x = x;
                    ui_state.mouse_y = y;
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    ui_state.mouse_down = true;
                }
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    ui_state.mouse_down = false;
                }
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

fn render<T: RenderTarget>(canvas: &mut Canvas<T>, state: &UiState) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.fill_rect(Rect::new(0, 0, 640, 480))?;

    canvas.set_draw_color(Color::RGB(
        0,
        if state.mouse_down { 255 } else { 0 },
        if !state.mouse_down { 255 } else { 0 },
    ));
    canvas.fill_rect(Rect::new(state.mouse_x - 32, state.mouse_y - 24, 64, 48))?;

    canvas.present();

    Ok(())
}

#[derive(Debug, Default)]
struct UiState {
    mouse_x: i32,
    mouse_y: i32,
    mouse_down: bool,

    hot_item: i32,
    active_item: i32,
}
