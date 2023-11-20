#[cfg(feature = "debug")]
use colored::Colorize;

// Enum of Minesweeper tiles
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
  Bomb,
  BombNeighbour(u8),
  Empty,
}

impl Tile {
  pub const fn is_bomb(&self) -> bool {
    matches!(self, Self::Bomb)
  }

  #[cfg(feature = "debug")]
  pub fn console_output(&self) -> String {
    format!(
      "{}",
      match self {
        Tile::Bomb => "*".bright_red(),
        Tile::BombNeighbour(n) => match n {
          1 => "1".cyan(),
          2 => "2".green(),
          4 => "3".yellow(),
          _ => n.to_string().red()
        },
        Tile::Empty => " ".normal(),
      }
    )
  }
}