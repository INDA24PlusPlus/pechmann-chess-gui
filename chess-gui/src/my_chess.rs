#![allow(warnings)]


use std::usize;
use std::env;
use std::collections::HashMap;

use std::io::{self, BufRead};

pub struct Game {
    pub turn_counter: usize,
    pub board: Board,
    //from previous turn, for testing
    pub old_state: Board,
    pub last_capture: usize,
    pub max_repeated: usize,
    pub repeat_map:HashMap<[[Position; 8]; 8],usize>   
}

pub struct Board {
    pub positions: [[Position; 8]; 8],
    pub bk_pos:(usize,usize),
    pub wk_pos:(usize,usize),
    pub last_pass: (usize,usize,usize) //to-x,to-y,turn
}
#[derive(Copy, Clone,Debug,Hash,PartialEq,Eq)]
pub struct Position {
    pub content : PositionContent
}

#[derive(Copy, Clone,PartialEq,Debug,Hash,Eq)]
pub enum PositionContent {
    PIECE_CONT(Piece),
    NONE
}


impl PositionContent {
    fn print(self)-> char{
        match self {
            PositionContent::NONE => return ' ',
            PositionContent::PIECE_CONT(mut p) => return p.print()
        }
    }

    fn is_empty(self)-> bool{
        match self {
            PositionContent::NONE => return true,
            _ => return false
        }
    }

    pub fn get_piece(&mut self) -> Piece {
        match self {
            PositionContent::NONE => panic!("NO PIECES PRESENT"),
            PositionContent::PIECE_CONT(mut p) => return p
        }
    }   
}

#[derive(Copy, Clone,PartialEq,Debug,Eq,Hash)]
pub struct Piece {
    pub variant : PieceType,
    pub color : Color,
    pub has_moved : bool
}

#[derive(Clone, Copy,PartialEq,Debug)]
pub enum GameState {
    ONGOING,
    WIN_W,
    WIN_B,
    DRAW
}

impl Piece {
    fn print(&mut self) -> char {
        let ret_char: char = match self.variant {
            PieceType::PAWN =>  if self.color != Color::W {'♟' } else {'♙' },
            PieceType::KNIGHT =>  if self.color != Color::W { '♞'} else {'♘' }, 
            PieceType::BISHIOP => if self.color != Color::W {'♝' } else {'♗' },
            PieceType::ROOK =>  if self.color != Color::W {'♜' } else {'♖' },
            PieceType::QUEEN => if self.color != Color::W { '♛'} else {'♕'},
            PieceType::KING =>  if self.color != Color::W { '♚'} else { '♔'},
            _ =>  ' '
        };

        return ret_char;
    }
    //provides the longest possible path that each piece can take in an ideal scenario
    fn get_paths(self) -> Vec<(i16,i16)> {
        match self.variant {
            PieceType::PAWN => if self.color == Color::W {vec![(0,1)]} else {vec![(0,-1)]} ,
            PieceType::KNIGHT => vec![(1,2),(2,1),(-1,2),(2,-1),(-1,-2),(-2,-1),(1,-2),(-2,1)], 
            PieceType::BISHIOP => vec![(-1,1),(1,1),(-1,-1),(1,-1)],
            PieceType::ROOK => vec![(0,1),(-1,0),(1,0),(0,-1)],
            PieceType::QUEEN => vec![(-1,1),(0,1),(1,1),(-1,0),(1,0),(-1,-1),(0,-1),(1,-1)],
            PieceType::KING => vec![(-1,1),(0,1),(1,1),(-1,0),(1,0),(-1,-1),(0,-1),(1,-1)],
            _ => return vec![]
        }
    }
}
#[derive(Copy, Clone,PartialEq,Debug,Eq,Hash)]
pub enum PieceType {
    PAWN,
    KNIGHT,
    BISHIOP,
    ROOK,
    QUEEN,
    KING, 
    NONE
}


#[derive(Copy, Clone,PartialEq,Debug,Eq,Hash)]
pub enum Color {
    W,
    B
}

impl Color {
    pub fn get_inverted(self)->Color{
        match self {
            Color::W => Color::B,
            Color::B => Color::W
        }
    }
}

pub fn decode_notation(chess_not: &str)-> (usize,usize){
    let mut chars = chess_not.chars();
    let x_coord:char = chars.next().unwrap();
    let y_coord:char = chars.next().unwrap();

    let  x_converted:usize = match x_coord {
        'a'| 'A' =>  0,
        'b'| 'B' =>  1,
        'c'| 'C' =>  2,
        'd'| 'D' =>  3,
        'e'| 'E' =>  4,
        'f'| 'F' =>  5,
        'g'| 'G' =>  6,
        'h'| 'H' =>  7,
        _ => panic!()
    };

    let y_converted:usize = y_coord.to_digit(32).unwrap() as usize - 1;


    return (x_converted,y_converted);
}

