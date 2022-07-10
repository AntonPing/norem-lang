use std::default::Default;
use std::fmt::{self, Debug};
use std::hash::Hash;

/// utils::position module
/// Defining Position and Span


/// A Position is a location in the source code.
/// the `abs` field is the absolute index of the character.
/// while `row` and `col` fields mark the line and the column number
/// 
/// # Example
/// 
/// ```text
/// Hello, world!
/// This is an
/// example source code
/// ```
/// Here the letter 'i' in word "This" has Position
/// Position { row: 2, col: 3, abs: 16 }
/// (Hope I didn't count it wrong)

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Position {
    pub row: usize,
    pub col: usize,
    pub abs: usize,
}

impl Position {
    /// Creating a new Position.
    pub fn new(col: usize, row: usize, abs: usize) -> Position {
        Position { row, col, abs }
    }

    /// Creating a dummy Position, which means you don't care what it is.
    /// In several passes abstract syntax tree was transformed or desugared so it doesn't really have a corresponding source code position. In such case it could be used.
    pub fn dummy() -> Position {
        Position {
            /// All fields are the max value of `usize`.
            /// In binary forms it looks like a `0b111...1`. You may think it as -1, but has type `usize`.
            row: usize::max_value(),
            col: usize::max_value(),
            abs: usize::max_value(),
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}({})", self.row, self.col, self.abs)
    }
}

/// A Span is a structure of two position.
/// It annotates the `start` and the `end` of a small piece of source code.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    /// Creating a new Span.
    pub fn new(start: Position, end: Position) -> Span {
        Span { start, end }
    }
    /// Same usage as [`Positon::dummy`]
    /// See [`Positon::dummy`]
    pub fn dummy() -> Span {
        Span {
            start: Position::dummy(),
            end: Position::dummy(),
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "&[{}...{}]", self.start, self.end)
    }
}
