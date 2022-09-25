/* Board:
        O
ooooooooooooooooo
o               o
ooooooooooooooooo
        o
      ooooo
      o   o
    ooooooooo
    o       o
  ooooooooooooo
  o   o   o   o
ooooooooooooooooo
o   o   o   o   o
ooooooooooooooooo
 */

macro_rules! new_barricade {
    ( $x:expr, $y:expr ) => {
        Piece::new($x, $y, Colors::White)
    };
}

#[derive(Debug)]
struct Pos(i32, i32);

/**
 * Caracteraize the different states of the game.
 */
enum GameStates {
    Starting,
    Running,
    Paused,
    Ending,
}

/**
 * Colors of the piecies.
 * Empty means that there is no pieces in the position.
 */
#[derive(Clone, Copy)]
enum Colors {
    Blue,
    Red,
    Yellow,
    Green,
    White,
}

struct Piece {
    position: Pos,
    color: Colors,
}

impl Piece {
    pub fn new(x: i32, y: i32, c: Colors) -> Self {
        Piece {
            position: Pos(x, y),
            color: c,
        }
    }
}

struct Player {
    piecies: [Piece; 5],
    color: Colors,
}

impl Player {
    pub fn new(c: Colors) -> Self {
        Player {
            piecies: [
                Piece::new(-1, -1, c),
                Piece::new(-1, -2, c),
                Piece::new(-1, -3, c),
                Piece::new(-1, -4, c),
                Piece::new(-1, -5, c),
            ],
            color: c,
        }
    }
}

pub struct Barricade {
    current_state: GameStates,
    number_of_player: u8,
    players: [Player; 4],
    barricades: [Piece; 11], // board: [Pos; 112],
}

impl Barricade {
    pub fn new(number_of_player: u8) -> Self {
        Barricade {
            current_state: GameStates::Starting,
            number_of_player: number_of_player,

            // creating the players
            players: [
                Player::new(Colors::Blue),
                Player::new(Colors::Red),
                Player::new(Colors::Yellow),
                Player::new(Colors::Green),
            ],
            // creating the barricades
            barricades: [
                new_barricade!(0, 2),
                new_barricade!(4, 2),
                new_barricade!(8, 2),
                new_barricade!(12, 2),
                new_barricade!(16, 2),
                new_barricade!(6, 6),
                new_barricade!(10, 6),
                new_barricade!(8, 8),
                new_barricade!(8, 9),
                new_barricade!(8, 10),
                new_barricade!(8, 12),
            ],
        }
    }
}
