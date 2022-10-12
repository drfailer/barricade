extern crate sdl2;

mod barricade;
mod ui;

use crate::ui::SDLProp;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::mouse::MouseState;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::Window;
use std::time::Duration;


/**
 * Main loop.
 */
fn run(sdlprop: &mut SDLProp) {
    let mut run: bool = true;

    while run {
        /* draw */
        ui::draw(sdlprop); // can't give a copy of the canvas

        /* handle user inputs */
        ui::event_handler(sdlprop, &mut run);

        /* refresh and wait */
        sdlprop.canvas.present();
        std::thread::sleep(Duration::from_millis(100));
    }
}

pub fn main() {
    /* INIT SDL */
    let sdl_context = sdl2::init().unwrap();

    /* SDL WINDOW */
    let window = sdl_context
        .video().unwrap()
        .window("My happy window", 900, 900).position_centered().build()
        .unwrap();

    /* SDL RENDERER */
    let c = window.into_canvas().build().unwrap(); // SDL_Renderer

    /* EVENT PUMP */
    let ep = sdl_context.event_pump().unwrap(); // SDL_Event

    /* TEXTURE */
    let _image_context = sdl2::image::init(InitFlag::PNG).unwrap();
    let texture_creator = c.texture_creator();
    let t = texture_creator.load_texture("./img/barricade_texture.png").unwrap();

    /* structure for quick access to sdl component */
    let mut sdlprop = SDLProp {
        canvas: c,
        event_pump: ep,
        texture: t,
        window_width: 900,
        window_height: 900
    };

    run(&mut sdlprop);
}
