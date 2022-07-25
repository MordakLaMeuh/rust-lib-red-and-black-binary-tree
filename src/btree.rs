#[macro_export]
macro_rules! link {
    ($child:expr, $parent:expr) => {};
}

#[macro_export]
macro_rules! set_black {
    ($item:expr) => {
        $item.color = Color::Black;
    };
}

#[macro_export]
macro_rules! set_red {
    ($item:expr) => {
        $item.color = Color::Red;
    };
}

#[macro_export]
macro_rules! is_black {
    ($item:expr) => {
        $item.color as u64 == Color::Black as u64
    };
}
#[macro_export]
macro_rules! is_red {
    ($item:expr) => {
        $item.color as u64 == Color::Red as u64
    };
}

use super::raw_vec::RawVec;

#[repr(u64)]
#[derive(Debug, Copy, Clone)]
enum Color {
    Red,
    Black,
}

#[repr(C, align(64))]
#[derive(Debug)]
struct Node<T> {
    content: T,
    color: Color,
    parent: usize,
    left: usize,
    right: usize,
}

const NO_ENTRY: usize = usize::MAX;

impl<T: std::cmp::PartialOrd> Node<T> {
    fn new(content: T) -> Self {
        Self {
            content,
            color: Color::Red,
            parent: NO_ENTRY,
            left: NO_ENTRY,
            right: NO_ENTRY,
        }
    }
}
impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        // println!("Node droped for value {:?}", self.content);
    }
}

pub struct BinaryTree<T> {
    data: RawVec<Node<T>>,
    root: Option<usize>,
    n: usize,
}

pub struct BinaryTreeIterator<'a, T> {
    data: &'a RawVec<Node<T>>,
    x: usize,
    stack: Vec<usize>,
}

