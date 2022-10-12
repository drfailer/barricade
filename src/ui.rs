extern crate sdl2;

use crate::Event;
use crate::MouseState;
use crate::Color;
use crate::Rect;
use crate::Window;

use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::EventPump;

/*****************************************************************************/
/*                               SDLProp STRUCT                              */
/*****************************************************************************/

pub struct SDLProp<'a> {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub texture: Texture<'a>,
    pub window_width: u32,
    pub window_height: u32,
}

/*****************************************************************************/
/*                              EVENTS HANDLERS                              */
/*****************************************************************************/

/* Treate one keyboard event. */
pub fn keyboard_event_handler(event: Event, run: &mut bool) { // {{{
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
} // }}}

/* Treate one mouse event. */
pub fn mouse_event_handler(mouse_event: MouseState, run: &mut bool) { // {{{
    let mx = mouse_event.x();
    let my = mouse_event.y();

    if mouse_event.left() {
        println!("mx: {}, my: {}", mx, my);
        if mx > 200 && mx < 220 && my > 200 && my < 220 {
            *run = false;
        }
    }
} // }}}

/* Treate user inputs. */
pub fn event_handler(sdlprop: &mut SDLProp<'_>, run: &mut bool) { // {{{
    let mut _mstate: MouseState;

    /* KEYBOARD */
    let events_iter = sdlprop.event_pump.poll_iter();
    for event in events_iter {
        keyboard_event_handler(event, run);
    }

    /* MOUSE */
    _mstate = sdlprop.event_pump.mouse_state();
    mouse_event_handler(_mstate, run);
} // }}}

/*****************************************************************************/
/*                               DRAW FUNCTIONS                              */
/*****************************************************************************/

/* Clear the screen (fill it in black). */
pub fn clear(sdlprop: &mut SDLProp) { // {{{
    sdlprop.canvas.set_draw_color(Color::RGB(0, 0, 0));
    sdlprop.canvas.clear();
} // }}}

/* Display the board in the middle of the window */
pub fn drawBoard(sdlprop: &mut SDLProp, zoom: f32) { // {{{
    let dest_width = 813 * zoom as u32;
    let dest_height = 719 * zoom as u32;
    let dest_x: i32 = (((sdlprop.window_width - dest_width) / 2)).try_into().unwrap();
    let dest_y: i32 = ((sdlprop.window_height - dest_height) / 2).try_into().unwrap(); // TODO: keep place for the player's pieces
    let src = Rect::new(0, 0, 813, 719);
    let dest = Rect::new(dest_x, dest_y, dest_width, dest_height);

    sdlprop.canvas.copy(&sdlprop.texture, src, dest);
} // }}}

/* Main draw function. */
pub fn draw(sdlprop: &mut SDLProp) { // {{{
    clear(sdlprop);
    drawBoard(sdlprop, 1.2);
} // }}}
