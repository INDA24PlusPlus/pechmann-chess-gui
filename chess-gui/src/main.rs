#![allow(warnings)]
use chesslib::*;
use ggez::*;
use graphics::{Canvas, Color, Image, Rect, Text};
use mint::Point2;
use std::{ops::Div, path,env};

//Imports my original chess library (slighlty modified for the sake of compatibility with my GUI)
//The GUI implementation uses a wrapper so that I can easily swap between my own backend and the one that I'm using for the assignment
// #[path = "./my_chess.rs"]
// mod my_chess;


const GRID_SIZE: (usize,usize) = (8,8);
const GRID_CELL_SIZE: (usize,usize) = (64,64);
const BOARD_SIZE: (usize,usize) = ((GRID_SIZE.0*GRID_CELL_SIZE.0) ,(GRID_SIZE.1*GRID_CELL_SIZE.1));

const SCREEN_SIZE: (f32,f32) = ((GRID_SIZE.0*GRID_CELL_SIZE.0) as f32 ,(GRID_SIZE.1*GRID_CELL_SIZE.1) as f32);
const OFFSET_X_P:usize = 2;
const OFFSET_Y_P:usize = 2;
const OFFSET_X_H:usize = 32;
const OFFSET_Y_H:usize = 32;
const OFFSET_X_T:usize = 0;
const OFFSET_Y_T:usize = 0;
const OFFSET_X_BT:usize = 55;
const OFFSET_Y_BT:usize = 50;

const FPS: usize = 120;

enum off {
    P,
    H,
    T,
    BT
}


// struct GameWrapper {
//     game: my_chess::Game,
//     promo_from: Option<GridPosition>,
//     promo_to: Option<GridPosition>,
//     is_promotion: bool
// }

// impl GameWrapper {
//     fn make_move(&mut self,from:GridPosition,to:GridPosition)->bool{
//         if self.is_promotion {return false};


//         if (to.y == 7 || to.y == 0) && self.game.board.get_piece(from.to_tup()).variant == my_chess::PieceType::PAWN {
//             self.is_promotion = true;
//             self.promo_from = Some(from);
//             self.promo_to = Some(to);
//             return false;
//         }
        
//         self.promo_from = None;
//         self.promo_to = None;
//         let enc_from = my_chess::encode_notation(from.to_tup());
//         let enc_to = my_chess::encode_notation(to.to_tup());

//         return self.game.move_piece(&enc_from, &enc_to, "X");
//     }

//     fn promote(&mut self,promotion:chesslib::PieceType)->bool{
//         let promotion_str: &str = match promotion {
//             PieceType::Pawn => "P",
//             PieceType::King => "K",
//             PieceType::Queen => "Q",
//             PieceType::Rook => "R",
//             PieceType::Bishop => "B",
//             PieceType::Knight => "N",
//             PieceType::Empty => "X",
//         };
//         let enc_from = my_chess::encode_notation(self.promo_from.unwrap().to_tup());
//         let enc_to = my_chess::encode_notation(self.promo_to.unwrap().to_tup());
//         self.game.move_piece(&enc_from, &enc_to, promotion_str);
//         self.is_promotion = false;
//         self.promo_from = None;
//         self.promo_to = None;
//         return true;
//     }

//     fn get_state(&mut self) -> State {
//         if self.is_promotion {return State::Promotion};
        
//         match self.game.check_state() {
//             my_chess::GameState::DRAW => {return State::Draw},
//             my_chess::GameState::ONGOING => {return State::Playing;},
//             my_chess::GameState::WIN_B | my_chess::GameState::WIN_W => {return State::Checkmate;}
//         }
//     }


//     fn get_turn(&mut self) -> Side{
//         let side = match self.game.turn_counter % 2 {
//             0 => Side::Black,
//             1 => Side::White,
//             _ => panic!()
//         };
//         return side;
//     }
//     fn get_all_pieces(&mut self) -> Vec<RendPiece> {
//         let mut unt_pieces = self.game.board.positions;
        
//         let mut pieces: Vec<RendPiece> = vec![];

