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
    board: [Cell; 212],
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

    /* Construct the graph of the map. */
    fn construct_board() -> [Cell; 212] {
        let mut arr: [Cell; 212];
        let mut arr_index: usize;
        let mut x: i32;
        let mut y: i32;
        let map: [238; u8] = [
            1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
            1,0,0,0,1,0,0,0,1,0,0,0,1,0,0,0,1,
            1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
            0,0,1,0,0,0,1,0,0,0,1,0,0,0,1,0,0,
            0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,
            0,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0,
            0,0,0,0,1,1,1,1,1,1,1,1,1,0,0,0,0,
            0,0,0,0,0,0,1,0,0,0,1,0,0,0,0,0,0,
            0,0,0,0,0,0,1,1,1,1,1,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,
            1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
            1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
            1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
            0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0
        ];

        // init arr
        for i in 0..212 {
            arr[i] = new_cell!(-1, -1, 0, [-1,-1,-1])
        }

        // TODO: update que les voisins de droite, puis voisins haut et bas une ligne sur 2
        // PS: si trop chiant faire à la main ou via fichier

        for i in 0..237 {
            if map[i] == 1 { // update neigbours up and right
                arr[arr_index].pos = Pos((i%17).try_into().unwrap(), (i/17).try_into().unwrap());
                if map[arr_index+1] == 1 && i < 16 { // right
                    arr[arr_index+1].neigbours[arr[arr_index+1].neigbours_num] = arr_index;
                    arr[arr_index+1].neigbours_num += 1;
                    arr[arr_index].neigbours[arr[arr_index].neigbours_num] = arr_index + 1;
                    arr[arr_index].neigbours_num += 1;
                }

                if map[i+17] == 1 && i < 221 { // up
                    // c'est ma merde :(
                }
                arr_index += 1;
            }
        }


















        // first line
        for i in 0..17_i32 {
            if i%4 != 0 {
                arr[(i as usize)] = new_cell!(i, 0, 2, [i-1, i+1, -1]);
            }
            else {
                arr[(i as usize)] = new_cell!(i, 0, 3, [i-1, 17 + i, i+1]);
            }
        }

        // second line
        let mut j = 0;
        for i in 17..22_i32 {
            arr[(i as usize)] = new_cell!(i, 1, 2, [j, 17*2+j, -1]);
            j += 4;
        }

        arr
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
    pub neigbours: [i32; 3],
}
