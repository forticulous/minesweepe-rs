use piston::input::{RenderArgs, Button, MouseButton};
use glfw_window::OpenGL;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use graphics::color::BLACK;
use graphics::{clear, rectangle, text, Transformed};
use std::path::Path;

use tile::Tile;
use tile_type::TileType;

pub struct App {
    gl: GlGraphics,
    mouse_xy: [f64; 2],
    window_xy: [f64; 2],
    minefield: [[Tile; 5]; 5]
}

impl App {
    pub fn new(opengl: OpenGL) -> App {
        App { 
            gl: GlGraphics::new(opengl), 
            mouse_xy: [0.0; 2],
            window_xy: [0.0; 2],
            minefield: App::random_minefield()
        }
    }

    fn random_minefield() -> [[Tile; 5]; 5] {
        use rand::{thread_rng, Rng};
        use std::cmp;

        let mut rng = thread_rng();
        let num_mines = 5;

        let mut minefield = [[Tile { tile_type: TileType::Blank, hidden: true, adjacent_mines: 0 }; 5]; 5];
        let (max_r, max_c) = (minefield.len(), minefield[0].len());
        let mut mines_added = 0;
        while mines_added < num_mines {
            let (r, c) = (rng.gen_range(0, max_r),
                          rng.gen_range(0, max_c));

            if minefield[r][c].tile_type == TileType::Blank { 
                minefield[r][c].tile_type = TileType::Mine;

                // update adjacent mine counts
                let min_dr = r.checked_sub(1).unwrap_or(0);
                for dr in min_dr..cmp::min(r + 2, max_r) {
                    let min_dc = c.checked_sub(1).unwrap_or(0);
                    for dc in min_dc..cmp::min(c + 2, max_c) {
                        let near_tile = &mut minefield[dr][dc];
                        near_tile.adjacent_mines += 1;
                    }
                }

                mines_added += 1;
            }
        }
         
        minefield
    }

    pub fn render(&mut self, args: &RenderArgs) {

        self.window_xy = [args.width as f64, args.height as f64];
        let minefield = &self.minefield;

        let border: f64 = 2.0;
        let (cols, rows) = (minefield[0].len() as f64, minefield.len() as f64);
        let (window_x, window_y) = (self.window_xy[0], self.window_xy[1]);
        let (size_x, size_y) = ((window_x / cols) - (2.0 * border),
                                (window_y / rows) - (2.0 * border));

        let font_path = Path::new("font/nevis.ttf");
        let mut glyph_cache = GlyphCache::new(font_path)
            .expect("Failed to load font");

        self.gl.draw(args.viewport(), |ctx, gl| {
            clear(BLACK, gl);

            for (r, row) in minefield.iter().enumerate() {
                for (c, tile) in row.iter().enumerate() {
                    let (x, y) = (border + ((size_x + (2.0 + border)) * c as f64),
                                  border + ((size_y + (2.0 + border)) * r as f64));
                    
                    let square = rectangle::square(0.0, 0.0, size_x);
                    let trans = ctx.transform.trans(x, y);

                    rectangle(tile.color(), square, trans, gl);


                    if tile.tile_type == TileType::Blank && !tile.hidden {
                        let nearby_mines = tile.nearby_mines();
                        let trans = trans.trans(size_x / 3.25, size_y / 1.50);
                        text(BLACK, 40, &nearby_mines, &mut glyph_cache, trans, gl);
                    }
                }
            }
        });
    }

    pub fn mouse_move(&mut self, mouse_xy: &[f64; 2]) {
        self.mouse_xy = *mouse_xy;
    }

    pub fn press(&mut self, button: &Button) {
        if *button == Button::Mouse(MouseButton::Left) {
            self.handle_click();
        }
    }

    pub fn handle_click(&mut self) {
        let (cols, rows) = (self.minefield[0].len() as f64, self.minefield.len() as f64);
        let (window_x, window_y) = (self.window_xy[0], self.window_xy[1]);
        let (mouse_x, mouse_y) = (self.mouse_xy[0], self.mouse_xy[1]);
        let (size_x, size_y) = (window_x / cols, window_y / rows);
        let (c, r) = ((mouse_x / size_x) as usize,
                      (mouse_y / size_y) as usize);

        self.minefield[r][c].hidden = false;
    }
}
