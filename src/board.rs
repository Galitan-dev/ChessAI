use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::BufReader,
    thread,
    time::Duration,
};

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
use rodio::{Decoder, OutputStream, Source};

use crate::piece::Piece;

const FRONT_ROW: [Piece; 8] = [Piece::Pawn; 8];
const BACK_ROW: [Piece; 8] = [
    Piece::Rook,
    Piece::LeftKnight,
    Piece::Bishop,
    Piece::Queen,
    Piece::King,
    Piece::Bishop,
    Piece::RightKnight,
    Piece::Rook,
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opponent {
    Player,
    Computer,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    Playing,
    Stalemate,
    Checkmate,
    Dead,
}

#[derive(Clone)]
pub struct Board {
    pieces: [Piece; 64],
    selected: Option<usize>,
    dragging: bool,
    last_move: [usize; 2],
    moved_pieces: HashSet<usize>,
    legal_moves: HashMap<usize, Vec<[usize; 2]>>,
    current_turn: Piece,
    white_opponent: Opponent,
    black_opponent: Opponent,
    rng: ThreadRng,
    flying_piece: Option<(usize, [f64; 2], usize)>,
    square_in_promotion: Option<usize>,
    status: Status,
    pieces_locations: HashMap<Piece, Vec<usize>>,
}

impl Board {
    pub fn get_piece_maybe(&self, x: isize, y: isize) -> Option<Piece> {
        if x < 0 || y < 0 {
            return None;
        }

        self.pieces.get((y * 8 + x) as usize).copied()
    }

    pub fn get_piece(&self, x: usize, y: usize) -> Piece {
        self.pieces[y * 8 + x]
    }

    pub fn is_selected(&self, x: usize, y: usize) -> bool {
        self.selected == Some(y * 8 + x)
    }

    pub fn is_dragging(&self) -> bool {
        self.dragging
    }

    pub fn get_last_move(&self) -> [usize; 2] {
        self.last_move
    }

    pub fn get_selected(&self) -> Piece {
        self.selected.map(|i| self.pieces[i]).unwrap_or(Piece::None)
    }

    pub fn is_in_promotion(&self, x: usize, y: usize) -> bool {
        self.square_in_promotion == Some(y * 8 + x)
    }

    pub fn get_selected_piece_legal_moves(&mut self) -> Vec<[usize; 2]> {
        self.selected
            .map(|i| self.get_legal_moves(i))
            .unwrap_or_default()
    }

    fn get_sub_legal_moves(&mut self, square_index: usize) -> Vec<[usize; 2]> {
        self.pieces[square_index].legal_moves(
            [
                square_index % 8,
                (square_index as f64 / 8.).floor() as usize,
            ],
            self,
        )
    }

    fn get_legal_moves(&mut self, square_index: usize) -> Vec<[usize; 2]> {
        if let Some(legal_moves) = self.legal_moves.get(&square_index) {
            return legal_moves.clone();
        }

        let legal_moves: Vec<[usize; 2]> = self
            .get_sub_legal_moves(square_index)
            .iter()
            .filter(|[x, y]| {
                let mut board = self.clone();
                board.force_move_piece(square_index, *y * 8 + *x, true);
                !board.is_check(self.current_turn)
            })
            .map(|m| m.clone())
            .collect();

        self.legal_moves.insert(square_index, legal_moves.clone());

        legal_moves
    }

    fn get_legal_moves_square_indices(&mut self, square_index: usize) -> Vec<usize> {
        self.get_legal_moves(square_index)
            .iter()
            .map(|[x, y]| y * 8 + x)
            .collect()
    }

    fn get_sub_legal_moves_square_indices(&mut self, square_index: usize) -> Vec<usize> {
        self.get_sub_legal_moves(square_index)
            .iter()
            .map(|[x, y]| y * 8 + x)
            .collect()
    }

    fn get_all_legal_moves(&mut self) -> Vec<[usize; 2]> {
        let mut legal_moves = Vec::new();

        for from in 0..64 {
            if self.pieces[from].color() == self.current_turn {
                for to in self.get_legal_moves_square_indices(from) {
                    legal_moves.push([from, to])
                }
            }
        }

        legal_moves
    }

    fn get_all_sub_legal_moves(&mut self) -> Vec<[usize; 2]> {
        let mut legal_moves = Vec::new();

        for from in 0..64 {
            if self.pieces[from].color() == self.current_turn {
                for to in self.get_sub_legal_moves_square_indices(from) {
                    legal_moves.push([from, to])
                }
            }
        }

        legal_moves
    }

    fn is_check(&mut self, color: Piece) -> bool {
        let king = self
            .pieces_locations
            .get(&(Piece::King | color))
            .unwrap()
            .first()
            .unwrap()
            .clone();

        self.get_all_sub_legal_moves()
            .iter()
            .find(|[_, to]| *to == king)
            .is_some()
    }

    pub fn is_in_last_move(&self, x: usize, y: usize) -> bool {
        if self.last_move[0] == self.last_move[1] {
            return false;
        }

        let square_index = y * 8 + x;
        self.last_move[0] == square_index || self.last_move[1] == square_index
    }

    pub fn piece_has_moved(&self, x: usize, y: usize) -> bool {
        self.moved_pieces.contains(&(y * 8 + x))
    }

    pub fn current_opponent(&self) -> Opponent {
        if self.current_turn.is_white() {
            self.white_opponent
        } else {
            self.black_opponent
        }
    }

    pub fn mouse_press(&mut self, mouse_x: f64, mouse_y: f64) {
        if self.current_opponent() != Opponent::Player {
            return;
        }

        let x = (mouse_x * 8.).floor() as usize;
        let y = (mouse_y * 8.).floor() as usize;
        let square_index = y * 8 + x;

        if self.square_in_promotion.is_some() {
            if self.is_in_promotion(x, y) {
                let realtive_x = mouse_x * 8. - x as f64;
                let realtive_y = mouse_y * 8. - y as f64;
                match [
                    (realtive_x * 2.).floor() as usize,
                    (realtive_y * 2.).floor() as usize,
                ] {
                    [0, 0] => self.promote(Piece::Queen),
                    [1, 0] => self.promote(Piece::Rook),
                    [0, 1] => self.promote(Piece::Bishop),
                    [1, 1] => self.promote(Piece::LeftKnight),
                    combination => panic!("unexpected combination: {:?}", combination),
                }
            }
        }

        if let Some(selected) = self.selected {
            self.selected = None;
            if selected != square_index {
                self.move_piece(selected, square_index);
            }
        } else {
            if self.pieces[square_index].color() != self.current_turn {
                return;
            }

            self.selected = Some(square_index);
            self.dragging = true;
        }
    }

    pub fn mouse_relase(&mut self, mouse_x: f64, mouse_y: f64) {
        if self.current_opponent() != Opponent::Player {
            return;
        }

        let x = (mouse_x * 8.).floor() as usize;
        let y = (mouse_y * 8.).floor() as usize;
        let square_index = y * 8 + x;

        self.dragging = false;
        if let Some(selected) = self.selected {
            if selected != square_index {
                self.move_piece(selected, square_index);
                self.selected = None;
            }
        }
    }

    pub fn move_piece(&mut self, from: usize, to: usize) {
        if self.get_legal_moves_square_indices(from).contains(&to) {
            self.force_move_piece(from, to, false);
        }

        let legal_moves = self.get_all_legal_moves();
        if legal_moves.len() == 0 {
            if self.is_check(self.current_turn) {
                self.status = Status::Checkmate
            } else {
                self.status = Status::Stalemate
            }
        }

        if self.square_in_promotion.is_none() {
            let remaining_pieces = self
                .pieces_locations
                .iter()
                .filter(|(piece, _)| !piece.is_none() && piece.split().0 != Piece::King)
                .map(|(piece, locations)| locations.iter().map(move |_| piece.clone()))
                .flatten()
                .collect::<Vec<_>>();

            if remaining_pieces.len() == 0
                || (remaining_pieces.len() == 1
                    && [Piece::Bishop, Piece::LeftKnight, Piece::RightKnight]
                        .contains(&remaining_pieces.first().unwrap().split().0))
            {
                self.status = Status::Dead
            }

            if self.status != Status::Playing {
                println!("{:?}", self.status);
            }
        }
    }

    pub fn force_move_piece(&mut self, from: usize, to: usize, silent: bool) {
        let (piece, _) = self.pieces[from].split();

        let is_little_castle = piece == Piece::King && to == from + 2;
        let is_big_castle = piece == Piece::King && to + 2 == from;
        let is_en_passant = piece == Piece::Pawn && to % 8 != from % 8 && self.pieces[to].is_none();
        let is_promotion = piece == Piece::Pawn && [0., 7.].contains(&(to as f64 / 8.).floor());

        let mut ate = !self.pieces[to].is_none();

        let piece_locations = self.pieces_locations.get_mut(&self.pieces[from]).unwrap();
        piece_locations.remove(piece_locations.iter().position(|i| *i == from).unwrap());
        piece_locations.push(to);
        if !self.pieces[to].is_none() {
            let piece_locations = self.pieces_locations.get_mut(&self.pieces[to]).unwrap();
            piece_locations.remove(piece_locations.iter().position(|i| *i == to).unwrap());
        }

        self.pieces.swap(from, to);
        self.pieces[from] = Piece::None;
        self.last_move = [from, to];
        self.moved_pieces.insert(from);
        self.moved_pieces.insert(to);

        if is_little_castle {
            let piece_locations = self
                .pieces_locations
                .get_mut(&self.pieces[from + 3])
                .unwrap();
            piece_locations.remove(piece_locations.iter().position(|i| *i == from + 3).unwrap());
            piece_locations.push(from + 1);

            self.pieces.swap(from + 3, from + 1);
            self.moved_pieces.insert(from + 3);
        }
        if is_big_castle {
            let piece_locations = self
                .pieces_locations
                .get_mut(&self.pieces[from - 4])
                .unwrap();
            piece_locations.remove(piece_locations.iter().position(|i| *i == from - 4).unwrap());
            piece_locations.push(from - 1);

            self.pieces.swap(from - 4, from - 1);
            self.moved_pieces.insert(from - 4);
        }
        if is_en_passant {
            let piece_locations = self
                .pieces_locations
                .get_mut(&self.pieces[from + to % 8 - from % 8])
                .unwrap();
            piece_locations.remove(
                piece_locations
                    .iter()
                    .position(|i| *i == from + to % 8 - from % 8)
                    .unwrap(),
            );

            self.pieces[from + to % 8 - from % 8] = Piece::None;
            self.moved_pieces.insert(from + to % 8 - from % 8);
            ate = true;
        }

        if !silent {
            self.play_sound(if ate { "kill" } else { "move" });
        }
        self.legal_moves.drain();

        if is_promotion {
            let piece_locations = self.pieces_locations.get_mut(&self.pieces[to]).unwrap();
            piece_locations.remove(piece_locations.iter().position(|i| *i == to).unwrap());

            self.pieces[to] = Piece::None;
            self.square_in_promotion = Some(to);
        } else {
            self.current_turn = self.current_turn.ennemy();
        }
    }

    pub fn promote(&mut self, piece: Piece) {
        if let Some(square_in_promotion) = self.square_in_promotion {
            self.pieces_locations
                .get_mut(&(piece | self.current_turn))
                .unwrap()
                .push(square_in_promotion);
            self.pieces[square_in_promotion] = piece | self.current_turn;
            self.square_in_promotion = None;
            self.current_turn = self.current_turn.ennemy();
            self.play_sound("promotion");
        }
    }

    pub fn play_sound(&self, name: &'static str) {
        thread::spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();

            let assets = find_folder::Search::ParentsThenKids(3, 3)
                .for_folder("assets")
                .unwrap();
            let file = BufReader::new(
                File::open(assets.join("sound").join(name).with_extension("ogg")).unwrap(),
            );

            let source = Decoder::new(file).unwrap();
            stream_handle.play_raw(source.convert_samples()).unwrap();
            thread::sleep(Duration::from_secs(1));
        });
    }

    pub fn flying_piece(&self) -> Option<([usize; 2], [f64; 2], [usize; 2])> {
        self.flying_piece.map(|(from, current, to)| {
            (
                [from % 8, (from as f64 / 8.).floor() as usize],
                current,
                [to % 8, (to as f64 / 8.).floor() as usize],
            )
        })
    }

    pub fn update(&mut self, dt: Duration) {
        if self.status == Status::Playing && self.current_opponent() == Opponent::Computer {
            if self.square_in_promotion.is_some() {
                self.promote(
                    *[Piece::Queen, Piece::Rook, Piece::Bishop, Piece::LeftKnight]
                        .choose(&mut self.rng.clone())
                        .unwrap(),
                );
            } else if let Some((from, current, to)) = self.flying_piece {
                let start = [(from as f64 / 8.).floor(), from as f64 % 8.];
                let target = [(to as f64 / 8.).floor(), to as f64 % 8.];
                let dist_x = target[0] - current[0];
                let dist_y = target[1] - current[1];
                let dist = (dist_x.powi(2) + dist_y.powi(2)).sqrt();
                let total_dist =
                    ((target[0] - start[0]).powi(2) + (target[1] - start[1]).powi(2)).sqrt();
                let v = total_dist * 5.;
                let d = dt.as_secs_f64() * v;
                if d >= dist {
                    self.flying_piece = None;
                    self.move_piece(from, to);
                } else {
                    let dy = d * dist_y / dist;
                    let dx = d * dist_x / dist;
                    self.flying_piece = Some((from, [current[0] + dx, current[1] + dy], to));
                }
            } else {
                let legal_moves = self.get_all_legal_moves();
                if let Some([from, to]) = legal_moves.choose(&mut self.rng).copied() {
                    self.flying_piece =
                        Some((from, [(from as f64 / 8.).floor(), from as f64 % 8.], to));
                    self.last_move = [from, to];
                }
            }
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut pieces = [Piece::None; 64];

        let black_pieces: [Piece; 16] =
            concat_arrays!(BACK_ROW, FRONT_ROW).map(|p| p | Piece::Black);
        let white_pieces: [Piece; 16] =
            concat_arrays!(FRONT_ROW, BACK_ROW).map(|p| p | Piece::White);

        black_pieces.clone().swap_with_slice(&mut pieces[..16]);
        white_pieces.clone().swap_with_slice(&mut pieces[48..]);

        let mut pieces_locations = HashMap::new();
        for i in 0..64 {
            pieces_locations
                .entry(pieces[i])
                .or_insert(Vec::new())
                .push(i);
        }

        Self {
            pieces,
            selected: None,
            dragging: false,
            last_move: [0; 2],
            moved_pieces: HashSet::new(),
            legal_moves: HashMap::new(),
            current_turn: Piece::White,
            white_opponent: Opponent::Computer,
            black_opponent: Opponent::Computer,
            rng: thread_rng(),
            flying_piece: None,
            square_in_promotion: None,
            status: Status::Playing,
            pieces_locations,
        }
    }
}
