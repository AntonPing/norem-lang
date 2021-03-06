use std::collections::hash_map;
use std::collections::HashMap;
use std::default::Default;
use std::fmt::{self, Debug};
use std::hash::Hash;


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Position {
    pub row: usize,
    pub col: usize,
    pub abs: usize,
}

impl Position {
    pub fn new(
        col: usize,
        row: usize,
        abs: usize,
    ) -> Position {
        Position { row, col, abs }
    }
    pub fn dummy() -> Position {
        Position { row: 0, col: 0, abs: 0 }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}({})", self.row, self.col, self.abs)
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

