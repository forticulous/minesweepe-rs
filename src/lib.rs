extern crate piston;
extern crate glfw_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate rand;

mod app;
mod tile;
mod tile_type;

use piston::window::{Window, WindowSettings};
use piston::input::{RenderEvent, MouseCursorEvent, PressEvent, Button, MouseButton, Key};
use piston::event_loop::{Events, WindowEvents};
use glfw_window::{GlfwWindow, OpenGL};

pub fn sweep_mines() {
    let opengl = OpenGL::V4_1;

    let mut window: GlfwWindow = WindowSettings::new("minesweepe-rs", (400, 400))
        .fullscreen(false)
        .vsync(true)
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .expect("Failed to create GLFW window.");

    let mut app = app::App::new(opengl, 5);

    while !window.should_close() {
        handle_window_events(&mut window, &mut app);
    }
}

fn handle_window_events(window: &mut GlfwWindow, app: &mut app::App) {
    let mut events: WindowEvents = window.events();
    while let Some(event) = events.next(window) {
        if let Some(render_args) = event.render_args() {
            app.render(&render_args);
        }
        if let Some(mouse_xy) = event.mouse_cursor_args() {
            app.mouse_move(&mouse_xy);
        }
        if let Some(button) = event.press_args() {
            match button {
                Button::Mouse(MouseButton::Left) => app.handle_click(),
                Button::Mouse(MouseButton::Right) => app.handle_mark(),
                Button::Keyboard(Key::N) => app.new_game(),
                _ => { }
            }
        }
    }
}