//         for i in 0..8 {
//             for j in 0..8 {
//                 let field: my_chess::PositionContent  = unt_pieces[j][i].content;
//                 match field {
//                     my_chess::PositionContent::NONE => {},
//                     my_chess::PositionContent::PIECE_CONT(p) => {
//                         let color = match p.color {
//                             my_chess::Color::B => Side::Black,
//                             my_chess::Color::W => Side::White
//                         };
//                         let variant = match p.variant {
//                               my_chess::PieceType::PAWN => {chesslib::PieceType::Pawn},
//                               my_chess::PieceType::KING => {chesslib::PieceType::King},
//                               my_chess::PieceType::QUEEN => {chesslib::PieceType::Queen},
//                               my_chess::PieceType::ROOK => {chesslib::PieceType::Rook},
//                               my_chess::PieceType::BISHIOP => {chesslib::PieceType::Bishop},
//                               my_chess::PieceType::KNIGHT => {chesslib::PieceType::Knight},
//                               _=>panic!()
//                         };

//                         pieces.push(RendPiece::new(j, i, variant, color));
//                     }
//                 }
//             }
//         }

//         return pieces;
//     }
//     fn get_moves(&mut self, from:GridPosition) -> Vec<MoveHighlight> {
        
//         let enc_from = my_chess::encode_notation(from.to_tup());
//         let mut moves = self.game.get_legal_moves(&enc_from);

//         let mut high = vec![];

//         for lmove in moves {
//             high.push(MoveHighlight::new(lmove.0, lmove.1));
//         }

//         return high;
//     }
//     fn is_selectable(&mut self,from:GridPosition) ->bool {
//         let enc_from = my_chess::encode_notation(from.to_tup());
//         return !self.game.get_legal_moves(&enc_from).is_empty();
//     }
//     fn new()->Self {
//         return GameWrapper{game:my_chess::start(),promo_from:None,promo_to:None,is_promotion:false};
//     }
// }

struct alt_GameWrapper {
    game: Chess
}

impl alt_GameWrapper {

    fn make_move(&mut self,from:GridPosition,to:GridPosition)->bool{
        return self.game.make_move(from.map_to_bitboard(),to.map_to_bitboard());
    }

    fn promote(&mut self,promotion:chesslib::PieceType)->bool{
        self.game.promote(promotion);
        return true;
    }

    fn get_state(&mut self) -> State {
        return self.game.get_state();
    }

    fn get_turn(&mut self) -> Side{
        if self.get_state() == State::Promotion {return self.game.get_playing_side().get_opposite() } else {return self.game.get_playing_side()}
        ;
    }
    fn get_all_pieces(&mut self) -> Vec<RendPiece> {
        let mut pieces =RendPiece::convert_pieces(self.game.get_all_pieces());
        return pieces;
    }
    fn get_moves(&mut self, from:GridPosition) -> Vec<MoveHighlight> {
        let mut moves = self.game.get_moves(from.map_to_bitboard());

        let mut high = vec![];

        for lmove in moves {
            high.push(MoveHighlight::new(lmove.0, lmove.1));
        }

        return high;
    }
    fn is_selectable(&mut self,from:GridPosition) ->bool {
        return self.game.is_selectable(from.map_to_bitboard());
    }
    fn new()->Self {
        return alt_GameWrapper{game:chesslib::Chess::new()};
    }

}



struct Assets {
    RB: graphics::Image,
    RW: graphics::Image,
    QB: graphics::Image,
    QW: graphics::Image,
    KB: graphics::Image,
    KW: graphics::Image,
    PB: graphics::Image,
    PW: graphics::Image,
    BB: graphics::Image,
    BW: graphics::Image,
    NB: graphics::Image,
    NW: graphics::Image,
    //move_sound:audio::Source,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let PB: Image=  graphics::Image::from_path(ctx, "/PB.png")?;
        let PW: Image=  graphics::Image::from_path(ctx, "/PW.png")?;
        let KB: Image=  graphics::Image::from_path(ctx, "/KB.png")?;
        let KW: Image=  graphics::Image::from_path(ctx, "/KW.png")?;
        let QB: Image=  graphics::Image::from_path(ctx, "/QB.png")?;
        let QW: Image=  graphics::Image::from_path(ctx, "/QW.png")?;
        let RB: Image=  graphics::Image::from_path(ctx, "/RB.png")?;
        let RW: Image=  graphics::Image::from_path(ctx, "/RW.png")?;
        let BB: Image=  graphics::Image::from_path(ctx, "/BB.png")?;
        let BW: Image=  graphics::Image::from_path(ctx, "/BW.png")?;
        let NB: Image=  graphics::Image::from_path(ctx, "/NB.png")?;
        let NW: Image=  graphics::Image::from_path(ctx, "/NW.png")?;
        


