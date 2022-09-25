extern crate sdl2;

use crate::barricade::Barricade;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::video::Window;
use sdl2::EventPump;

// TODO: change the game state

pub struct Game<'a> {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub texture: Texture<'a>,
    window_width: u32,
    window_height: u32,
    barricade: Barricade,
}

impl<'a> Game<'a> {
    pub fn new(
        c: Canvas<Window>,
        ep: EventPump,
        t: Texture<'a>,
        number_of_player: u8,
        ww: u32,
        wh: u32,
    ) -> Self {
        Game {
            canvas: c,
            event_pump: ep,
            texture: t,
            window_width: ww,
            window_height: wh,
            barricade: Barricade::new(number_of_player),
        }
    }

    /**
     * Display the board in the middle of the window
     */
    pub fn drawBoard(&mut self, zoom: f32) {
        let dest_width = 813 * zoom as u32;
        let dest_height = 719 * zoom as u32;
        let dest_x: i32 = (((self.window_width - dest_width) / 2)).try_into().unwrap();
        let dest_y: i32 = ((self.window_height - dest_height) / 2).try_into().unwrap(); // TODO: keep place for the player's pieces
        let src = Rect::new(0, 0, 813, 719);
        let dest = Rect::new(dest_x, dest_y, dest_width, dest_height);

        self.canvas.copy(&self.texture, src, dest);
    }
}