pub fn decode_promotion(chess_not: &str)-> PieceType{
    let mut chars = chess_not.chars();
    let c_char:char = chars.next().unwrap();

    match c_char {
        'b'| 'B' => PieceType::BISHIOP,
        'r'| 'R' => PieceType::ROOK,
        'n'| 'N' => PieceType::KNIGHT,
        'q'| 'Q' => PieceType::QUEEN,
        'x'| 'X' => PieceType::NONE,
        _ => PieceType::NONE
    }
}

pub fn encode_notation(pos:(usize,usize)) -> String {

    let  ycoord: String = (pos.1+1).to_string();
    let xcoord = match  pos.0 {
        0 =>  "a",
        1 =>  "b",
        2 =>  "c",
        3 =>  "d",
        4 =>  "e",
        5 =>  "f",
        6 =>  "g",
        7 =>  "h",
        _ => panic!()
    };
    
    let mut encoded: String = String::from(xcoord);
    encoded.push_str(&ycoord);
    return encoded.to_string();
}

//modified fen, for testing
pub fn map_fen(fen:char)-> Position{
    let pos_c = PositionContent::NONE;
    let mut col = Color::W;
    let mut pc:PieceType = PieceType::PAWN;

    match fen {
        '.' => {return Position{content:pos_c}},
        'p' => {col=Color::B; pc=PieceType::PAWN},
        'k' => {col=Color::B; pc=PieceType::KING},
        'q' => {col=Color::B; pc=PieceType::QUEEN},
        'n' => {col=Color::B; pc=PieceType::KNIGHT},
        'b' => {col=Color::B; pc=PieceType::BISHIOP},
        'r' => {col=Color::B; pc=PieceType::ROOK}, 
        'P' => {pc=PieceType::PAWN},
        'K' => {pc=PieceType::KING},
        'Q' => {pc=PieceType::QUEEN},
        'N' => {pc=PieceType::KNIGHT},
        'B' => {pc=PieceType::BISHIOP},
        'R' => {pc=PieceType::ROOK}, 
        _=>panic!()
    }
    let pos_c = PositionContent::PIECE_CONT(Piece{variant:pc,color:col,has_moved:false});
    return Position{content:pos_c};
}

pub fn demap_fen(pos:Position)-> char {
    match pos.content {
        PositionContent::NONE => {return  '.'},
        PositionContent::PIECE_CONT(p) => {
            let mut c = 'l';
            match p.variant {
                PieceType::PAWN => {c='p'},
                PieceType::BISHIOP => {c='b'},
                PieceType::KNIGHT => {c='n'},
                PieceType::ROOK => {c='r'},
                PieceType::QUEEN => {c='q'},
                PieceType::KING => {c='k'},
                _=>panic!()
            }

            if p.color==Color::W {return c.to_ascii_uppercase()} else {return c}
        }
    }
}



impl Game {
    pub fn load_state(&mut self,state:String){
        let rows:Vec<String> =state.split('\n').map(|s| s.to_string()).collect();

        let mut j = 0;
        let mut i = 7;

        for row in rows {
            for place in row.chars() {
                if place =='k' {
                    self.board.bk_pos = (j,i);
                } else if place == 'K' {
                    self.board.wk_pos = (j,i);
                    
                }
                self.board.positions[j][i] = map_fen(place);
                j+=1;
            }
            j=0;
            if i>0{
                i-=1;
            }
        }

    }

    pub fn restart(&mut self) {
        self.board.reset();
        self.turn_counter = 1;
        self.last_capture = 0;
        self.max_repeated = 0;
        self.repeat_map.clear();
        self.old_state.reset();
    }

