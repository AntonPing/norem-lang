use std::collections::hash_map;
use std::collections::HashMap;
use std::default::Default;
use std::fmt::{self, Debug};
use std::hash::Hash;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Position {
    pub pos: usize,
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(pos: usize, row: usize, col: usize) -> Position {
        Position { pos, row, col }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}:{:?}({:?})", self.row, self.col, self.pos)
    }
}

/// A span in the source, with a start and end location
#[derive(Clone, Copy, Debug, PartialEq, Hash, Default)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Span {
        Span { start, end }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "&[{}...{}]", self.start, self.end)
    }
}

// multiset

pub struct MultiSet<T>(HashMap<T, usize>);

impl<T> MultiSet<T>
where
    T: Hash + Eq,
{
    pub fn new() -> MultiSet<T> {
        MultiSet(HashMap::new())
    }

    pub fn insert(&mut self, elem: T) {
        if let Some((k, v)) = self.0.remove_entry(&elem) {
            self.0.insert(k, v + 1);
        } else {
            self.0.insert(elem, 1);
        }
    }

    pub fn remove(&mut self, elem: &T) {
        if let Some((k, v)) = self.0.remove_entry(elem) {
            if v > 1 {
                self.0.insert(k, v - 1);
            }
        }
    }

    pub fn remove_all(&mut self, elem: &T) -> usize {
        self.0.remove(elem).unwrap_or(0)
    }

    pub fn get(&self, value: &T) -> usize {
        self.0.get(value).map(|x| *x).unwrap_or(0)
    }

    pub fn contains(&self, value: &T) -> bool {
        self.0.get(value).is_some()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn to_vec(self) -> Vec<T> {
        let mut vec = Vec::new();
        for (k, _) in self.0 {
            vec.push(k);
        }
        vec
    }
}

impl<T> IntoIterator for MultiSet<T> {
    type Item = (T, usize);
    type IntoIter = hash_map::IntoIter<T, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub fn no_repeat<T: Eq>(vec: Vec<T>) -> bool {
    for i in 0..vec.len() {
        for j in i..vec.len() {
            if vec[i] == vec[j] {
                return false;
            }
        }
    }
    return true;
}

#[derive(Clone, Debug, PartialEq)]
enum EnvOp<K, V> {
    // has such key, old value covered
    Update(K, V),
    // has no such key, the key was inserted
    Insert(K),
    // symbol was deleted from env
    Delete(K, V),
    // symbol not in env, no need to delete
    Nothing,
}

/*
#[derive(Clone, Debug)]
pub struct Env<K,V> {
    context: HashMap<K,V>,
    history: Vec<EnvOp<K,V>>,
}

impl<K,V> Env<K,V> where K: Eq + Hash + Clone {
    pub fn new() -> Env<K,V> {
        Env {
            context: HashMap::new(),
            history: Vec::new()
        }
    }

    pub fn contains(&self, key: &K) -> bool {
        self.context.contains_key(key)
    }

    pub fn lookup(&self, key: &K) -> Option<&V> {
        self.context.get(key)
    }

    pub fn update(&mut self, key: K, val: V) {
        if let Some(old) = self.context.insert(key.clone(),val) {
            self.history.push(EnvOp::Update(key,old));
        } else {
            self.history.push(EnvOp::Insert(key));
        }
    }

    pub fn delete(&mut self, key: K) {
        if let Some(old) = self.context.remove(&key) {
            self.history.push(EnvOp::Delete(key,old));
        } else {
            self.history.push(EnvOp::Nothing);
        }
    }

    pub fn backup(&self) -> usize {
        self.history.len()
    }

    pub fn recover(&mut self, mark: usize) {
        for _ in mark..self.history.len() {
            if let Some(op) = self.history.pop() {
                match op {
                    EnvOp::Update(k,v) => {
                        let r = self.context.insert(k,v);
                        assert!(r.is_some());
                    }
                    EnvOp::Insert(k) => {
                        let r = self.context.remove(&k);
                        assert!(r.is_some());
                    }
                    EnvOp::Delete(k,v) => {
                        let r = self.context.insert(k,v);
                        assert!(r.is_none());
                    }
                    EnvOp::Nothing => {
                        // Well, Nothing...
                    }
                }
            } else {
                panic!("history underflow!");
            }
        }
    }
}
*/

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Message {
    pub level: LogLevel,
    pub title: String,
    pub lines: Vec<String>,
    pub span: Span,
    pub other: Vec<Message>,
}

trait Logger : Sized {
    fn into_msg(vec: Vec<Self>) -> Message;
}

impl Message {
    pub fn error<S: Into<String>>(span: Span, title: S) -> Message {
        Message {
            level: LogLevel::Error,
            title: title.into(),
            lines: Vec::new(),
            span,
            other: Vec::new(),
        }
    }

    pub fn warn<S: Into<String>>(span: Span, title: S) -> Message {
        Message {
            level: LogLevel::Warn,
            title: title.into(),
            lines: Vec::new(),
            span,
            other: Vec::new(),
        }
    }

    pub fn info<S: Into<String>>(span: Span, title: S) -> Message {
        Message {
            level: LogLevel::Info,
            title: title.into(),
            lines: Vec::new(),
            span,
            other: Vec::new(),
        }
    }

    pub fn message<S: Into<String>>(mut self, span: Span, msg: Message) -> Message {
        self.other.push(msg);
        self
    }

    pub fn lines(&self) -> std::ops::Range<usize> {
        let mut range = std::ops::Range {
            start: self.span.start.row,
            end: self.span.end.row + 1,
        };

        for each in &self.other {
            if each.span.start.row < range.start {
                range.start = each.span.start.row;
            }
            if each.span.end.row + 1 > range.end {
                range.end = each.span.end.row + 1;
            }
        }
        range
    }
}