        Ok(Assets {
            RB,
            RW,
            QB,
            QW,
            KB,
            KW,
            PB,
            PW,
            BB,
            BW,
            NB,
            NW
        })
    }

    fn get_image(&mut self,piece:RendPiece) -> &Image {
        match (piece.color, piece.variant) {
            (Side::Black,PieceType::Pawn) => &self.PB,
            (Side::White,PieceType::Pawn) => &self.PW,
            (Side::Black,PieceType::King) => &self.KB,
            (Side::White,PieceType::King) => &self.KW,
            (Side::Black,PieceType::Queen) => &self.QB,
            (Side::White,PieceType::Queen) => &self.QW,
            (Side::Black,PieceType::Rook) => &self.RB,
            (Side::White,PieceType::Rook) => &self.RW,
            (Side::Black,PieceType::Bishop) => &self.BB,
            (Side::White,PieceType::Bishop) => &self.BW,
            (Side::Black,PieceType::Knight) => &self.NB,
            (Side::White,PieceType::Knight) => &self.NW,
            _ => panic!("No matching image")
        }
    }

}



#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct GridPosition {
    x: usize,
    y: usize
}


impl GridPosition {
    fn new(x:usize,y:usize)-> Self {
        return GridPosition {x,y}
    }

    fn to_tup(self) -> (usize,usize) {
        return (self.x,self.y);
    }

    fn map_to_coords(&mut self,tp:off) -> Point2<f32> {
        
        let offset = match tp {
            off::H => (OFFSET_X_H,OFFSET_Y_H),
            off::P => (OFFSET_X_P,OFFSET_Y_P),
            off::T => (OFFSET_X_T,OFFSET_Y_T),
            off::BT => (OFFSET_X_BT,OFFSET_Y_BT)
        };

        let x = (self.x*GRID_CELL_SIZE.0 +offset.0) as f32;
        let y = ((7-self.y)*GRID_CELL_SIZE.1+offset.1) as f32;
        return Point2 { x: x, y: y }
    }

    fn map_to_bitboard(self)->usize{
        return self.x + self.y*8; 
    }

}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct RendPiece {
    pos: GridPosition,
    variant: PieceType,
    color: Side
}

impl RendPiece {
    fn new(x:usize,y:usize,variant:PieceType,color:Side)-> Self {
        return RendPiece {
            pos: GridPosition::new(x, y),
            variant:variant,
            color:color
        }
    }

    fn move_piece(&mut self,x:usize,y:usize){
        self.pos.x = x;
        self.pos.y = y;
    }

    

    fn render(&mut self, canvas:&mut Canvas,assets:&mut Assets){
        let image = assets.get_image(*self);
    
        let drawparams = graphics::DrawParam::new().dest(self.pos.map_to_coords(off::P));
        canvas.draw(image, drawparams);

        //  canvas.draw(drawable, param);
    }

    fn convert_pieces(def_pieces:Vec<Piece>) -> Vec<RendPiece> {
        let mut v = vec![];
        for def_piece in def_pieces{
            let col = def_piece.get_color();
            let x = def_piece.get_occupied_slot() % 8;
            let y = (def_piece.get_occupied_slot() -x)/8;
            let tp = def_piece.get_piece_type();
            v.push(RendPiece::new(x, y,tp,col));
        }
        return v;
    }

}

struct BoardState {
    wrap:alt_GameWrapper,
    state:State,
    to_move:Side,
    pieces: Vec<RendPiece>,
    highlights: Vec<MoveHighlight>,
    assets:Assets,
    from:Option<GridPosition>,
    in_restart_box:bool,
    reset_flag:bool

}

impl BoardState {

    fn new(ctx: &mut Context) -> Self {
        let mut wrap = alt_GameWrapper::new();
        let pieces = wrap.get_all_pieces();
        let assets = Assets::new(ctx);
        return BoardState{assets:assets.unwrap(),wrap:wrap,state:State::Playing, to_move:Side::White,pieces:pieces,highlights:vec![],from:None,in_restart_box:false,reset_flag:false};
    }

    fn reset(&mut self){
        let mut wrap = alt_GameWrapper::new();
        let pieces = wrap.get_all_pieces();
        self.highlights = vec![];
        self.wrap = wrap;
        self.to_move = Side::White;
        self.state= State::Playing;
        self.pieces=pieces;
        self.from=None;
        self.in_restart_box=false;
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct MoveHighlight {
    pos: GridPosition
}

impl MoveHighlight {
    fn new(x:usize,y:usize) -> Self {
        return MoveHighlight{pos:GridPosition::new(x, y)};
    }
    fn render(&mut self, canvas: &mut Canvas,ctx: &mut Context){


        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.pos.map_to_coords(off::H),
            8.0,
            0.1,
            graphics::Color::from([0.5, 0.5, 0.5, 0.8]),
        ).unwrap();
    
        canvas.draw(&circle, graphics::DrawParam::default());
    }
    fn to_highlights(coords:Vec<(usize,usize)>)->Vec<MoveHighlight>{
        let mut highlights = vec![];
        
        for coord in coords {
            highlights.push(MoveHighlight::new(coord.0, coord.1));
        }
        return highlights;
    }
}



impl ggez::event::EventHandler<GameError> for BoardState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(FPS as u32) {
            let state = self.wrap.get_state();
            