    //moves a piece from a position to a postion, returns true if successful, false if not.
    //from is the initial position, to is the destionation, promotion is required as it is used when promoting a pawn
    //valid values for promotion are: "Q","B","N","R","X" - "X" is a placeholder and should be used when moving but not promoting 
    pub fn move_piece(&mut self, from: &str, to: &str,promotion:&str) -> bool {

        if self.check_state() != GameState::ONGOING {
            return false
        } 

        let f_coords = decode_notation(from);
        let t_coords = decode_notation(to);

        let p_type = decode_promotion(promotion);

        //custom logic for pawn/king

        
        //get legal moves from that place
        let lg_moves = self.get_legal_moves(from);
        
        if lg_moves.contains(&t_coords) {

            let has_captured = self.board.force_move(f_coords, t_coords,p_type,self.turn_counter);

            if has_captured {
                self.last_capture = self.turn_counter; 
            }
            self.turn_counter += 1;
            //inserts the position hash into the map of positions and their amount
            let ret = self.repeat_map.insert(self.board.positions, 1);
            match ret {
                None => {},
                Some(val) => {
                    self.repeat_map.insert(self.board.positions, val+1);
                    if self.max_repeated < val +1 {
                        self.max_repeated = val + 1;
                    }
                }
            }

            return true
        }

        
        //else return false
            false
    }

    
    //returns all legal moves in the standard chess notation <letter><number>. First part is from, second is to. 
    pub fn get_all_legal_moves(&mut self) -> Vec<(String,String)> {
        let mut all_moves:Vec<(String,String)> = vec![];
        //for all PLACES on the board call get_legal_moves on pieces for the right color
        //yeah, not the best thing ever

        

        for i in 0..8 {
            for j in 0..8 {
                let mut pos = self.board.positions[j][i];

                if pos.content != PositionContent::NONE {
                    if pos.content.get_piece().color == Color::B && self.turn_counter % 2 == 0 {
                        let l_moves = self.get_legal_moves(encode_notation((j,i)).as_str());
                        let j_val = j;
                        let i_val = i;
                        let enc_from: &str = &encode_notation((j_val,i_val))[..];
                        let temp:Vec<(String,String)> = l_moves.iter().map(|m| (enc_from.to_string(),encode_notation(*m))).collect();
                        all_moves.extend(temp);

                    } else if pos.content.get_piece().color == Color::W && self.turn_counter % 2 == 1 {
                        let l_moves =self.get_legal_moves(encode_notation((j,i)).as_str());
                        let j_val = j;
                        let i_val = i; 
                        let enc_from: &str = &encode_notation((j_val,i_val))[..];
                        let temp:Vec<(String,String)> = l_moves.iter().map(|m| (enc_from.to_string(),encode_notation(*m))).collect();
                        all_moves.extend(temp);
                    };
                }
            }
        } 

        return all_moves;
    }

    //gives a vec of legal moves notation is (usize,usize) where the first place is the x-coordinate, and the other one the y. Can be converted
    //to the chess board notation by using encode_notation function.
    
