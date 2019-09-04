use std::{mem, sync::Mutex, thread, time::Duration};

use lazy_static::lazy_static;
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

        render(&mut canvas, &mut ui_state)?;

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

macro_rules! gen_id {
    ($offset:expr) => {
        line!() as i32 + $offset
    };
}

lazy_static! {
    static ref BACKGROUND_COLOR: Mutex<Color> = Mutex::new(Color::RGB(30, 0, 127));
}

#[allow(clippy::cast_lossless)]
fn render<T: RenderTarget>(canvas: &mut Canvas<T>, state: &mut UiState) -> Result<(), String> {
    let bg_color = &mut BACKGROUND_COLOR.lock().unwrap();

    canvas.set_draw_color(bg_color.clone());
    canvas.fill_rect(Rect::new(0, 0, 640, 480))?;

    state.prepare();

    state.button(canvas, gen_id!(0), 50, 50, 64, 48)?;
    state.button(canvas, gen_id!(0), 150, 50, 64, 48)?;

    if state.button(canvas, gen_id!(0), 50, 150, 64, 48)? {
        mem::replace(&mut **bg_color, Color::RGB(200, 150, 50));
    }

    if state.button(canvas, gen_id!(0), 150, 150, 64, 48)? {
        panic!();
    }

    let mut red = bg_color.r as i32;
    if state.slider(canvas, gen_id!(0), 500, 40, 255, &mut red)? {
        bg_color.r = red as u8;
    }
    let mut green = bg_color.g as i32;
    if state.slider(canvas, gen_id!(0), 550, 40, 255, &mut green)? {
        bg_color.g = green as u8;
    }
    let mut blue = bg_color.b as i32;
    if state.slider(canvas, gen_id!(0), 600, 40, 255, &mut blue)? {
        bg_color.b = blue as u8;
    }

    state.finish();

    canvas.present();

    Ok(())
}

#[derive(Debug, Default)]
struct UiState {
    mouse_x: i32,
    mouse_y: i32,
    mouse_down: bool,

    hot_item: Option<i32>,
    active_item: Option<i32>,
}

impl UiState {
    fn region_hit(&self, x: i32, y: i32, width: u32, height: u32) -> bool {
        !(self.mouse_x < x
            || self.mouse_y < y
            || self.mouse_x >= x + width as i32
            || self.mouse_y >= y + height as i32)
    }

    fn button<T>(
        &mut self,
        canvas: &mut Canvas<T>,
        id: i32,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Result<bool, String>
    where
        T: RenderTarget,
    {
        if self.region_hit(x, y, width, height) {
            self.hot_item = Some(id);
            if self.active_item.is_none() && self.mouse_down {
                self.active_item = Some(id);
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(Rect::new(x + 8, y + 8, width, height))?;

        if self.hot_item == Some(id) {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            if self.active_item == Some(id) {
                canvas.fill_rect(Rect::new(x + 2, y + 2, width, height))?;
            } else {
                canvas.fill_rect(Rect::new(x, y, width, height))?;
            }
        } else {
            canvas.set_draw_color(Color::RGB(127, 127, 127));
            canvas.fill_rect(Rect::new(x, y, width, height))?;
        }

        Ok(self.mouse_down && self.hot_item == Some(id) && self.active_item == Some(id))
    }

    #[allow(dead_code)]
    fn slider<T>(
        &mut self,
        canvas: &mut Canvas<T>,
        id: i32,
        x: i32,
        y: i32,
        max: i32,
        value: &mut i32,
    ) -> Result<bool, String>
    where
        T: RenderTarget,
    {
        let ypos = ((256 - 16) * *value) / max;

        if self.region_hit(x + 8, y + 8, 16, 255) {
            self.hot_item = Some(id);
            if self.active_item.is_none() && self.mouse_down {
                self.active_item = Some(id);
            }
        }

        canvas.set_draw_color(Color::RGB(0x77, 0x77, 0x77));
        canvas.fill_rect(Rect::new(x, y, 32, 256 + 16))?;

        if self.active_item == Some(id) || self.hot_item == Some(id) {
            canvas.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        } else {
            canvas.set_draw_color(Color::RGB(0xaa, 0xaa, 0xaa));
        }
        canvas.fill_rect(Rect::new(x + 8, y + 8 + ypos, 16, 16))?;

        if self.active_item == Some(id) {
            let mut mouse_pos = self.mouse_y - (y + 8);
            if mouse_pos < 0 {
                mouse_pos = 255;
            }
            if mouse_pos > 255 {
                mouse_pos = 255;
            }
            let v = (mouse_pos * max) / 255;
            if v != *value {
                mem::replace(value, v);
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn prepare(&mut self) {
        self.hot_item = None;
    }

    fn finish(&mut self) {
        if !self.mouse_down {
            self.active_item = None;
        }
    }
}
