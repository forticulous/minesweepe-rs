use viewport::Viewport;
use glfw_window::OpenGL;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use graphics::color::BLACK;
use graphics::{clear, rectangle, text, Transformed};
use rand::{thread_rng, Rng};
use std::path::Path;
use std::cmp;

use tile::Tile;
use tile_type::TileType;

pub struct App {
    gl: GlGraphics,
    mouse_xy: [f64; 2],
    viewport: Viewport,
    minefield: [[Tile; 5]; 5],
    mine_count: usize
}

impl App {
    pub fn new(opengl: OpenGL, mine_count: usize) -> App {
        App { 
            gl: GlGraphics::new(opengl), 
            mouse_xy: [0.0; 2],
            viewport: Viewport { rect: [0; 4], draw_size: [0; 2], window_size: [0; 2] },
            minefield: App::random_minefield(mine_count),
            mine_count: mine_count
        }
    }

    fn random_minefield(mine_count: usize) -> [[Tile; 5]; 5] {
        let mut rng = thread_rng();

        let mut minefield = [[Tile { 
            tile_type: TileType::Blank, 
            hidden: true, 
            marked: false,
            adjacent_mines: 0 
        }; 5]; 5];
        let (max_r, max_c) = (minefield.len(), minefield[0].len());
        let mut mines_added = 0;
        while mines_added < mine_count {
            let (r, c) = (rng.gen_range(0, max_r),
                          rng.gen_range(0, max_c));

            if minefield[r][c].tile_type == TileType::Blank { 
                minefield[r][c].tile_type = TileType::Mine;

                App::update_adjacent_mine_count(&mut minefield, (r, c), (max_r, max_c));

                mines_added += 1;
            }
        }
         
        minefield
    }

    fn update_adjacent_mine_count(minefield: &mut [[Tile; 5]; 5], (r, c): (usize, usize), (max_r, max_c): (usize, usize)) {
        let min_dr = r.checked_sub(1).unwrap_or(0);
        for dr in min_dr..cmp::min(r + 2, max_r) {
            let min_dc = c.checked_sub(1).unwrap_or(0);
            for dc in min_dc..cmp::min(c + 2, max_c) {
                let near_tile = &mut minefield[dr][dc];
                near_tile.adjacent_mines += 1;
            }
        }
    }

    pub fn render(&mut self, viewport: &Viewport) {
        self.viewport = viewport.clone();
        let minefield = &self.minefield;

        let border: f64 = 2.0;
        let (cols, rows) = (minefield[0].len() as f64, minefield.len() as f64);
        let (window_x, window_y) = (self.viewport.window_size[0] as f64, self.viewport.window_size[1] as f64);
        let (size_x, size_y) = ((window_x / cols) - (2.0 * border),
                                (window_y / rows) - (2.0 * border));

        let font_path = Path::new("font/nevis.ttf");
        let mut glyph_cache = GlyphCache::new(font_path)
            .expect("Failed to load font");

        self.gl.draw(*viewport, |ctx, gl| {
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

    fn find_tile(&mut self) -> (usize, usize) {
        let (cols, rows) = (self.minefield[0].len() as f64, self.minefield.len() as f64);
        let (window_x, window_y) = (self.viewport.window_size[0] as f64, self.viewport.window_size[1] as f64);
        let (mouse_x, mouse_y) = (self.mouse_xy[0], self.mouse_xy[1]);
        let (size_x, size_y) = (window_x / cols, window_y / rows);
        let (c, r) = ((mouse_x / size_x) as usize,
                      (mouse_y / size_y) as usize);

        (c, r)
    }

    pub fn new_game(&mut self) {
        self.minefield = App::random_minefield(self.mine_count);
    }

    pub fn handle_click(&mut self) {
        let (c, r) = self.find_tile();

        self.minefield[r][c].click();
    }

    pub fn handle_mark(&mut self) {
        let (c, r) = self.find_tile();

        self.minefield[r][c].mark();
    }
}