    pub fn get_legal_moves(&mut self, from: &str) -> Vec<(usize,usize)> {
        let f_coords: (usize, usize) = decode_notation(from);
      
        let f_content: PositionContent = self.board.positions[f_coords.0][f_coords.1].content;
    
        match f_content {
            PositionContent::NONE => return vec![],
            PositionContent::PIECE_CONT( p) => {
                if p.color == Color::W && self.turn_counter % 2 == 0 {
                   
                    return vec![]  
                } else if p.color == Color::B && self.turn_counter % 2 == 1 {
                 
                    return vec![] 
                } else {
                    let paths: Vec<(i16, i16)> = p.get_paths();
    
                    let mut moves: Vec<(usize,usize)> = vec![];
                    match p.variant {
           
                        PieceType::BISHIOP|PieceType::ROOK|PieceType::QUEEN => {
                            for path in paths {
                                moves.append(&mut self.board.traverse_path(p, f_coords, path,10));
                            }
                        },
                        PieceType::KNIGHT => {
                            for path in paths {
                                moves.append(&mut self.board.traverse_path(p, f_coords, path,1));
                            } 
                        }
                        PieceType::KING => {
                            //normal moves
                            for path in paths {
                                moves.append(&mut self.board.traverse_path(p, f_coords, path,1));
                            }

                            //castle
                            if p.has_moved == false && self.board.is_in_check(f_coords) == false {
                                let mut rook_l: PositionContent = self.board.positions[0][f_coords.1].content;
                                let mut rook_r: PositionContent = self.board.positions[7][f_coords.1].content;
                                //check if the spot has a rook that has yet to have moved
                                if !rook_l.is_empty() && rook_l.get_piece().variant == PieceType::ROOK && rook_l.get_piece().has_moved == false {
                                   
                                    //check if the next 3 spots to the left are empty 
                                    if self.board.positions[f_coords.0-1][f_coords.1].content.is_empty() && self.board.positions[f_coords.0-2][f_coords.1].content.is_empty() && self.board.positions[f_coords.0-3][f_coords.1].content.is_empty() {
                                        //check if the next 1 spot to the left are valid (king cannot cross if any of that spots would give a check) 
                                       if self.board.is_legal(f_coords, (f_coords.0-1,f_coords.1),PieceType::QUEEN,self.turn_counter) {
                                            //can castle to the left
                                            moves.push((f_coords.0-2,f_coords.1));
                                       }

                                    }
                                }
                                if !rook_r.is_empty() && rook_r.get_piece().variant == PieceType::ROOK && rook_r.get_piece().has_moved == false {
                                   
                                    //check if the next 2 spots to the right are empty 
                                    if self.board.positions[f_coords.0+1][f_coords.1].content.is_empty() && self.board.positions[f_coords.0+2][f_coords.1].content.is_empty() {
                                        //check if the next 1 spot to the right are valid (king cannot cross if any of that spots would give a check) 
                                       if self.board.is_legal(f_coords, (f_coords.0+1,f_coords.1),PieceType::QUEEN,self.turn_counter) {
                                            //can castle to the right
                                            moves.push((f_coords.0+2,f_coords.1));
                                       }

                                    }
                                }
                            }
                        },
                        PieceType::PAWN => {
                            let dir:i16= if p.color == Color::W {1} else {-1};
                            if self.board.is_empty((f_coords.0,(f_coords.1 as i16 + dir) as usize)) {
                                moves.push((f_coords.0,(f_coords.1 as i16 + dir) as usize));
                            }
                            //can move forward, check if double move is possible
                            let dir:i16= if p.color == Color::W {2} else {-2};
                            if !moves.is_empty() && p.has_moved == false && self.board.is_empty((f_coords.0,(f_coords.1 as i16 + dir) as usize))  {
                                moves.push((f_coords.0,(f_coords.1 as i16 + dir) as usize));
                            }

                            //check diagonal capture
                            let dir = if p.color==Color::W {1} else {-1};

                            //check left/right side of pawn
                            for n in vec![-1,1] {
                                let c_coord: (i32, i32) = ((f_coords.0 as i32 + n),(f_coords.1 as i32 + dir));
                                //check if in bounds of board
                                if c_coord.0 >= 0 && c_coord.1 >= 0 && c_coord.0 <= 7 && c_coord.1 <= 7 {
                                    // if both non-empty&enemy, we can capture
                                    if !self.board.is_empty((c_coord.0 as usize, c_coord.1 as usize)) && self.board.get_piece((c_coord.0 as usize, c_coord.1 as usize)).color == p.color.get_inverted(){
                                        moves.push((c_coord.0 as usize,c_coord.1 as usize));
                                    }
                                }
                            }

                            //check NPASS
                            //if the last 2-p-push happened 1 turn ago, there is a possiblility
                            if self.turn_counter.abs_diff(self.board.last_pass.2) == 1 {
                                for n in vec![-1,1] {
                                    let c_coord: (i32, i32) = ((f_coords.0 as i32 + n),(f_coords.1 as i32));
                                    //check if in bounds of board
                                    if c_coord.0 >= 0 && c_coord.1 >= 0 && c_coord.0 <= 7 && c_coord.1 <= 7 {
                                        // if the coordinates of the landing match the field to the either side of the pawn, we have an NPASS 
                                        if self.board.last_pass.0 == c_coord.0 as usize && self.board.last_pass.1 == c_coord.1 as usize {
                                            
                                            let dir = if p.color==Color::W {1} else {-1};
                                            moves.push((c_coord.0 as usize,(c_coord.1 +dir) as usize));
                                        }
                                    }
                                }
                            }
                        }
                        _ => panic!()
                    } 
                    let mut to_retain = vec![];

                    for n in 0..moves.len() {
                        
                        //the queen is a placeholder in this case
                        if self.board.is_legal(f_coords, moves[n],PieceType::QUEEN,self.turn_counter) {
                            to_retain.push(moves[n]);
                        }
                        
                    }
                    
                    return to_retain;
                }
            }
        }
    }

    
    pub fn check_state(&mut self) -> GameState {
         //if 50moves since last capture -> draw
        //if pos repeated 3 times -> draw
        //else false
        if self.turn_counter - self.last_capture > 50 || self.max_repeated >= 3 {
            return GameState::DRAW
        } 

        //check who is to move
        //if !draw and color who is moving does not have any legal moves -> opposite color wins 
        if self.get_all_legal_moves().is_empty() {
            if self.turn_counter % 2 == 0 {
                //black moves 
                if self.board.is_in_check(self.board.bk_pos) {
                    return GameState::WIN_W;
                } else {
                    return GameState::DRAW;
                }

            } else {
                //white moves
                if self.board.is_in_check(self.board.wk_pos) {
                    return GameState::WIN_B;
                } else {
                    return GameState::DRAW;
                }
            }
        }
        return GameState::ONGOING;
    }

    //returns the chessboard as a string
    pub fn get_board_representation(&mut self) -> String {
        return self.board.to_string();
    }



}

