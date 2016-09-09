extern crate piston;
extern crate glfw_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate rand;

use piston::window::{Window, WindowSettings};
use piston::input::{RenderEvent, RenderArgs, MouseCursorEvent, PressEvent, Button, MouseButton};
use piston::event_loop::{Events, WindowEvents};
use glfw_window::{GlfwWindow, OpenGL};
use opengl_graphics::GlGraphics;
use graphics::color::{BLACK, WHITE};
use graphics::types::Color;

#[derive(Copy, Clone)]
enum TileType {
    Mine,
    Blank
}

impl TileType {
    fn color(&self) -> Color {
        const RED: Color = [1.0, 0.0, 0.0, 1.0];
        match *self {
            TileType::Mine => RED,
            TileType::Blank => WHITE
        }
    }
}

#[derive(Copy, Clone)]
struct Tile {
    tile_type: TileType,
    hidden: bool
}

impl Tile {
    fn color(&self) -> Color {
        const GREY: Color = [0.66, 0.66, 0.66, 1.0];
        if self.hidden { GREY } else { self.tile_type.color() }
    }
}

struct App {
    gl: GlGraphics,
    mouse_xy: [f64; 2],
    window_xy: [f64; 2],
    minefield: [[Tile; 5]; 5]
}

impl App {
    fn new(opengl: OpenGL) -> App {
        App { 
            gl: GlGraphics::new(opengl), 
            mouse_xy: [0.0; 2],
            window_xy: [0.0; 2],
            minefield: App::random_minefield()
        }
    }

    fn random_minefield() -> [[Tile; 5]; 5] {
        use rand::{thread_rng, Rng};

        let mut rng = thread_rng();
        let tile_choices = [TileType::Mine, TileType::Blank];

        let mut minefield = [[Tile { tile_type: TileType::Blank, hidden: true }; 5]; 5];
        for row in minefield.iter_mut() {
            for tile in row.iter_mut() {
                tile.tile_type = *rng.choose(&tile_choices).unwrap();
            }
        }
         
        minefield
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::{clear, rectangle, Transformed};

        self.window_xy = [args.width as f64, args.height as f64];
        let minefield = &self.minefield;

        let border: f64 = 2.0;
        let (cols, rows) = (minefield[0].len() as f64, minefield.len() as f64);
        let (window_x, window_y) = (self.window_xy[0], self.window_xy[1]);
        let (size_x, size_y) = ((window_x / cols) - (2.0 * border),
                                (window_y / rows) - (2.0 * border));

        self.gl.draw(args.viewport(), |ctx, gl| {
            clear(BLACK, gl);

            for (r, row) in minefield.iter().enumerate() {
                for (c, tile) in row.iter().enumerate() {
                    let (x, y) = (border + ((size_x + (2.0 + border)) * c as f64),
                                  border + ((size_y + (2.0 + border)) * r as f64));
                    
                    let square = rectangle::square(0.0, 0.0, size_x);
                    let trans = ctx.transform.trans(x, y);

                    rectangle(tile.color(), square, trans, gl);
                }
            }
        });
    }

    fn mouse_move(&mut self, mouse_xy: &[f64; 2]) {
        self.mouse_xy = *mouse_xy;
    }

    fn press(&mut self, button: &Button) {
        if *button == Button::Mouse(MouseButton::Left) {
            self.handle_click();
        }
    }

    fn handle_click(&mut self) {
        let (cols, rows) = (self.minefield[0].len() as f64, self.minefield.len() as f64);
        let (window_x, window_y) = (self.window_xy[0], self.window_xy[1]);
        let (mouse_x, mouse_y) = (self.mouse_xy[0], self.mouse_xy[1]);
        let (size_x, size_y) = (window_x / cols, window_y / rows);
        let (c, r) = ((mouse_x / size_x) as usize,
                      (mouse_y / size_y) as usize);

        self.minefield[r][c].hidden = false;
    }
}

pub fn sweep_mines() {
    let opengl = OpenGL::V4_1;

    let mut window: GlfwWindow = WindowSettings::new("minesweepe-rs", (400, 400))
        .fullscreen(false)
        .vsync(true)
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .expect("Failed to create GLFW window.");

    let mut app = App::new(opengl);

    while !window.should_close() {
	    handle_window_events(&mut window, &mut app);
    }
}

fn handle_window_events(window: &mut GlfwWindow, app: &mut App) {
    let mut events: WindowEvents = window.events();
    while let Some(event) = events.next(window) {
        if let Some(render_args) = event.render_args() {
            app.render(&render_args);
        }
        if let Some(mouse_xy) = event.mouse_cursor_args() {
            app.mouse_move(&mouse_xy);
        }
        if let Some(button) = event.press_args() {
            app.press(&button);
        }
    }
}