            if self.reset_flag {
                self.reset();
                self.reset_flag = false;
                return Ok(());
            }

            //possibly not needed
            //TODO update last move after playing
            match state {
                State::Playing | State::Check => {
                    self.state = state;
                },
                State::Draw | State::Checkmate | State::Stalemate => {self.state = state},
                State::Promotion => {self.state = state},
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 0.0]));
        // print!("STATE: {:?}",self.wrap.get_state());
        

        let letters = vec!["a","b","c","d","e","f","g","h"];
        let numbers = vec!["1","2","3","4","5","6","7","8"];
        

        for i in 0..8 {
            for j in 0..8 {
                let col  = if (j+i) % 2 == 1 {graphics::Color::from([0.0, 1.0, 1.0, 0.8])} else {graphics::Color::WHITE};
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(Rect::new((GRID_CELL_SIZE.0*j) as f32, (GRID_CELL_SIZE.1*i) as f32, GRID_CELL_SIZE.0 as f32, GRID_CELL_SIZE.1 as f32))
                        .color(col),
                );
                
            }
            
        }

        for i in 0..8 {
            draw_text(GridPosition::new(i, 0), letters[i].to_string(), &mut canvas, off::BT,Color::BLACK);
            draw_text(GridPosition::new(0, i), numbers[i].to_string(), &mut canvas, off::T,Color::BLACK);
        }

        for piece in &self.pieces {
            let mut t_piece = *piece;
            t_piece.render(&mut canvas, &mut self.assets);
        }

        for highlight in &self.highlights {
            let mut t_highlight = *highlight;
            t_highlight.render(&mut canvas,ctx);
            
        }

        if self.state == State::Promotion {
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(Rect::new((GRID_CELL_SIZE.0*2) as f32, (GRID_CELL_SIZE.1*3) as f32, 4 as f32 *GRID_CELL_SIZE.0 as f32, GRID_CELL_SIZE.1 as f32))
                    .color(graphics::Color::from([0.0,1.0,1.0,0.7])),
            );
            let mut promo_p = RendPiece {pos:GridPosition::new(2, 4),variant:PieceType::Queen,color:self.wrap.get_turn()};
            promo_p.render(&mut canvas, &mut self.assets);
            promo_p.variant = PieceType::Rook;
            promo_p.pos.x += 1;
            promo_p.render(&mut canvas, &mut self.assets);
            promo_p.variant = PieceType::Bishop;
            promo_p.pos.x += 1;
            promo_p.render(&mut canvas, &mut self.assets);
            promo_p.variant = PieceType::Knight;
            promo_p.pos.x += 1;
            promo_p.render(&mut canvas, &mut self.assets);
        }
        
        match self.wrap.get_state() {
            State::Checkmate => {
                draw_rectangle(&mut canvas, 3, 3, 2, 2, Color::BLACK);
                let won = if self.wrap.get_turn() == Side::Black {"White "} else {"Black "};
                let mut won_str = won.to_string();
                won_str.push_str("Wins");

                if self.in_restart_box { 
                    draw_rectangle(&mut canvas, 3, 2, 2, 1, Color::from([0.4,0.4,0.4,1.0]));
                }
                
                draw_text(GridPosition::new(3, 2), "Play again?".to_string(), &mut canvas, off::P,Color::WHITE);

                draw_text(GridPosition::new(3, 3), won_str, &mut canvas, off::P,Color::WHITE);         
                canvas.finish(ctx)?;    
                return  Ok(());
            },
            State::Draw | State::Stalemate => {
                draw_rectangle(&mut canvas, 3, 3, 2, 2, Color::BLACK);
                draw_text(GridPosition::new(3, 3), "Draw".to_string(), &mut canvas, off::P,Color::WHITE);

                if self.in_restart_box { 
                    draw_rectangle(&mut canvas, 3, 2, 2, 1, Color::from([0.4,0.4,0.4,1.0]));
                }

                draw_text(GridPosition::new(3, 2), "Play again?".to_string(), &mut canvas, off::P,Color::WHITE);
                
                canvas.finish(ctx)?;
                return  Ok(());
            }, 
            _ => {}
        }  

        
        canvas.finish(ctx)?;
        Ok(())
    }

    

    fn mouse_button_up_event(&mut self,ctx: &mut Context,button: input::mouse::MouseButton,x: f32,y: f32,) -> GameResult {

        if is_in_box(x, y, 3, 2, 2, 1) && self.wrap.get_state() != State::Playing && self.wrap.get_state() != State::Promotion {
            self.reset_flag = true;
            return Ok(());
        } 

        let x_coord = (8*x as usize).div(SCREEN_SIZE.0 as usize);
        let y_coord = 7-(8*y as usize).div(SCREEN_SIZE.1 as usize);

        let grid_pos = GridPosition::new(x_coord, y_coord);

        if self.wrap.get_state() == State::Promotion {
            //grid positions: Q(2,4),R(3,4),B(4,4),N(5,4)     
            match (x_coord,y_coord) {
                (2,4) => {self.wrap.promote(PieceType::Queen);},
                (3,4) => {self.wrap.promote(PieceType::Rook);},
                (4,4) => {self.wrap.promote(PieceType::Bishop);},
                (5,4) => {self.wrap.promote(PieceType::Knight);},
                _=>{return Ok(());}
            }
            self.pieces = self.wrap.get_all_pieces();       
            return Ok(());
        }

        if self.wrap.get_state() != State::Playing {
            return Ok(());
        }

        let is_valid = self.wrap.is_selectable(grid_pos);
        if is_valid {
            self.highlights = self.wrap.get_moves(grid_pos);
            self.from = Some(grid_pos);
            return Ok(());
        }


        
        match self.from {
            Some(res) => {
                let made_move =self.wrap.make_move(res, grid_pos);
                if made_move {
                    self.pieces = self.wrap.get_all_pieces();
                }
                self.highlights = vec![];
                self.from = None;
            },
            None => {
            }
        }
        
        Ok(())
    }

    fn mouse_motion_event(&mut self,_ctx: &mut Context,x: f32,y: f32,_dx: f32,_dy: f32,) -> Result<(), GameError> {
        
        //check if in box
        if is_in_box(x, y, 3, 2, 2, 1) {
            self.in_restart_box = true;
        } else {
            self.in_restart_box = false;
        }

        Ok(())
    }

}

