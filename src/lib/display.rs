use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

// Chip-8 language had a 16-key hexadecimal keypad.
pub(crate) enum Key {
    ZERO,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    A,
    B,
    C,
    D,
    E,
    F,
    NONE,
    QUIT,
}

impl Key {
    pub(crate) fn as_u8(&self) -> u8 {
        match self {
            Key::ZERO => 0,
            Key::ONE => 1,
            Key::TWO => 2,
            Key::THREE => 3,
            Key::FOUR => 4,
            Key::FIVE => 5,
            Key::SIX => 6,
            Key::SEVEN => 7,
            Key::EIGHT => 8,
            Key::NINE => 9,
            Key::A => 10,
            Key::B => 11,
            Key::C => 12,
            Key::D => 13,
            Key::E => 14,
            Key::F => 15,
            Key::NONE => 16,
            Key::QUIT => 17,
        }
    }
}

// display
pub struct Display {
    width: i32,
    height: i32,
    scale: i32,
    sdl_ctx: sdl2::Sdl,
    sdl_canvas: Canvas<Window>,
}

impl Display {
    // scale
    pub fn create(title: String, scale: i32) -> Display {
        let sdl_ctx = sdl2::init().unwrap();
        let sdl_win = sdl_ctx
            .video()
            .unwrap()
            .window(title.as_str(), 64 * scale as u32, 32 * scale as u32)
            .position_centered()
            .build()
            .unwrap();
        let sdl_canvas = sdl_win.into_canvas().build().unwrap();

        Display {
            width: 64,
            height: 32,
            scale,
            sdl_ctx,
            sdl_canvas,
        }
    }

    pub(crate) fn clear(&mut self) {
        self.sdl_canvas.set_draw_color(Color::BLACK);
        self.sdl_canvas.clear();
    }

    pub(crate) fn draw(&mut self, pixels: &[u8; 32 * 64]) {
        self.clear();
        self.sdl_canvas.set_draw_color(Color::GREEN);

        for i in 0..pixels.len() {
            if pixels[i] == 1 {
                let x = i as i32 % self.width;
                let y = i as i32 / self.width;

                self.sdl_canvas
                    .fill_rect(Rect::new(
                        x * self.scale,
                        y * self.scale,
                        self.scale as u32,
                        self.scale as u32,
                    ))
                    .unwrap();
            }
        }

        self.sdl_canvas.present();
    }

    pub(crate) fn user_event(&mut self) -> Result<Key, String> {
        let mut event_pump = self.sdl_ctx.event_pump()?;
        let mut key = Key::NONE;

        for event in event_pump.poll_iter() {
            key = match event {
                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::Num0 => Key::ZERO,
                    Keycode::Num1 => Key::ONE,
                    Keycode::Num2 => Key::TWO,
                    Keycode::Num3 => Key::THREE,
                    Keycode::Num4 => Key::FOUR,
                    Keycode::Num5 => Key::FIVE,
                    Keycode::Num6 => Key::SIX,
                    Keycode::Num7 => Key::SEVEN,
                    Keycode::Num8 => Key::EIGHT,
                    Keycode::Num9 => Key::NINE,
                    Keycode::A => Key::A,
                    Keycode::B => Key::B,
                    Keycode::C => Key::C,
                    Keycode::D => Key::D,
                    Keycode::E => Key::E,
                    Keycode::F => Key::F,
                    _ => Key::NONE,
                },
                Event::Quit { .. } => Key::QUIT,
                _ => Key::NONE,
            }
        }

        Ok(key)
    }
}
