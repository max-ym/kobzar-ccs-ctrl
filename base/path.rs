use std::rc::Rc;
use std::iter::*;
use std::collections::LinkedList;
use std::cmp::Ordering;

/// The node of the path.
#[derive(Clone)]
pub struct Path {

    /// Package name at current hierarchy level.
    name    : String,

    /// Previous path node if any.
    prev    : Option<Rc<Path>>,
}

/// Path nodes iterator.
pub struct PathIter<'a> {

    /// Last node of the path from which this iterator was created.
    end     : &'a Path,

    /// Current node.
    cur     : Option<Rc<Path>>,
}

/// Bidirectional path iterator.
pub struct BiPathIter<'a> {

    /// Whole path in array.
    path    : Vec<Rc<Path>>,

    /// The end path part from which this iterator was generated has
    /// limited lifetime and as the last Rc in 'path' field was created
    /// using non-full unsafe cloning of Path parent instance we must
    /// guarantee that parent outlives this iterator so we could not
    /// refer to non-existent data.
    parent_life: ::std::marker::PhantomData<&'a Path>,

    /// Front position of the iterator.
    front   : usize,

    /// Back position of the iterator. 0 means last node, 1 - one before last
    /// etc.
    back    : usize,
}

impl PartialEq for Path {

    fn eq(&self, other: &Path) -> bool {
        if self.prev.is_some() {
            if other.prev.is_none() {
                false
            } else {
                other.prev.clone().unwrap() == self.prev.clone().unwrap()
            }
        } else if other.prev.is_none() {
            self.name == other.name
        } else {
            false
        }
    }
}

impl Eq for Path {}

impl Ord for Path {

    fn cmp(&self, other: &Path) -> Ordering {
        use self::Ordering::*;

        let mut i0 = self.iter();
        let mut i1 = other.iter();

        match self.iter().count().cmp(&other.iter().count()) {
            Greater => Greater,
            Less    => Less,
            Equal   => loop {
                let next = i0.next();
                if next.is_none() {
                    return next.unwrap().name.cmp(&other.name);
                } else {
                    let next = next.unwrap();

                    return match next.cmp(&i1.next().unwrap()) {
                        Greater => Greater,
                        Less    => Less,
                        Equal   => continue,
                    };
                }
            }
        }
    }
}

impl PartialOrd for Path {

    fn partial_cmp(&self, other: &Path) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Path {

    /// Generate new root node for given package name.
    pub fn new(name: String) -> Self {
        Path {
            name,
            prev: None,
        }
    }

    /// Create new node at the end of given path.
    pub fn new_in_path(name: String, prev: Rc<Path>) -> Self {
        Path {
            name,
            prev: Some(prev),
        }
    }

    /// The name of the node.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Full path from this node to root node with given delimiter string.
    pub fn full_path(&self, delim: &str) -> String {
        let length = {

            // Delimiter repeat counter:
            let mut delims = 0;

            // Each node name length adds to this variable:
            let mut strings = 0;

            for node in self.iter() {
                strings += node.name.len();
                delims += 1;
            }

            // Note: currently delimiter was counted as if after each node
            // it is placed but in reality it is not placed after last one.
            // Fix this by subtracting 1.
            delims -= 1;

            strings + delims * delim.len()
        };

        let mut fullpath = String::with_capacity(length);

        for node in self.iter() {
            fullpath += &node.name;
            fullpath += delim;
        }

        fullpath
    }

    /// Iterator for current path.
    pub fn iter<'a>(&'a self) -> PathIter<'a> {
        PathIter {
            end: self,
            cur: Some(Rc::new(unsafe { self.clone_but_save_refs() })),
        }
    }

    /// Bi-directional iterator for current path.
    pub fn bi_iter<'a>(&'a self) -> BiPathIter<'a> {
        let path = {
            let len = self.iter().count();
            let mut vec = Vec::with_capacity(len);

            for item in self.iter() {
                vec.push(item.clone());
            }
            vec.reverse();
            vec
        };

        BiPathIter {
            path,
            parent_life: Default::default(),
            front: 0,
            back: 0,
        }
    }

    /// Make a clone of this instance but don't clone the name. Instead,
    /// create a String from raw parts to point to already allocated
    /// name.
    ///
    /// # Safety
    ///
    /// When parent instance gets dropped then
    /// this clone will refer to non-existent data.
    unsafe fn clone_but_save_refs(&self) -> Self {
        let name = {
            let ptr = self.name.as_bytes().as_ptr() as _;
            let length = self.name.len();
            let capacity = self.name.capacity();
            String::from_raw_parts(ptr, length, capacity)
        };
        Path {
            name,
            prev: self.prev.clone(),
        }
    }
}

impl<'a> Iterator for PathIter<'a> {

    type Item = Rc<Path>;

    fn next(&mut self) -> Option<Self::Item> {
        let has_cur = self.cur.is_some();
        if has_cur {
            let result = self.cur.clone().unwrap();
            self.cur = self.cur.clone().unwrap().prev.clone();
            Some(result)
        } else {
            None
        }
    }
}

impl<'a> FusedIterator for PathIter<'a> {}

impl<'a> Iterator for BiPathIter<'a> {

    type Item = Rc<Path>;

    fn next(&mut self) -> Option<Rc<Path>> {
        if self.len() >= self.front {
            None
        } else {
            let item = self.path[self.front].clone();
            self.front += 1;
            Some(item)
        }
    }
}

impl<'a> ExactSizeIterator for BiPathIter<'a> {

    fn len(&self) -> usize {
        self.path.len()
    }
}

impl<'a> FusedIterator for BiPathIter<'a> {}

impl<'a> DoubleEndedIterator for BiPathIter<'a> {

    fn next_back(&mut self) -> Option<Rc<Path>> {
        if self.len() >= self.back {
            None
        } else {
            let item = self.path[self.len() - self.back - 1].clone();
            self.back += 1;
            Some(item)
        }
    }
}
