use std::borrow::Borrow;
use std::collections::HashMap;
use std::collections::hash_map::{Iter,Keys,Values};
use std::fmt::Debug;
use std::hash::Hash;

/// EnvOpr<K,V> records all the operation have done.
/// During the EnvMap<K,V> doing [`EnvMap::insert`] or [`EnvMap::remove`]

#[derive(Clone, Debug, PartialEq)]
enum EnvOpr<K, V> {
    /// When doing [`EnvMap::insert`]
    /// If has such key in environment, the old value is covered.
    /// `Update(K,V)` record the key `K` and old value `V`
    Update(K, V),
    /// When doing [`EnvMap::insert`]
    /// If there was no such key in environment, the key and new value were inserted
    /// `Insert(K)` record only the key `K`
    Insert(K),
    /// When doing [`EnvMap::remove`]
    /// If has such key in environment, the old value is deleted.
    /// `Delete(K,V)` record the key `K` and old value `V`
    Delete(K, V),
    /// When doing [`EnvMap::remove`]
    /// If there was no such key in environment, nothing happend.
    /// Therefore, record nothing.
    Nothing,
}

/// A Wrapper for `usize`, just for type safty
/// Used in [`EnvMap::backup`] and [`EnvMap::recover`]
pub struct Backup(usize);

/// EnvMap is a wrapper of HashMap, but got the ability to backtrack and recover from the local modification
#[derive(Clone, Debug)]
pub struct EnvMap<K,V> {
    /// the HashMap to do all the work.
    bindmap: HashMap<K,V>,
    /// the history that records all the operation have done.
    history: Vec<EnvOpr<K,V>>,
}

/// Many implementations are just a copy of HashMap
impl<K,V> EnvMap<K,V> where K: Eq + Hash + Clone {

    /// Creating an empty EnvMap
    pub fn new() -> EnvMap<K,V> {
        EnvMap {
            bindmap: HashMap::new(),
            history: Vec::new()
        }
    }

    /// Creating an empty EnvMap with capacity
    pub fn with_capacity(capacity: usize) -> EnvMap<K,V> {
        EnvMap {
            bindmap: HashMap::with_capacity(capacity),
            history: Vec::new()
        }
    }

    /// Returns the number of elements the map can hold without reallocating.
    pub fn capacity(&self) -> usize {
        self.bindmap.capacity()
    }

    /// An iterator visiting all keys in arbitrary order.
    pub fn keys(&self) -> Keys<'_, K, V> {
        self.bindmap.keys()
    }

    /// An iterator visiting all values in arbitrary order.
    pub fn values(&self) -> Values<'_, K, V> {
        self.bindmap.values()
    }

    /// An iterator visiting all key-value pairs in arbitrary order.
    pub fn iter(&self) -> Iter<'_, K, V> {
        self.bindmap.iter()
    }

    /// Returns `true` if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.bindmap.is_empty()
    }

    /// Returns a reference to the value corresponding to the key.
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V> where
        K: Borrow<Q>,
        Q: Hash + Eq, 
    {
        self.bindmap.get(k)
    }

    /// Returns the key-value pair corresponding to the supplied key.
    pub fn get_key_value<Q: ?Sized>(&self, k: &Q) -> Option<(&K, &V)> where
        K: Borrow<Q>,
        Q: Hash + Eq, 
    {
        self.bindmap.get_key_value(k)
    }

    /// Returns `true` if the map contains a value for the specified key.
    pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool where
        K: Borrow<Q>,
        Q: Hash + Eq, 
    {
        self.bindmap.contains_key(k)
    }


    /// Inserts a key-value pair into the map.
    pub fn insert(&mut self, k: K, v: V) {
        if let Some(old) = self.bindmap.insert(k.clone(),v) {
            self.history.push(EnvOpr::Update(k,old));
        } else {
            self.history.push(EnvOpr::Insert(k));
        }
    }

    /// Removes a key from the map
    pub fn remove(&mut self, k: &K) {
        if let Some(old) = self.bindmap.remove(k) {
            self.history.push(EnvOpr::Delete(k.clone(),old));
        } else {
            self.history.push(EnvOpr::Nothing);
        }
    }

    /// Make a backup of EnvMap, you may use [`EnvMap::recover`] to recover from all the modification during this period.
    /// It just like a time machine!
    /// 
    /// # Example
    /// 
    /// ```
    /// use norem_utils::envmap::EnvMap;
    /// 
    /// let mut env = EnvMap::new();
    /// env.insert(&1,'a');
    /// let backup = env.backup();
    /// env.insert(&2,'b');
    /// env.insert(&3,'c');
    /// assert_eq!(env.get(&1), Some(&'a'));
    /// assert_eq!(env.get(&2), Some(&'b'));
    /// assert_eq!(env.get(&3), Some(&'c'));
    /// env.recover(backup);
    /// assert_eq!(env.get(&1), Some(&'a'));
    /// assert_eq!(env.get(&2), None);
    /// assert_eq!(env.get(&3), None);
    /// ```

    /// Make a backup of EnvMap
    pub fn backup(&self) -> Backup {
        Backup(self.history.len())
    }

    /// Recover from a backup.
    pub fn recover(&mut self, backup: Backup) {
        let n = backup.0;
        for _ in n..self.history.len() {
            match self.history.pop().unwrap() {
                EnvOpr::Update(k,v) => {
                    // recover the old value that was covered by insert
                    let r = self.bindmap.insert(k,v);
                    assert!(r.is_some());
                }
                EnvOpr::Insert(k) => {
                    // remove the inserted key and value
                    let r = self.bindmap.remove(&k);
                    assert!(r.is_some());
                }
                EnvOpr::Delete(k,v) => {
                    // recover the deleted key and value
                    let r = self.bindmap.insert(k,v);
                    assert!(r.is_none());
                }
                EnvOpr::Nothing => {
                    // Well, do nothing...
                }
            }
        }
    }
}

