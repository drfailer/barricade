/* Board:
        O         111..112
ooooooooooooooooo 94..110
o               o 92..93
ooooooooooooooooo 75..91
        o         74
      ooooo       69..73
      o   o       67..68
    ooooooooo     58..66
    o       o     56..57
  ooooooooooooo   43..55
  o   o   o   o   39..42
ooooooooooooooooo 22..38
o   o   o   o   o 17..21
ooooooooooooooooo 0..16
 */

macro_rules! new_cell {
    ( $x:expr, $y:expr, $neigbours_num:expr, $neigbours:expr ) => {
        Cell {
            position: Pos($x, $y),
            neigbours_num: $neigbours_num,
            neigbours: $neigbours
        }
    };
}

macro_rules! new_barricade {
    ( $x:expr, $y:expr ) => {
        Piece::new($x, $y, Colors::White)
    };
}

#[derive(Debug)]
struct Pos(i32, i32);

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

// should be attach to a cell
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
        Player { piecies: [
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
    number_of_player: u8,
    players: [Player; 4],
    barricades: [Piece; 11], // board: [Pos; 112],
    board: [Cell; 112],
}

impl Barricade {
    pub fn new(number_of_player: u8) -> Self {
        Barricade {
            number_of_player: number_of_player,

            // creating the players
            players: [
                Player::new(Colors::Blue),
                Player::new(Colors::Red),
                Player::new(Colors::Yellow),
                Player::new(Colors::Green),
            ],
            // creating the barricades
            barricades: [ // {{{
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
            ], // }}}
            board: [ // {{{
                new_cell!(0, 0, 2, [1 , 17, 112]),
                new_cell!(1, 0, 2, [0 , 2, 112]),
                new_cell!(2, 0, 2, [1 , 3, 112]),
                new_cell!(3, 0, 2, [2, 4, 112]),
                new_cell!(4, 0, 3, [3, 5, 18]),
                new_cell!(5, 0, 2, [4, 6, 112]),
                new_cell!(6, 0, 2, [5, 7, 112]),
                new_cell!(7, 0, 2, [6, 6, 112]),
                new_cell!(8, 0, 3, [7, 9, 19]),
                new_cell!(9, 0, 2, [8, 10, 112]),
                new_cell!(10, 0, 2, [9, 11, 112]),
                new_cell!(11, 0, 2, [10, 12, 112]),
                new_cell!(12, 0, 3, [11, 13, 20]),
                new_cell!(13, 0, 2, [12, 14, 112]),
                new_cell!(14, 0, 2, [13, 15, 112]),
                new_cell!(15, 0, 2, [14, 16, 112]),
                new_cell!(16, 0, 2, [15, 21, 112]),

                new_cell!(0, 1, 2, [0, 22, 112]),
                new_cell!(4, 1, 2, [4, 26, 112]),
                new_cell!(8, 1, 2, [8, 30, 112]),
                new_cell!(12, 1, 2, [12, 34, 112]),
                new_cell!(16, 1, 2, [16, 38, 112]),

                new_cell!(0, 2, 2, [23, 17, 112]),
                new_cell!(1, 2, 2, [22, 24, 112]),
                new_cell!(2, 2, 3, [23, 25, 39]),
                new_cell!(3, 2, 2, [24, 26, 112]),
                new_cell!(4, 2, 3, [25, 27, 18]),
                new_cell!(5, 2, 2, [26, 28, 112]),
                new_cell!(6, 2, 3, [27, 29, 40]),
                new_cell!(7, 2, 2, [28, 30, 112]),
                new_cell!(8, 2, 3, [29, 31, 19]),
                new_cell!(9, 2, 2, [30, 32, 112]),
                new_cell!(10, 2, 3, [31, 33, 41]),
                new_cell!(11, 2, 2, [32, 34, 112]),
                new_cell!(12, 2, 3, [33, 35, 20]),
                new_cell!(13, 2, 2, [34, 36, 112]),
                new_cell!(14, 2, 3, [35, 37, 42]),
                new_cell!(15, 2, 2, [36, 38, 112]),
                new_cell!(16, 2, 2, [37, 21, 112]),

                new_cell!(2,  3, 2, [24, 43, 112]),
                new_cell!(6,  3, 2, [28, 47, 112]),
                new_cell!(10, 3, 2, [32, 51, 112]),
                new_cell!(14, 3, 2, [36, 55, 112]),

                new_cell!(2,  4, 2, [44, 39, 112]),
                new_cell!(3,  4, 2, [43, 45, 112]),
                new_cell!(4,  4, 3, [44, 46, 56]),
                new_cell!(5,  4, 2, [45, 47, 112]),
                new_cell!(6,  4, 3, [46, 48, 40]),
                new_cell!(7,  4, 2, [47, 49, 112]),
                new_cell!(8,  4, 2, [48, 50, 112]),
                new_cell!(9,  4, 2, [49, 51, 112]),
                new_cell!(10, 4, 3, [50, 52, 41]),
                new_cell!(11, 4, 2, [51, 53, 112]),
                new_cell!(12, 4, 3, [52, 54, 57]),
                new_cell!(13, 4, 2, [53, 55, 112]),
                new_cell!(14, 4, 2, [54, 42, 112]),

                new_cell!(4, 5, 2, [45, 58, 112]),
                new_cell!(12, 5, 2, [53, 66, 112]),

                new_cell!(4, 6, 2, [59, 56, 112]),
                new_cell!(5, 6, 2, [58, 60, 112]),
                new_cell!(6, 6, 3, [59, 61, 67]),
                new_cell!(7, 6, 2, [60, 62, 112]),
                new_cell!(8, 6, 2, [61, 63, 112]),
                new_cell!(9, 6, 2, [62, 64, 112]),
                new_cell!(10, 6, 3, [63, 65, 68]),
                new_cell!(11, 6, 2, [64, 66, 112]),
                new_cell!(12, 6, 2, [65, 57, 112]),

                new_cell!(6, 7, 2, [60, 67, 112]),
                new_cell!(10, 7, 2, [64, 68, 112]),

                new_cell!(6, 8, 2, [70, 67, 112]),
                new_cell!(7, 8, 2, [69, 71, 112]),
                new_cell!(8, 8, 3, [70, 72, 74]),
                new_cell!(9, 8, 2, [71, 73, 112]),
                new_cell!(10, 8, 2, [72, 68, 112]),

                new_cell!(8, 9, 2, [71, 83, 112]),

                new_cell!(0, 10, 2, [76, 92, 112]),
                new_cell!(1, 10, 2, [75, 77, 112]),
                new_cell!(2, 10, 2, [76, 78, 112]),
                new_cell!(3, 10, 2, [77, 79, 112]),
                new_cell!(4, 10, 2, [78, 80, 112]),
                new_cell!(5, 10, 2, [79, 81, 112]),
                new_cell!(6, 10, 2, [80, 82, 112]),
                new_cell!(7, 10, 2, [81, 83, 112]),
                new_cell!(8, 10, 3, [82, 84, 71]),
                new_cell!(9, 10, 2, [83, 85, 112]),
                new_cell!(10, 10, 2, [84, 86, 112]),
                new_cell!(11, 10, 2, [85, 87, 112]),
                new_cell!(12, 10, 2, [86, 88, 112]),
                new_cell!(13, 10, 2, [87, 89, 112]),
                new_cell!(14, 10, 2, [88, 90, 112]),
                new_cell!(15, 10, 2, [89, 91, 112]),
                new_cell!(16, 10, 2, [90, 93, 112]),

                new_cell!(0, 11, 2, [75, 94, 112]),
                new_cell!(16, 11, 2, [91, 110, 112]),

                new_cell!(0, 12, 2, [92, 95, 112]),
                new_cell!(1, 12, 2, [94, 96, 112]),
                new_cell!(2, 12, 2, [95, 97, 112]),
                new_cell!(3, 12, 2, [96, 98, 112]),
                new_cell!(4, 12, 2, [97, 99, 112]),
                new_cell!(5, 12, 2, [98, 100, 112]),
                new_cell!(6, 12, 2, [99, 101, 112]),
                new_cell!(7, 12, 2, [100, 102, 112]),
                new_cell!(8, 12, 2, [101, 103, 111]),
                new_cell!(9, 12, 2, [102, 104, 112]),
                new_cell!(10, 12, 2, [103, 105, 112]),
                new_cell!(11, 12, 2, [104, 106, 112]),
                new_cell!(12, 12, 2, [105, 107, 112]),
                new_cell!(13, 12, 2, [106, 108, 112]),
                new_cell!(14, 12, 2, [107, 109, 112]),
                new_cell!(15, 12, 2, [108, 110, 112]),
                new_cell!(16, 12, 2, [93, 109, 112]),

                new_cell!(17, 12, 2, [93, 110, 112])
                    ], // }}}
        }
    }
}

/*****************************************************************************/
/*                                   CELLS                                   */
/*****************************************************************************/

/* Construct a graph of cells to create the accessible cells of the board.
   The board is an array of cells and each cell store the index of its neigbours
   in the board.
*/
struct Cell {
    pub position: Pos,
    pub neigbours_num: u8,
    pub neigbours: [usize; 3],
}