impl Board {
    //add back pieces to the vec
    pub fn reset(&mut self) {
        self.wk_pos=(4,0);
        self.bk_pos=(4,7);
        for i in 0..8 {
            for j in 0..8 {
                match j {
                    //white figures
                    0 => {
                        let figure = match i {
                            0 =>  PieceType::ROOK,
                            1 =>  PieceType::KNIGHT,
                            2 =>  PieceType::BISHIOP,
                            3 =>  PieceType::QUEEN,
                            4 =>  PieceType::KING,
                            5 =>  PieceType::BISHIOP,
                            6 =>  PieceType::KNIGHT,
                            7 =>  PieceType::ROOK,
                            _ => panic!()
                        };

                        self.positions[i][j] = Position {
                            content : PositionContent::PIECE_CONT(Piece { 
                                variant : figure,
                                color : Color::W,
                                has_moved: false
                            })
                        };

                    },
                    //pawns white
                    1 => {
                        self.positions[i][j] = Position {
                            content : PositionContent::PIECE_CONT(Piece { 
                                variant : PieceType::PAWN,
                                color : Color::W,
                                has_moved: false
                            })
                        };
                    },
                    
                    //pawns black
                    6 => {
                        self.positions[i][j] = Position {
                            content : PositionContent::PIECE_CONT(Piece { 
                                variant : PieceType::PAWN,
                                color : Color::B,
                                has_moved: false
                            })
                        };
                    },
                    
                    //black figures
                    7 => {  
                        let figure = match i {
                            0 =>  PieceType::ROOK,
                            1 =>  PieceType::KNIGHT,
                            2 =>  PieceType::BISHIOP,
                            3 =>  PieceType::QUEEN,
                            4 =>  PieceType::KING,
                            5 =>  PieceType::BISHIOP,
                            6 =>  PieceType::KNIGHT,
                            7 =>  PieceType::ROOK,
                            _ => panic!()
                        };

                        self.positions[i][j] = Position {
                            content : PositionContent::PIECE_CONT(Piece { 
                                variant : figure,
                                color : Color::B,
                                has_moved: false
                            })
                        };
                    },

                    _ => {
                        self.positions[i][j] = Position {
                            content : PositionContent::NONE
                        };
                    }
                }
            }
        }
        
      
    }
    
    // fn print_state(&mut self) {
    //     let nums = vec!["1","2","3","4","5","6","7","8"];
    //     let lettrs = vec!["a","b","c","d","e","f","g","h"];
        
    //     for i in (0..8).rev() {
    //         print!("{} ",nums[i]);
    //         for j in 0..8 {
    //                             let c_piece = self.positions[j][i].content.print();
    //             let mut prt_str:String = c_piece.to_string();
    //             prt_str.push(' ');
    //             match (j+i)%2 {
    //                 0 => print!("{}",prt_str.on_color("cyan")),
    //                 1 => print!("{}",prt_str.on_color("white")),
    //                 _ => panic!()
    //             }
                
    //         }
    //         println!("");
    //     }
    //     print!(" ");
    //     for lettr in lettrs{
    //         print!(" {}",lettr);
    //     }
    //     println!();
    // }
    
    pub fn is_in_check(&mut self,king:(usize,usize))->bool{
        //color of the king
        let c: Color = self.positions[king.0][king.1].content.get_piece().color;

        //diagonal checks
        let mut pc: Piece = Piece{variant:PieceType::BISHIOP,color:c.get_inverted(),has_moved:true};
        let paths: Vec<(i16, i16)> = pc.get_paths();

        for b_path in paths {
            let res = self.trace_ray(c, king, b_path, 8);
            //if not empty and queen or bishop (enemy) then the king must be in check
            if !res.is_empty() && (self.get_piece(res[0]).variant == PieceType::QUEEN || self.get_piece(res[0]).variant == PieceType::BISHIOP)   {
                return true
            }
        }        

        //horizontal checks
        pc.variant = PieceType::ROOK;
        let paths: Vec<(i16, i16)> = pc.get_paths();

        for r_path in paths {
            let res = self.trace_ray(c, king, r_path, 8);
            //if not empty and queen or rook (enemy) then the king must be in check
            if !res.is_empty() && (self.get_piece(res[0]).variant == PieceType::QUEEN || self.get_piece(res[0]).variant == PieceType::ROOK)   {
                return true
            }
        }

        //knight checks
        pc.variant = PieceType::KNIGHT;
        let paths: Vec<(i16, i16)> = pc.get_paths();

        for r_path in paths {
            let res = self.trace_ray(c, king, r_path, 1);
            //if not empty and knight (enemy) then the king must be in check
            if !res.is_empty() && self.get_piece(res[0]).variant == PieceType::KNIGHT   {
                return true
            }
        }

        //king checks
        pc.variant = PieceType::KING;
        let paths: Vec<(i16, i16)> = pc.get_paths();

        for r_path in paths {
            let res = self.trace_ray(c, king, r_path, 1);
            //if not empty and knight (enemy) then the king must be in check
            if !res.is_empty() &&  self.get_piece(res[0]).variant == PieceType::KING   {
                return true
            }
        }

        //check for pawns
        //if white then the pawn must be 1 below the king, else 1 above
        let dir = if c==Color::B {-1} else {1};

        //check left/right side of king
        for n in vec![-1,1] {
            let c_coord: (i32, i32) = ((king.0 as i32 + n),(king.1 as i32 + dir));
            //check if in bounds of board
            if c_coord.0 >= 0 && c_coord.1 >= 0 && c_coord.0 <= 7 && c_coord.1 <= 7 {
                // if both non-empty,enemy and PAWN then king is in check
                if !self.is_empty((c_coord.0 as usize, c_coord.1 as usize)) && self.get_piece((c_coord.0 as usize, c_coord.1 as usize)).color == c.get_inverted()  && self.get_piece((c_coord.0 as usize, c_coord.1 as usize)).variant == PieceType::PAWN{
                    return true;
                }
            }
        }

            
        return false;
    }
    