fn draw_rectangle(canvas: &mut Canvas,x:usize,y:usize,w:usize,h:usize,color:Color){
    
    canvas.draw(
        &graphics::Quad,
        graphics::DrawParam::new()
            .dest_rect(Rect::new((GRID_CELL_SIZE.0*x) as f32, (GRID_CELL_SIZE.1*(7-y)) as f32, w as f32 *GRID_CELL_SIZE.0 as f32, h as f32 * GRID_CELL_SIZE.1 as f32))
            .color(color),
    );
}

fn is_in_box(x:f32,y:f32,bx:usize,by:usize,bwidth:usize,bheight:usize) -> bool {
    let abs_x = (bx*GRID_CELL_SIZE.0) as f32;
    let abs_y = ((7-by)*GRID_CELL_SIZE.1) as f32;
    let box_end_x = abs_x + (bwidth*GRID_CELL_SIZE.0) as f32;
    let box_end_y = abs_y + (bheight*GRID_CELL_SIZE.1) as f32;

    if abs_x < x && x < box_end_x {
        if abs_y < y && y < box_end_y {
            return true;
        }
    } 
    false
}

fn draw_text(mut coord:GridPosition,text:String,canvas: &mut Canvas,txt_type:off,color:Color) {
    let mut txt = Text::new(text);
    canvas.draw(&txt, graphics::DrawParam::default().dest(coord.map_to_coords(txt_type)).color(color));
}

fn main() -> GameResult {

    // let resource_dir = path::PathBuf::from("D:/GitHubProjects/prog_kth/chess/pechmann-chess-gui/chess-gui/src/resources");
    
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("D:/GitHubProjects/prog_kth/chess/pechmann-chess-gui/chess-gui/src/resources")
    };

    
    print!("DIRDIRDIR: {:?}",resource_dir);

    let c = conf::Conf::new();
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("chess_gui", "pechmann")
        .window_setup(ggez::conf::WindowSetup::default().title("Chess"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .add_resource_path(resource_dir)
        .build()?;

    let state = BoardState::new(&mut ctx);
    
    event::run(ctx, event_loop, state);
}


