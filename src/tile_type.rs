use graphics::color::WHITE;
use graphics::types::Color;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TileType {
    Mine,
    Blank
}

impl TileType {
    pub fn color(&self) -> Color {
        const RED: Color = [1.0, 0.0, 0.0, 1.0];
        match *self {
            TileType::Mine => RED,
            TileType::Blank => WHITE
        }
    }
}