    //helper function for my sanity, can panic
    pub fn get_piece(&mut self, from:(usize,usize))->Piece{
        return self.positions[from.0][from.1].content.get_piece();
    }

    pub fn is_legal(&mut self, from:(usize,usize),to:(usize,usize),promotion:PieceType,t_count:usize)-> bool{
        let pos_copy: [[Position; 8]; 8] = self.positions;
        let mut cpy_board: Board = Board {positions:pos_copy, bk_pos:self.bk_pos,wk_pos:self.wk_pos,last_pass:self.last_pass};
        let c = self.positions[from.0][from.1].content.get_piece().color;
        
        cpy_board.force_move(from, to, promotion,t_count);

        //check legality of the move by checking for a check

        return !cpy_board.is_in_check(if c == Color::B {cpy_board.bk_pos} else {cpy_board.wk_pos});
    }

    //helper functions
    fn is_empty(&mut self, coords:(usize,usize))-> bool{
        let pos: Position = self.positions[coords.0][coords.1];
        match pos.content {
            PositionContent::NONE => return true,
            _ => return false
        }
    }

    fn is_enemy(&mut self, coords:(usize,usize),team_col:Color)-> bool{
        let pos: Position = self.positions[coords.0][coords.1];
        match pos.content {
            PositionContent::NONE => panic!("This is not meant to be used this way"),
            PositionContent::PIECE_CONT(p) => {
                return p.color!=team_col;
            }
        }
    }
    //moves pieces, it is assumed that the move is viable but check has to be verified afterwards
    fn force_move(&mut self,from:(usize,usize),to:(usize,usize),promotion:PieceType,t_count:usize) -> bool{
        let has_captured:bool = self.is_empty(to);

        //will panic if trying to move NONE
        let moved_piece = self.positions[from.0][from.1].content.get_piece();

        match moved_piece.variant {
            PieceType::KING => {
                //castle attempt
                if (to.0 as i16 - from.0 as i16).abs() > 1 {
                    self.positions[to.0][to.1].content = PositionContent::PIECE_CONT(Piece{variant:moved_piece.variant,color:moved_piece.color,has_moved:true});
                    //left or right
                    if to.0 > from.0 {
                        //right,move rook to the left of king
                        self.positions[to.0-1][to.1].content = PositionContent::PIECE_CONT(Piece{variant:PieceType::ROOK,color:moved_piece.color,has_moved:true});
                        self.positions[7][to.1].content = PositionContent::NONE;  
                        self.positions[from.0][from.1].content = PositionContent::NONE;

                    } else {
                        //left
                        self.positions[to.0+1][to.1].content = PositionContent::PIECE_CONT(Piece{variant:PieceType::ROOK,color:moved_piece.color,has_moved:true});
                        self.positions[0][to.1].content = PositionContent::NONE; 
                        self.positions[from.0][from.1].content = PositionContent::NONE; 

                    }

                } else {
                    self.positions[to.0][to.1].content = PositionContent::PIECE_CONT(Piece{variant:moved_piece.variant,color:moved_piece.color,has_moved:true});
                    self.positions[from.0][from.1].content = PositionContent::NONE;  
                }
                //update king tracker
                if self.get_piece(to).color == Color::W {
                    self.wk_pos = (to.0,to.1);
                } else {
                    self.bk_pos = (to.0,to.1);
                }
            },
            PieceType::PAWN => {
                //attempts to move diagonally to an empty square => NPASS
                if from.0 != to.0 && self.positions[to.0][to.1].content == PositionContent::NONE {
                    self.positions[to.0][to.1].content = PositionContent::PIECE_CONT(Piece{variant:moved_piece.variant,color:moved_piece.color,has_moved:true});
                    //eliminates the piece (PAWN) that was to its side when starting the move
                    self.positions[to.0][from.1].content = PositionContent::NONE;
                    self.positions[from.0][from.1].content = PositionContent::NONE;

                    //goes to the end of the board in y-axis => PROMO
                } else if to.1 == 0 || to.1 == 7 {
                    if promotion == PieceType::NONE {panic!()}

                    self.positions[to.0][to.1].content = PositionContent::PIECE_CONT(Piece{variant:promotion,color:moved_piece.color,has_moved:true});
                    self.positions[from.0][from.1].content = PositionContent::NONE;
                    //normal movement
                } else {
                    self.positions[to.0][to.1].content = PositionContent::PIECE_CONT(Piece{variant:moved_piece.variant,color:moved_piece.color,has_moved:true});
                    self.positions[from.0][from.1].content = PositionContent::NONE;

                    //moved two spaces, keep track to see if NPASS is viable next turn
                    if from.1.abs_diff(to.1) > 1 {
                        self.last_pass = (to.0,to.1,t_count);
                    }
                }
            },
            _ => {
                self.positions[to.0][to.1].content = PositionContent::PIECE_CONT(Piece{variant:moved_piece.variant,color:moved_piece.color,has_moved:true});
                self.positions[from.0][from.1].content = PositionContent::NONE;  
            },
        }
        return has_captured;
    }

