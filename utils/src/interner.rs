use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct InternStr {
    index: usize,
}

pub struct Interner {
    str_to_idx: HashMap<String, usize>,
    idx_to_str: Vec<String>,
}

impl Interner {
    pub fn new() -> Interner {
        Interner {
            str_to_idx: HashMap::new(),
            idx_to_str: Vec::new(),
        }
    }

    pub fn intern<S: Into<String>>(&mut self, s: S) -> InternStr {
        let s: String = s.into();
        if let Some(idx) = self.str_to_idx.get(&s) {
            InternStr { index: *idx }
        } else {
            let idx = self.idx_to_str.len();
            self.idx_to_str.push(s.clone());
            self.str_to_idx.insert(s, idx);
            InternStr{ index: idx }
        }
    }

    pub fn get_str<'a>(&'a self, s: InternStr) -> &'a str {
        let InternStr { index } = s;
        &self.idx_to_str[index]
    }
}

///Returns a reference to the interner stored in TLD
pub fn get_local_interner() -> Rc<RefCell<Interner>> {
    thread_local!(static INTERNER: Rc<RefCell<Interner>> = Rc::new(RefCell::new(Interner::new())));
    INTERNER.with(|interner| interner.clone())
}

pub fn intern(s: &str) -> InternStr {
    let i = get_local_interner();
    let mut i = i.borrow_mut();
    i.intern(s)
}

impl Deref for InternStr {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_ref()
    }
}

impl AsRef<str> for InternStr {
    fn as_ref(&self) -> &str {
        let interner = get_local_interner();
        let x = (*interner).borrow_mut();
        let r: &str = x.get_str(*self);
        //The interner is task local and will never remove a string so this is safe
        unsafe { std::mem::transmute(r) }
    }
}

impl fmt::Display for InternStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.as_ref())
    }
}