extern crate sdl2;

use crate::game::Game;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseState;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::Window;
use std::time::Duration;
mod barricade;
mod game;

fn keyboard_event_handler(event: Event, run: &mut bool) {
    match event {
        Event::Quit { .. }
        | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        }
        | Event::KeyDown {
            keycode: Some(Keycode::Q),
            ..
        } => *run = false,
        _ => {} // otherwhise do nothing
    }
}

fn mouse_event_handler(mouse_event: MouseState, run: &mut bool) {
    let mx = mouse_event.x();
    let my = mouse_event.y();
    if mouse_event.left() {
        println!("mx: {}, my: {}", mx, my);
        if mx > 200 && mx < 220 && my > 200 && my < 220 {
            *run = false;
        }
    }
}

fn draw(game: &mut Game) {
    game.canvas.set_draw_color(Color::RGB(0, 0, 0));
    game.canvas.clear();

    // (*canvas).set_draw_color(Color::RGB(255, 255, 255));
    // _ = canvas.fill_rect(Rect::new(200, 200, 20, 20)).unwrap();
    game.drawBoard(1.2);
}

fn run(game: &mut Game) {
    let mut run: bool = true;
    let mut _mstate: MouseState;

    while run { // TODO: use the game state
        /* DRAW */
        // draw stuff
        draw(game); // can't give a copy of the canvas

        /* KEYBOARD */
        // manage events
        for event in game.event_pump.poll_iter() {
            keyboard_event_handler(event, &mut run);
        }

        /* MOUSE */
        // get a mouse state and test
        _mstate = game.event_pump.mouse_state();
        mouse_event_handler(_mstate, &mut run);

        // refresh the screen (not necessary here but this is where it should be)
        game.canvas.present();
        std::thread::sleep(Duration::from_millis(100));
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap(); // sdl if (SDL_Init(...) { ... })
    let video_subsystem = sdl_context.video().unwrap(); // sdl init video + error handling
    let window = video_subsystem // SDL_Window
        .window("My happy window", 900, 900)
        .position_centered()
        .build()
        .unwrap();
    let canvas = window.into_canvas().build().unwrap(); // SDL_Renderer
    let event_pump = sdl_context.event_pump().unwrap(); // SDL_Event
    let _image_context = sdl2::image::init(InitFlag::PNG).unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .load_texture("./img/barricade_texture.png")
        .unwrap(); // ./ is the root of the project and not the scr/ directory
    let mut game = Game::new(canvas, event_pump, texture, 2, 900, 900);
    run(&mut game);
}