    fn traverse_path(&mut self, by:Piece, from:(usize,usize),path:(i16,i16), length:usize) -> Vec<(usize,usize)>{     
        //list of all viable moves
        let mut possible_moves:Vec<(usize,usize)> = Vec::new(); 

        

        let mut curr_x =from.0 as i16;
        let mut curr_y =from.1 as i16;
        let mut p_travelled =0;

        while p_travelled<length{
            curr_x += path.0;
            curr_y += path.1;
            //out of bounds on x
            if curr_x<0 ||  curr_x>7 {
                break;
            }
            //out of bounds on y
            if curr_y<0 || curr_y>7 {
                break;
            }

            if self.is_empty((curr_x as usize,curr_y as usize)){
                possible_moves.push((curr_x as usize,curr_y as usize));
            } else {
                if self.is_enemy((curr_x as usize,curr_y as usize),by.color){
                    possible_moves.push((curr_x as usize,curr_y as usize));
                }
                break;
                
            }
            p_travelled +=1;

        }
        return possible_moves;
    }

    fn trace_ray(&mut self, by:Color, from:(usize,usize),path:(i16,i16),length:usize) -> Vec<(usize,usize)>{

        //first obstacle hit by ray
        let mut first_obstacle:Vec<(usize,usize)> = Vec::new(); 

        

        let mut curr_x =from.0 as i16;
        let mut curr_y =from.1 as i16;
        let mut p_travelled =0;

        while p_travelled<length{
            curr_y += path.0;
            curr_x += path.1;
            //out of bounds on x
            if curr_x<0 ||  curr_x>7 {
                break;
            }
            //out of bounds on y
            if curr_y<0 || curr_y>7 {
                break;
            }

            //if the field is not empty and not an ally then we have hit an enemy
            if !self.is_empty((curr_x as usize,curr_y as usize)){
                if self.is_enemy((curr_x as usize,curr_y as usize),by){
                    first_obstacle.push((curr_x as usize,curr_y as usize));
                }
                break;
            }
            
            p_travelled +=1;
        }
        return first_obstacle;
    }

    
    fn to_string(&mut self) -> String {
        let mut str = String::new();
        for i in 0..8 {
            for j in 0..8 {
                let pos = self.positions[j][i];
                let c:char = demap_fen(pos);
                str.push(c);
            }
            str.push('/');
        }
        str.pop();
        return str;
    }

}

