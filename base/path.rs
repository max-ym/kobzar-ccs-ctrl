use std::rc::Rc;
use std::iter::*;
use std::collections::LinkedList;

/// The node of the path.
pub struct PathNode {

    /// Package name at current hierarchy level.
    pack   : String,
}

/// The path (location) of the package.
pub struct Path {

    /// All nodes of the path in order from top level node first in array to
    /// current level node last in array.
    nodes   : Vec<Rc<PathNode>>,
}

/// Path nodes iterator.
pub struct PathIter {
    path    : Path,
    pos     : usize,
}

/// PathConstructor allows to create the path by adding nodes one by one.
pub struct PathConstructor {
    list    : LinkedList<Rc<PathNode>>,
}

impl PathNode {

    /// Generate new node for given package name.
    pub fn new(package_name: String) -> Self {
        PathNode {
            pack: package_name,
        }
    }

    /// The name of the package.
    pub fn package_name(&self) -> &String {
        &self.pack
    }
}

impl Path {

    /// Retrieve all path nodes in the array.
    pub fn nodes(&self) -> &Vec<Rc<PathNode>> {
        &self.nodes
    }
}

impl Iterator for PathIter {

    type Item = Rc<PathNode>;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.path.nodes.len();
        if len <= self.pos {
            return None;
        }

        let item = self.path.nodes[self.pos].clone();

        self.pos += 1;

        Some(item)
    }
}

impl ExactSizeIterator for PathIter {

    fn len(&self) -> usize {
        self.path.nodes.len()
    }
}

impl FusedIterator for PathIter {}

impl IntoIterator for Path {

    type Item = Rc<PathNode>;
    type IntoIter = PathIter;

    fn into_iter(self) -> Self::IntoIter {
        PathIter {
            path    : self,
            pos     : 0,
        }
    }
}

impl From<PathIter> for Path {

    fn from(iter: PathIter) -> Path {
        iter.path
    }
}

impl PathConstructor {

    /// Create new path constructor with empty path.
    pub fn new() -> PathConstructor {
        PathConstructor {
            list: LinkedList::new(),
        }
    }

    /// Internal path list from top level node in the beginning of the list to
    /// current node at the end.
    pub fn list(&self) -> &LinkedList<Rc<PathNode>> {
        &self.list
    }

    /// Internal path list from top level node in the beginning of the list to
    /// current node at the end.
    pub fn list_mut(&mut self) -> &mut LinkedList<Rc<PathNode>> {
        &mut self.list
    }

    /// Finish composing Path. If no PathNodes were added then None is
    /// returned.
    pub fn finish(self) -> Option<Path> {
        let len = self.list.len();
        let list_is_empty = len == 0;

        if list_is_empty {
            None
        } else {
            let mut vec = Vec::with_capacity(len);

            for i in self.list {
                vec.push(i.clone());
            }

            Some(Path { nodes: vec })
        }
    }
}
