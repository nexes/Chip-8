use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Display {
    width: u32,
    height: u32,
    sdl_ctx: sdl2::Sdl,
    sdl_canvas: Canvas<Window>,
}

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

impl Display {
    pub fn create(title: String, width: u32, height: u32) -> Display {
        let sdl_ctx = sdl2::init().unwrap();
        let sdl_win = sdl_ctx
            .video()
            .unwrap()
            .window(title.as_str(), width, height)
            .position_centered()
            .build()
            .unwrap();
        let sdl_canvas = sdl_win.into_canvas().build().unwrap();

        Display {
            width,
            height,
            sdl_ctx,
            sdl_canvas,
        }
    }

    pub(crate) fn clear(&mut self) {
        self.sdl_canvas.set_draw_color(Color::BLACK);
        self.sdl_canvas.clear();
        self.sdl_canvas.present();
    }

    pub(crate) fn draw(&mut self) {}

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
