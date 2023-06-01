pub mod game {
    use std::fmt;

    /// Represents board coordinates `(row, col)`
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Coords(u8, u8);
    #[derive(Debug)]
    pub enum CoordsBuildError {
        OutOfBounds,
    }
    impl Coords {
        pub fn build(row: u8, col: u8) -> Result<Self, CoordsBuildError> {
            let bounds = 0..3;
            if !(bounds.contains(&row) && bounds.contains(&col)) {
                return Err(CoordsBuildError::OutOfBounds);
            }
            Ok(Self(row, col))
        }
    }

    #[derive(Debug)]
    pub struct Board([[Option<TileValue>; 3]; 3]);

    impl Board {
        pub fn new() -> Self {
            Self([[None; 3]; 3])
        }

        pub fn value_at_coords(&self, coords: &Coords) -> &Option<TileValue> {
            &self.0[coords.0 as usize][coords.1 as usize]
        }

        fn value_at_coords_mut(&mut self, coords: &Coords) -> &mut Option<TileValue> {
            &mut self.0[coords.0 as usize][coords.1 as usize]
        }

        pub fn set_tile(&mut self, coords: &Coords, value: &Option<TileValue>) {
            *self.value_at_coords_mut(coords) = *value;
        }
    }

    /// Represents one turn of Tic-Tac-Toe, with a player playing `value` at `coords`
    #[derive(Debug, PartialEq)]
    pub struct Turn {
        value: TileValue,
        coords: Coords,
    }

    impl Turn {
        pub fn new(value: TileValue, coords: Coords) -> Self {
            Self { value, coords }
        }
    }

    /// Represents and manages a game of Tic-Tac-Toe
    #[derive(Debug)]
    pub struct Game {
        board: Board,
        turn_history: Vec<Turn>,
        player_turn: TileValue,
        result: Option<GameResult>,
    }

    type TurnResult = Result<Option<GameResult>, TurnError>;

    impl Game {
        /// Initializes a new game with an empty board, no turns, and turn X
        pub fn new() -> Self {
            Self {
                board: Board::new(),
                turn_history: Vec::new(),
                player_turn: TileValue::X,
                result: None,
            }
        }

        /// Attempts to set the the tile at `turn.coords` to `turn.value`, and if
        /// the tile is already full then returns a `TurnError::TileFull` containing
        /// the `TileValue` that is already in the tile
        ///
        /// # Examples
        /// ```rust
        /// use ric_rac_roe_game::game::*;
        /// let mut g = Game::new();
        /// let value = TileValue::X;
        /// let coords = Coords::build(0, 0).expect("is in bounds");
        /// let turn = Turn::new(value, coords);
        /// assert!(g.take_turn(turn).expect("Should pass because [0,0] is open").is_none());
        /// let value2 = TileValue::O;
        /// let coords2 = Coords::build(0, 0).expect("is in bounds");
        /// let turn2 = Turn::new(value2, coords2);
        /// assert!(matches!(g.take_turn(turn2), Result::Err(TurnError::TileFull(TileValue::X))));
        /// ```
        pub fn take_turn(&mut self, turn: Turn) -> TurnResult {
            if let Some(x) = self.result {
                return Err(TurnError::GameOver(x));
            }
            let val_ref: &Option<TileValue> = self.board.value_at_coords(&turn.coords);
            if let Some(val) = *val_ref {
                Err(TurnError::TileFull(val))
            } else {
                self.board.set_tile(&turn.coords, &Some(turn.value));
                self.turn_history.push(turn);
                Ok(self.check_and_update_result())
            }
        }

        /// Plays a turn with `take_turn` using the internal current `player_turn` as the player
        /// # Examples
        /// ```rust
        /// use ric_rac_roe_game::game::*;
        /// use TileValue::*;
        /// let mut g = Game::new();
        /// let t1 = g.play_coords(Coords::build(0, 0).expect("is in bounds")).expect("Should not error, as tile [0,0] should be empty");
        /// let t2_first_try = g.play_coords(Coords::build(0, 0).expect("is in bounds"));
        /// assert!(matches!(t2_first_try, Err(TurnError::TileFull(TileValue::X))));
        /// let t2_second_try = g.play_coords(Coords::build(0, 2).expect("is in bounds")).expect("Tile [1,0] should be empty and open for O to go there");
        /// assert!(matches!(*g.board().value_at_coords(&Coords::build(0, 2).expect("is in bounds")), Some(TileValue::O)));
        /// let t3 = g.play_coords(Coords::build(2, 2).expect("is in bounds"));
        /// assert!(matches!(*g.board().value_at_coords(&Coords::build(2, 2).expect("is in bounds")), Some(TileValue::X)));
        /// let turns: Vec<Turn> = vec![
        ///     Turn::new(X, Coords::build(0, 0).expect("is in bounds")),
        ///     Turn::new(O, Coords::build(0, 2).expect("is in bounds")),
        ///     Turn::new(X, Coords::build(2, 2).expect("is in bounds"))
        /// ];
        /// assert!(g.turn_history().iter().eq(turns.iter()));
        /// ```
        pub fn play_coords(&mut self, coords: Coords) -> TurnResult {
            let result = self.take_turn(Turn {
                value: self.player_turn,
                coords,
            })?;
            self.player_turn = self.player_turn.toggle();
            Ok(result)
        }

        pub const WIN_LINES: [[Coords; 3]; 8] = [
            [
                Coords(0, 0),
                Coords(0, 1),
                Coords(0, 2),
            ],
            [
                Coords(1, 0),
                Coords(1, 1),
                Coords(1, 2),
            ],
            [
                Coords(2, 0),
                Coords(2, 1),
                Coords(2, 2),
            ],
            [
                Coords(0, 0),
                Coords(1, 0),
                Coords(2, 0),
            ],
            [
                Coords(0, 1),
                Coords(1, 1),
                Coords(2, 1),
            ],
            [
                Coords(0, 2),
                Coords(1, 2),
                Coords(2, 2),
            ],
            [
                Coords(0, 0),
                Coords(1, 1),
                Coords(2, 2),
            ],
            [
                Coords(0, 2),
                Coords(1, 1),
                Coords(2, 0),
            ],
        ];

        /// Checks if the current game is over, returning the (potential) result
        ///
        /// # Examples
        /// ```rust
        /// use ric_rac_roe_game::game::*;
        /// let mut g = Game::new();
        /// g.play_coords(Coords::build(0, 0).expect("is in bounds")).expect("This tile is open and the game is not over");
        /// g.play_coords(Coords::build(2, 0).expect("is in bounds")).expect("This tile is open and the game is not over");
        /// g.play_coords(Coords::build(1, 1).expect("is in bounds")).expect("This tile is open and the game is not over");
        /// g.play_coords(Coords::build(2, 1).expect("is in bounds")).expect("This tile is open and the game is not over");
        /// g.play_coords(Coords::build(2, 2).expect("is in bounds")).expect("This tile is open and the game is not over");
        /// assert!(matches!(g.check_end(), Some(GameResult::Winner(TileValue::X))));
        /// ```
        ///
        /// ```rust
        /// use ric_rac_roe_game::game::*;
        /// let mut g = Game::new();
        /// vec![
        ///     (1,1),
        ///     (0,0),
        ///     (1,2),
        ///     (1,0),
        ///     (0,2),
        ///     (2,0)
        /// ].iter().map(|c| -> Option<GameResult>{
        ///     g.play_coords(Coords::build(c.0, c.1).expect("is in bounds")).expect("This tile is open and the game is not over yet")
        /// }).collect::<Vec<_>>();
        /// let result = g.check_end();
        /// assert!(matches!(result, Some(GameResult::Winner(TileValue::O))));
        /// ```
        ///
        /// ```rust
        /// use ric_rac_roe_game::game::*;
        /// let mut g = Game::new();
        /// vec![
        ///     (1,1),
        ///     (0,2),
        ///     (2,2),
        ///     (0,0),
        ///     (0,1),
        ///     (2,1),
        ///     (1,0),
        ///     (1,2),
        ///     (2,0)
        /// ].iter().map(|c| -> Option<GameResult>{
        ///     g.play_coords(Coords::build(c.0, c.1).expect("is in bounds")).expect("This tile is open and the game is not over yet")
        /// }).collect::<Vec<_>>();
        /// let result = g.check_end();
        /// assert!(matches!(result, Some(GameResult::Tie)));
        /// ```
        pub fn check_end(&self) -> Option<GameResult> {
            if self.result.is_some() {
                return self.result;
            }
            for line in Self::WIN_LINES {
                let tile_line = line.iter().map(|coords: &Coords| -> &Option<TileValue> {
                    self.board.value_at_coords(coords)
                });
                // println!("tile line pre check: {:?}", &tile_line);
                if tile_line.clone().any(|tile| -> bool { tile.is_none() }) {
                    continue;
                }
                // println!("tile line post chcek: {:?}", &tile_line);
                let mut line_values = tile_line.map(|tile| -> TileValue{
                    tile.expect("Should be `Some` because loop should have continued if any tiles in the line were none")
                });
                // println!("line values {:?}", &line_values);
                if line_values
                    .clone()
                    .all(|tile| -> bool { tile == TileValue::X })
                    || line_values
                        .clone()
                        .all(|tile| -> bool { tile == TileValue::O })
                {
                    return Some(GameResult::Winner(
                        line_values
                            .nth(0)
                            .expect("Iterator should have a 0th element"),
                    ));
                }
            }
            if self
                .board
                .0
                .iter()
                .all(|row| -> bool { row.iter().all(|tile| -> bool { tile.is_some() }) })
            {
                return Some(GameResult::Tie);
            }
            None
        }

        pub fn check_and_update_result(&mut self) -> Option<GameResult> {
            self.result = self.check_end();
            self.result
        }

        /// Returns the `Game`'s board
        /// # Examples
        /// ```rust
        /// use ric_rac_roe_game::game::*;
        /// let g = Game::new();
        /// assert!(matches!(g.board().value_at_coords(&Coords::build(0, 0).expect("is in bounds")), Option::None))
        /// ```
        pub fn board(&self) -> &Board {
            &self.board
        }

        pub fn turn_history(&self) -> &Vec<Turn> {
            &self.turn_history
        }
        pub fn player_turn(&self) -> &TileValue {
            &self.player_turn
        }

        pub fn result(&self) -> &Option<GameResult> {
            &self.result
        }
    }
    impl fmt::Display for Game {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let tiles: &[[DisplayTileValueOption; 3]; 3] =
                &self.board.0.map(|row| -> [DisplayTileValueOption; 3] {
                    row.map(DisplayTileValueOption::from)
                });
            writeln!(f, "")?;
            writeln!(f, "           |           |           ")?;
            #[rustfmt::skip]
            writeln!(f, "     {}     |     {}     |     {}  ",tiles[0][0], tiles[0][1], tiles[0][2])?;
            writeln!(f, "           |           |           ")?;
            writeln!(f, "-----------|-----------|-----------")?;
            writeln!(f, "           |           |           ")?;
            #[rustfmt::skip]
            writeln!(f, "     {}     |     {}     |     {}  ",tiles[1][0], tiles[1][1], tiles[1][2])?;
            writeln!(f, "           |           |           ")?;
            writeln!(f, "-----------|-----------|-----------")?;
            writeln!(f, "           |           |           ")?;
            #[rustfmt::skip]
            writeln!(f, "     {}     |     {}     |     {}  ",tiles[2][0], tiles[2][1], tiles[2][2])?;
            writeln!(f, "           |           |           ")
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum TileValue {
        X,
        O,
    }

    impl TileValue {
        /// Swaps X and O
        /// Examples
        /// ```rust
        /// use ric_rac_roe_game::game::*;
        /// assert!(matches!(TileValue::X.toggle(), TileValue::O));
        /// assert!(matches!(TileValue::O.toggle(), TileValue::X));
        /// ```
        pub fn toggle(self) -> Self {
            use TileValue::*;
            match self {
                X => O,
                O => X,
            }
        }
    }

    impl fmt::Display for TileValue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use TileValue::*;
            write!(
                f,
                "{}",
                match self {
                    X => 'X',
                    O => 'O',
                }
            )
        }
    }

    enum DisplayTileValueOption {
        X,
        O,
        None,
    }
    impl fmt::Display for DisplayTileValueOption {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use DisplayTileValueOption::*;
            write!(
                f,
                "{}",
                match self {
                    X => 'X',
                    O => 'O',
                    None => '-',
                }
            )
        }
    }
    impl From<Option<TileValue>> for DisplayTileValueOption {
        fn from(o: Option<TileValue>) -> Self {
            match o {
                None => DisplayTileValueOption::None,
                Some(TileValue::X) => DisplayTileValueOption::X,
                Some(TileValue::O) => DisplayTileValueOption::O,
            }
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub enum GameResult {
        Winner(TileValue),
        Tie,
    }

    #[derive(Debug)]
    pub enum TurnError {
        TileFull(TileValue),
        GameOver(GameResult),
    }
}