//creates the game object that is used to interact with the game
pub fn start() -> Game {
    let mut g:Game = Game{repeat_map:HashMap::new(),board:Board{last_pass:(0,0,0),positions: [[Position{content:PositionContent::NONE}; 8]; 8],wk_pos:(4,0),bk_pos:(4,7)},
    old_state:Board{last_pass:(0,0,0),positions: [[Position{content:PositionContent::NONE}; 8]; 8],wk_pos:(4,0),bk_pos:(4,7)},
    turn_counter:1,last_capture:0,max_repeated:0};

    g.restart();

    return g;
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }

    #[test]
    fn stalemate(){
        let mut gam = start();
        
        gam.move_piece("e2", "e3", "x");
        gam.move_piece("a7", "a5", "x");
        gam.move_piece("d1", "h5", "x");
        gam.move_piece("a8", "a6", "x");
        gam.move_piece("h5", "a5", "x");
        gam.move_piece("h7", "h5", "x");
        gam.move_piece("h2", "h4", "x");
        gam.move_piece("a6", "h6", "x");
    
        gam.move_piece("a5", "c7", "x");
        gam.move_piece("f7", "f6", "x");
        gam.move_piece("c7", "d7", "x");
        gam.move_piece("e8", "f7", "x");
    
        gam.move_piece("d7", "b7", "x");
        gam.move_piece("d8", "d3", "x");
        gam.move_piece("b7", "b8", "x");
        gam.move_piece("d3", "h7", "x");
    
        gam.move_piece("b8", "c8", "x");
        gam.move_piece("f7", "g6", "x");
        gam.move_piece("c8", "e6", "x");

        let moves = gam.get_all_legal_moves();
        let state = gam.check_state();

        assert_eq!(moves, vec![]);
        assert_eq!(state,GameState::DRAW);

    }

    #[test]
    fn repetition(){
        let mut gam = start();
        
        gam.move_piece("a2", "a4", "x");
        gam.move_piece("a7", "a5", "x");
        gam.move_piece("a1", "a3", "x");
        gam.move_piece("a8", "a6", "x");

        gam.move_piece("a3", "h3", "x");
        gam.move_piece("a6", "h6", "x");
        gam.move_piece("h3", "a3", "x");
        gam.move_piece("h6", "a6", "x");

        gam.move_piece("a3", "h3", "x");
        gam.move_piece("a6", "h6", "x");
        gam.move_piece("h3", "a3", "x");
        gam.move_piece("h6", "a6", "x");

        print!("REP{:?}",gam.max_repeated);

        // let moves = gam.get_all_legal_moves();
        // let state = gam.check_state();
        // assert_eq!(moves, vec![]);
        // assert_eq!(state,GameState::DRAW);
        


    }

    #[test]
    fn castle(){
        let mut gam = start();
        
        gam.move_piece("a2", "a4", "x");
        gam.move_piece("a7", "a5", "x");
        gam.move_piece("b1", "a3", "x");
        gam.move_piece("a8", "a6", "x");
         
        gam.move_piece("d2", "d4", "x");
        gam.move_piece("a6", "a8", "x");
        gam.move_piece("c1", "h6", "x");
        gam.move_piece("a8", "a6", "x");

        gam.move_piece("d1", "d3", "x");
        gam.move_piece("a6", "a8", "x");
        gam.move_piece("b6", "b5", "x");
        gam.move_piece("e1", "c1", "x");

        print!("REP{:?}",gam.max_repeated);

        let moves = gam.get_all_legal_moves();
        let state = gam.check_state();
       
        
        let representation = gam.get_board_representation();
        let correct_state = "..KR.BNR/.PP.PPPP/N..Q..../P..P..../p......./.......B/.ppppppp/rnbqkbnr";
        print!("{:?}",representation);
        assert_eq!(correct_state,representation);


    }

    #[test]
    fn check_representation(){
        let mut gam = start();
        


        let representation = gam.get_board_representation();
        let strt_moves = gam.get_all_legal_moves();
        
        let correct_state = "RNBQKBNR/PPPPPPPP/......../......../......../......../pppppppp/rnbqkbnr";

        assert_eq!(correct_state,representation);
        


    }

    
    #[test]
    fn enpassant(){
        let mut gam = start();
        
        gam.move_piece("a2", "a4", "x");
        gam.move_piece("a7", "a6", "x");
        gam.move_piece("a4", "a5", "x");
        gam.move_piece("b7", "b5", "x");

        //en passant 
        gam.move_piece("a5", "b6", "x");

        print!("REP{:?}",gam.max_repeated);

        let state = gam.check_state();
       
        
        let representation = gam.get_board_representation();
        let correct_state = "RNBQKBNR/.PPPPPPP/......../......../......../pP....../..pppppp/rnbqkbnr";
        assert_eq!(state,GameState::ONGOING);
        assert_eq!(correct_state,representation);


    }
    #[test]
    fn promotion(){
        let mut gam = start();
        
        gam.move_piece("a2", "a4", "x");
        gam.move_piece("a7", "a6", "x");
        gam.move_piece("a4", "a5", "x");
        gam.move_piece("b7", "b5", "x");

        //en passant 
        gam.move_piece("a5", "b6", "x");
        gam.move_piece("a8", "a7", "x");
        gam.move_piece("b6", "a7", "x");
        gam.move_piece("c8", "b7", "x");
        //promotion
        gam.move_piece("a7", "b8", "Q");

        print!("REP{:?}",gam.max_repeated);

        let state = gam.check_state();
       
        
        let representation = gam.get_board_representation();
        let correct_state = "RNBQKBNR/.PPPPPPP/......../......../......../p......./.bpppppp/.Q.qkbnr";
        
        assert_eq!(state,GameState::ONGOING);
        assert_eq!(correct_state,representation);


    }

    #[test]
    fn checkmate(){
        let mut gam = start();
        
        gam.move_piece("f2", "f3", "x");
        gam.move_piece("e7", "e6", "x");
        gam.move_piece("g2", "g4", "x");
        
        //fool's mate
        gam.move_piece("d8", "h4", "x");



        print!("REP{:?}",gam.max_repeated);

        let state = gam.check_state();
       
        
        let representation = gam.get_board_representation();
        let correct_state = "RNBQKBNR/PPPPP..P/.....P../......Pq/......../....p.../pppp.ppp/rnb.kbnr";

        assert_eq!(state,GameState::WIN_B);
        assert_eq!(correct_state,representation);


    }

}