impl<T: std::cmp::PartialOrd + std::fmt::Debug> BinaryTree<T> {
    pub fn new() -> Self {
        Self {
            data: RawVec::new(),
            root: None,
            n: 0,
        }
    }
    pub fn iter<'a>(&'a self) -> BinaryTreeIterator<'a, T> {
        BinaryTreeIterator {
            data: &self.data,
            x: self.root.unwrap_or(NO_ENTRY),
            stack: Vec::new(),
        }
    }

    fn prefix_dump_recurse(&self, x: usize, level: u32) {
        if self.data[x].left != NO_ENTRY {
            self.prefix_dump_recurse(self.data[x].left, level + 1);
        }
        println!(
            "lvl {} {:?} self: {} p: {} l: {} r: {}",
            level,
            self.data[x].content,
            x,
            self.data[x].parent,
            self.data[x].left,
            self.data[x].right
        );
        if self.data[x].right != NO_ENTRY {
            self.prefix_dump_recurse(self.data[x].right, level + 1);
        }
    }

    pub fn prefix_dump(&self) {
        if let Some(index) = self.root {
            self.prefix_dump_recurse(index, 0);
        }
    }
    #[inline(always)]
    fn rotate_right(&mut self, low: usize, high: usize) {
        self.data[low].parent = self.data[high].parent; // Assign new parents
        self.data[high].parent = low;
        self.data[high].left = self.data[low].right; // Move values
        let left_index = self.data[high].left;
        if left_index != NO_ENTRY {
            // tell the son that I am his father
            self.data[left_index].parent = high;
        }
        self.data[low].right = high;
        if self.data[low].parent == NO_ENTRY {
            // Root may be changed
            self.root = Some(low);
        } else {
            // Or juste change parent ref
            let new_parent = self.data[low].parent;
            self.set_new_child(new_parent, high, low);
        }
    }
    #[inline(always)]
    fn rotate_left(&mut self, low: usize, high: usize) {
        self.data[low].parent = self.data[high].parent; // Assign new parents
        self.data[high].parent = low;
        self.data[high].right = self.data[low].left; // Move values
        let right_index = self.data[high].right;
        if right_index != NO_ENTRY {
            // tell the son that I am his father
            self.data[right_index].parent = high;
        }
        self.data[low].left = high;
        if self.data[low].parent == NO_ENTRY {
            // Root may be changes
            self.root = Some(low);
        } else {
            // Or juste change parent ref
            let new_parent = self.data[low].parent;
            self.set_new_child(new_parent, high, low);
        }
    }
    fn insert_recurse(&mut self, x: usize) {
        let p = self.data[x].parent;
        if is_red!(self.data[p]) {
            if p != self.root.unwrap() {
                let pp = self.data[p].parent;
                let f = if p == self.data[pp].left {
                    self.data[pp].right
                } else {
                    self.data[pp].left
                };
                if f == NO_ENTRY || is_black!(self.data[f]) {
                    if p == self.data[pp].left {
                        if x == self.data[p].left {
                            self.rotate_right(p, pp);
                            set_black!(self.data[p]);
                            set_red!(self.data[pp]);
                        } else {
                            self.rotate_left(x, p);
                            return self.insert_recurse(p);
                        }
                    } else {
                        if x == self.data[p].right {
                            self.rotate_left(p, pp);
                            set_black!(self.data[p]);
                            set_red!(self.data[pp]);
                        } else {
                            self.rotate_right(x, p);
                            return self.insert_recurse(p);
                        }
                    }
                } else {
                    set_black!(self.data[p]);
                    set_black!(self.data[f]);
                    set_red!(self.data[pp]);
                    self.data[self.root.unwrap()].color = Color::Black;
                    if pp != self.root.unwrap() {
                        return self.insert_recurse(pp);
                    }
                }
            }
        }
    }

    pub fn insert(&mut self, content: T) {
        self.n += 1;
        match self.root {
            Some(mut index) => {
                index = loop {
                    if content < self.data[index].content {
                        if self.data[index].left != NO_ENTRY {
                            index = self.data[index].left;
                        } else {
                            self.data.push(Node::new(content));
                            let new_index = self.data.len() - 1;
                            self.data[index].left = new_index;
                            self.data[new_index].parent = index;
                            break new_index;
                        }
                    } else {
                        if self.data[index].right != NO_ENTRY {
                            index = self.data[index].right;
                        } else {
                            self.data.push(Node::new(content));
                            let new_index = self.data.len() - 1;
                            self.data[index].right = new_index;
                            self.data[new_index].parent = index;
                            break new_index;
                        }
                    }
                };
                self.insert_recurse(index);
            }
            None => {
                self.data.push(Node::new(content));
                self.data[0].color = Color::Black;
                self.root = Some(0);
            }
        };
    }
    fn swap_remove(&mut self, index: usize) -> Node<T> {
        let node = self.data.swap_remove(index);
        if index < self.data.len() {
            let p = self.data[index].parent;
            let l = self.data[index].left;
            let r = self.data[index].right;
            if p != NO_ENTRY {
                let old_len = self.data.len() + 1;
                if self.data[p].left == (old_len - 1) {
                    self.data[p].left = index;
                } else if self.data[p].right == (old_len - 1) {
                    self.data[p].right = index;
                } else {
                    panic!("sa mere");
                }
            } else {
                self.root = Some(index);
            }
            if l != NO_ENTRY {
                self.data[l].parent = index;
            }
            if r != NO_ENTRY {
                self.data[r].parent = index;
            }
        }
        dbg!(node)
    }
    #[inline(always)]
    fn set_new_child(&mut self, parent: usize, old_entry: usize, entry: usize) {
        if old_entry == self.data[parent].left {
            self.data[parent].left = entry;
        } else if old_entry == self.data[parent].right {
            self.data[parent].right = entry;
        } else {
            panic!("sa mere");
        }
    }
    #[inline(always)]
    fn set_as_root(&mut self, new_root: usize) {
        self.root = Some(new_root);
        self.data[new_root].parent = NO_ENTRY;
        set_black!(self.data[new_root]);
    }
    fn remove_find_case(&mut self, index: usize) {
        let p = self.data[index].parent;
        let r = self.data[index].right;
        let l = self.data[index].left;
        let is_root = || -> bool { p == NO_ENTRY };
        let right_child_present = || -> bool { r != NO_ENTRY };
        let left_child_present = || -> bool { l != NO_ENTRY };
        if !left_child_present() && !right_child_present() {
            println!("case no right or left son");
            if !is_root() {
                self.set_new_child(p, index, NO_ENTRY);
                if is_black!(self.data[index]) {
                    // move tree f s1 s2 p ON index WARNING retreave F before set_new_child
                }
            } else {
                println!("case root - END");
                self.root = None;
            }
        } else if left_child_present() && !right_child_present() {
            println!("case l but no r");
            if !is_root() {
                self.set_new_child(p, index, l);
                self.data[l].parent = p;
                if is_black!(self.data[l]) {
                    // move tree f s1 s2 p ON l
                }
            } else {
                println!("case root");
                self.set_as_root(l);
            }
        } else if !left_child_present() && right_child_present() {
            println!("case r but no l");
            if !is_root() {
                self.set_new_child(p, index, r);
                self.data[r].parent = p;
                if is_black!(self.data[r]) {
                    // move tree f s1 s2 p ON r
                }
            } else {
                println!("case root");
                self.set_as_root(r);
            }
        } else {
            println!("case two childs");
            let mut foreign_index = r; // Find right [left...] node
            if self.data[foreign_index].left != NO_ENTRY {
                println!("case right - [left]...");
                while self.data[foreign_index].left != NO_ENTRY {
                    foreign_index = self.data[foreign_index].left;
                }
            }

            let foreign_parent = self.data[foreign_index].parent;
            self.data[index].parent = if foreign_parent != index {
                self.set_new_child(foreign_parent, foreign_index, index);
                foreign_parent
            } else {
                foreign_index
            };
            self.data[foreign_index].parent = p;
            if p == NO_ENTRY {
                self.root = Some(foreign_index);
            } else {
                self.set_new_child(p, index, foreign_index);
            }

            self.data[foreign_index].left = l;
            if l != NO_ENTRY {
                self.data[l].parent = foreign_index;
            }
            self.data[index].left = NO_ENTRY;

            let foreign_right_child = self.data[foreign_index].right;
            self.data[index].right = foreign_right_child;
            if foreign_right_child != NO_ENTRY {
                self.data[foreign_right_child].parent = index;
            }
            self.data[foreign_index].right = if r != foreign_index {
                if r != NO_ENTRY {
                    self.data[r].parent = foreign_index;
                }
                r
            } else {
                index
            };
            self.remove_find_case(index);
        };
    }

    pub fn remove(&mut self, value: &T) -> bool {
        println!("Removing stage 1");
        match self.root {
            Some(mut index) => {
                index = loop {
                    if value == &self.data[index].content {
                        break index;
                    } else if value < &self.data[index].content {
                        if self.data[index].left != NO_ENTRY {
                            index = self.data[index].left;
                        } else {
                            break NO_ENTRY;
                        }
                    } else {
                        if self.data[index].right != NO_ENTRY {
                            index = self.data[index].right;
                        } else {
                            break NO_ENTRY;
                        }
                    }
                };
                if index == NO_ENTRY {
                    false
                } else {
                    self.n -= 1;
                    self.remove_find_case(index);
                    drop(self.swap_remove(index));
                    true
                }
            }
            None => false,
        }
    }
    fn check_nodes_recurse(
        &self,
        x: usize,
        mut black_nodes: usize,
        acc: &mut usize,
        color: Color,
    ) -> usize {
        let x_ref = &self.data[x];
        let parent = x_ref.parent;
        if parent != NO_ENTRY {
            if x != self.data[parent].left && x != self.data[parent].right {
                panic!("Orphelan Node");
            }
        }
        if color as u64 == Color::Red as u64 && x_ref.color as u64 == Color::Red as u64 {
            panic!("A red node follow a red node: bl_lvl {}", acc);
        }
        let color = x_ref.color;
        if let Color::Black = color {
            black_nodes += 1;
        }
        *acc += 1;
        let black_left = if x_ref.left != NO_ENTRY {
            self.check_nodes_recurse(x_ref.left, black_nodes, acc, color)
        } else {
            black_nodes
        };
        let black_right = if x_ref.right != NO_ENTRY {
            self.check_nodes_recurse(x_ref.right, black_nodes, acc, color)
        } else {
            black_nodes
        };
        assert_eq!(black_left, black_right);
        black_left
    }

    pub fn check_nodes(&self) {
        if let Some(index) = self.root {
            assert_eq!(self.data[index].color as u64, Color::Black as u64);
            let mut total_nodes = 0;
            self.check_nodes_recurse(index, 0, &mut total_nodes, Color::Black);
            assert_eq!(total_nodes, self.n);
        } else {
            assert_eq!(0, self.n);
        }
    }
}

impl<'a, T> Iterator for BinaryTreeIterator<'a, T> {
    // we will be counting with usize
    type Item = &'a T;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        while !self.stack.is_empty() || self.x != NO_ENTRY {
            if self.x != NO_ENTRY {
                self.stack.push(self.x);
                self.x = self.data[self.x].left;
            } else {
                self.x = self.stack.pop().unwrap();
                let content = &self.data[self.x].content;
                self.x = self.data[self.x].right;
                return Some(content);
            }
        }
        return None;
    }
}

impl<T> Drop for BinaryTree<T> {
    fn drop(&mut self) {}
}
