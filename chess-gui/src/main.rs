use chesslib::*;
use ggez::*;
use graphics::{Canvas, Color, Image, Rect, Text};
use mint::Point2;
use std::{ops::Div, path};



const GRID_SIZE: (usize,usize) = (8,8);
const GRID_CELL_SIZE: (usize,usize) = (64,64);

const SCREEN_SIZE: (f32,f32) = ((GRID_SIZE.0*GRID_CELL_SIZE.0) as f32 ,(GRID_SIZE.1*GRID_CELL_SIZE.1) as f32);
const OFFSET_X_P:usize = 2;
const OFFSET_Y_P:usize = 2;
const OFFSET_X_H:usize = 32;
const OFFSET_Y_H:usize = 32;
const OFFSET_X_T:usize = 0;
const OFFSET_Y_T:usize = 0;
const OFFSET_X_BT:usize = 55;
const OFFSET_Y_BT:usize = 50;

const FPS: usize = 10;

enum off {
    P,
    H,
    T,
    BT
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
    board:chesslib::Chess,
    state:State,
    to_move:Side,
    pieces: Vec<RendPiece>,
    highlights: Vec<MoveHighlight>,
    assets:Assets,
    from:Option<GridPosition>

}

impl BoardState {

    fn new(ctx: &mut Context) -> Self {
        let board = chesslib::Chess::new();
        let pieces = RendPiece::convert_pieces(board.get_all_pieces());
        let assets = Assets::new(ctx);


        return BoardState{assets:assets.unwrap(),board:board,state:State::Playing, to_move:Side::White,pieces:pieces,highlights:vec![],from:None};
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
            let state = self.board.get_state();
            
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
            draw_text(GridPosition::new(i, 0), letters[i].to_string(), &mut canvas, off::BT);
            draw_text(GridPosition::new(0, i), numbers[i].to_string(), &mut canvas, off::T);
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
            let mut promo_p = RendPiece {pos:GridPosition::new(2, 4),variant:PieceType::Queen,color:self.board.get_playing_side().get_opposite()};
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
        
        
        canvas.finish(ctx)?;
        Ok(())
    }

    

    fn mouse_button_up_event(&mut self,ctx: &mut Context,button: input::mouse::MouseButton,x: f32,y: f32,) -> GameResult {

        

        let x_coord = (8*x as usize).div(SCREEN_SIZE.0 as usize);
        let y_coord = 7-(8*y as usize).div(SCREEN_SIZE.1 as usize);

        let grid_pos = GridPosition::new(x_coord, y_coord);

        if self.board.get_state() == State::Promotion {
            //grid positions: Q(2,4),R(3,4),B(4,4),N(5,4)     
            match (x_coord,y_coord) {
                (2,4) => self.board.promote(PieceType::Queen),
                (3,4) => self.board.promote(PieceType::Rook),
                (4,4) => self.board.promote(PieceType::Bishop),
                (5,4) => self.board.promote(PieceType::Knight),
                _=>{return Ok(());}
            }
            self.pieces = RendPiece::convert_pieces(self.board.get_all_pieces());       
            return Ok(());
        }

        let is_valid = self.board.is_selectable(grid_pos.map_to_bitboard());
        if is_valid {
            let pos_moves = self.board.get_moves(grid_pos.map_to_bitboard());
            self.highlights = MoveHighlight::to_highlights(pos_moves);
            self.from = Some(grid_pos);
            return Ok(());
        }


        
        match self.from {
            Some(res) => {
                let made_move =self.board.make_move(res.map_to_bitboard(), grid_pos.map_to_bitboard());
                if made_move {
                    self.pieces = RendPiece::convert_pieces(self.board.get_all_pieces());
                }
                self.highlights = vec![];
                self.from = None;
            },
            None => {
            }
        }
        
        Ok(())
    }

}

fn draw_text(mut coord:GridPosition,text:String,canvas: &mut Canvas,txt_type:off) {
    let mut txt = Text::new(text);
    canvas.draw(&txt, graphics::DrawParam::default().dest(coord.map_to_coords(txt_type)).color(Color::BLACK));
}

fn main() -> GameResult {

    let resource_dir = path::PathBuf::from("D:/GitHubProjects/prog_kth/chess/pechmann-chess-gui/chess-gui/src/resources");
    
    

    let c = conf::Conf::new();
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("chess_gui", "pechmann")
        .window_setup(ggez::conf::WindowSetup::default().title("Chess"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .add_resource_path(resource_dir)
        .build()?;

    let state = BoardState::new(&mut ctx);
    
    event::run(ctx, event_loop, state);
}
