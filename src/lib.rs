//! Red and Black Binary Tree based
#![deny(missing_docs)]
#![cfg_attr(nightly, feature(allocator_api))]
#![cfg_attr(nightly, feature(nonnull_slice_from_raw_parts))]

#[cfg(nightly)]
use std::alloc::{Allocator, Global};

/// Main Structure
pub struct RBBTree<T: std::cmp::PartialOrd, A: Allocator = Global> {
    #[cfg(nightly)]
    data: Vec<Node<T>, A>,
    #[cfg(not(nightly))]
    data: Vec<Node<T>>,
    root: Option<usize>,
    n: usize,
    #[cfg(not(nightly))]
    phantom: std::marker::PhantomData<A>,
}

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
    fn drop(&mut self) {}
}

/// Iterator over Red and Black Binary Tree
pub struct RBBTreeIterator<'a, T, A: Allocator = Global> {
    #[cfg(not(nightly))]
    data: &'a Vec<Node<T>>,
    #[cfg(nightly)]
    data: &'a Vec<Node<T>, A>,
    x: usize,
    stack: Vec<usize>,
    #[cfg(not(nightly))]
    phantom: std::marker::PhantomData<A>,
}

macro_rules! set_black {
    ($item:expr) => {
        $item.color = Color::Black;
    };
}
macro_rules! set_red {
    ($item:expr) => {
        $item.color = Color::Red;
    };
}
macro_rules! is_black {
    ($item:expr) => {
        $item.color as u64 == Color::Black as u64
    };
}
macro_rules! is_red {
    ($item:expr) => {
        $item.color as u64 == Color::Red as u64
    };
}

#[cfg(not(nightly))]
/// Dummy Allocator trait for Stable rust
pub trait Allocator {}
#[cfg(not(nightly))]
/// Dummy Globale struct for Stable rust
pub struct Global {}
#[cfg(not(nightly))]
impl Allocator for Global {}

impl<T: std::cmp::PartialOrd> RBBTree<T> {
    /// Create a new Binary Tree
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            root: None,
            n: 0,
            #[cfg(not(nightly))]
            phantom: std::marker::PhantomData,
        }
    }
}
impl<T: std::cmp::PartialOrd, A: Allocator> RBBTree<T, A> {
    #[cfg(nightly)]
    /// Create a new Binary Tree with Custom Allocator
    pub fn new_in(alloc: A) -> Self {
        Self {
            data: Vec::new_in(alloc),
            root: None,
            n: 0,
        }
    }
    /// Create an iterator over the Binary Tree
    pub fn iter<'a>(&'a self) -> RBBTreeIterator<'a, T, A> {
        RBBTreeIterator {
            data: &self.data,
            x: self.root.unwrap_or(NO_ENTRY),
            stack: Vec::new(),
            #[cfg(not(nightly))]
            phantom: std::marker::PhantomData,
        }
    }

    /// Insert a single element into the Binary Tree
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
    /// Remove a signgle element into the Binary Tree
    pub fn remove(&mut self, value: &T) -> bool {
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
                    self.remove_find_case(
                        index,
                        #[cfg(debug_assertions)]
                        false,
                    );
                    drop(self.swap_remove(index));
                    true
                }
            }
            None => false,
        }
    }
    /// Check if the tree is okay
    #[cfg(any(debug_assertions, test))]
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
    /// Dump the entier Tree with Prefix rules
    #[cfg(any(debug_assertions, test))]
    pub fn prefix_dump(&self)
    where
        T: std::fmt::Debug,
    {
        if let Some(index) = self.root {
            self.prefix_dump_recurse(index, 0);
        }
    }
    #[cfg(any(debug_assertions, test))]
    fn prefix_dump_recurse(&self, x: usize, level: u32)
    where
        T: std::fmt::Debug,
    {
        if self.data[x].left != NO_ENTRY {
            self.prefix_dump_recurse(self.data[x].left, level + 1);
        }
        println!(
            "lvl {} {:?} self: {} p: {} l: {} r: {} color: {:?}",
            level,
            self.data[x].content,
            x,
            self.data[x].parent,
            self.data[x].left,
            self.data[x].right,
            self.data[x].color,
        );
        if self.data[x].right != NO_ENTRY {
            self.prefix_dump_recurse(self.data[x].right, level + 1);
        }
    }
    #[inline(always)]
    fn rotate_right(&mut self, low: usize, high: usize) {
        debug_assert_eq!(self.data[low].parent, high);
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
        debug_assert_eq!(self.data[low].parent, high);
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
                    panic!("swap_remove woot ?");
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
        node
    }
    #[inline(always)]
    fn set_new_child(&mut self, parent: usize, old_entry: usize, entry: usize) {
        if old_entry == self.data[parent].left {
            self.data[parent].left = entry;
        } else if old_entry == self.data[parent].right {
            self.data[parent].right = entry;
        } else {
            panic!("set_new_child woot ?");
        }
    }
    #[inline(always)]
    fn set_as_root(&mut self, new_root: usize) {
        self.root = Some(new_root);
        self.data[new_root].parent = NO_ENTRY;
        set_black!(self.data[new_root]);
    }
    #[inline(always)]
    fn get_brother(&self, index: usize) -> usize {
        debug_assert_ne!(self.data[index].parent, NO_ENTRY);
        let parent = self.data[index].parent;
        if index == self.data[parent].right {
            self.data[parent].left
        } else if index == self.data[parent].left {
            self.data[parent].right
        } else {
            panic!("get_brother woot ?");
        }
    }
    fn remove_modify_tree(&mut self, p: usize, f: usize) {
        debug_assert_ne!(p, NO_ENTRY);
        debug_assert_ne!(f, NO_ENTRY);
        let p_color = self.data[p].color;
        let f_color = self.data[f].color;
        let sl = self.data[f].left;
        let sl_color = if sl != NO_ENTRY {
            self.data[sl].color
        } else {
            Color::Black
        };
        let sr = self.data[f].right;
        let sr_color = if sr != NO_ENTRY {
            self.data[sr].color
        } else {
            Color::Black
        };
        enum Symetric {
            Left,
            Right,
        }
        use Symetric::*;
        let symetric = if f == self.data[p].left {
            Left
        } else if f == self.data[p].right {
            Right
        } else {
            panic!("remove_modify_tree woot ?");
        };
        match (symetric, f_color, sl_color, sr_color) {
            (_, Color::Black, Color::Black, Color::Black) => {
                set_red!(self.data[f]);
                let pp = self.data[p].parent;
                if p_color as u64 == Color::Red as u64 {
                    self.data[p].color = Color::Black;
                } else if p_color as u64 == Color::Black as u64 && pp != NO_ENTRY {
                    let fp = self.get_brother(p);
                    self.remove_modify_tree(pp, fp);
                } else {
                }
            }
            (Left, Color::Black, Color::Red, _) => {
                self.data[f].color = self.data[p].color;
                set_black!(self.data[p]);
                set_black!(self.data[sl]);
                self.rotate_right(f, p);
            }
            (Right, Color::Black, _, Color::Red) => {
                self.data[f].color = self.data[p].color;
                set_black!(self.data[p]);
                set_black!(self.data[sr]);
                self.rotate_left(f, p);
            }
            (Left, Color::Black, _, Color::Red) => {
                self.data[sr].color = self.data[p].color;
                set_black!(self.data[p]);
                self.rotate_left(sr, f);
                self.rotate_right(sr, p);
            }
            (Right, Color::Black, Color::Red, _) => {
                self.data[sl].color = self.data[p].color;
                set_black!(self.data[p]);
                self.rotate_right(sl, f);
                self.rotate_left(sl, p);
            }
            (Left, Color::Red, _, _) => {
                self.rotate_right(f, p);
                set_black!(self.data[f]);
                set_red!(self.data[p]);
                self.remove_modify_tree(p, sr);
            }
            (Right, Color::Red, _, _) => {
                self.rotate_left(f, p);
                set_black!(self.data[f]);
                set_red!(self.data[p]);
                self.remove_modify_tree(p, sl);
            }
        }
    }
    fn remove_find_case(&mut self, index: usize, #[cfg(debug_assertions)] recursive_call: bool) {
        let p = self.data[index].parent;
        let r = self.data[index].right;
        let l = self.data[index].left;
        let is_root = || -> bool { p == NO_ENTRY };
        let right_child_present = || -> bool { r != NO_ENTRY };
        let left_child_present = || -> bool { l != NO_ENTRY };
        if !left_child_present() && !right_child_present() {
            if !is_root() {
                let f = self.get_brother(index);
                self.set_new_child(p, index, NO_ENTRY);
                if is_black!(self.data[index]) {
                    self.remove_modify_tree(p, f);
                }
            } else {
                self.root = None;
            }
        } else if left_child_present() && !right_child_present() {
            if !is_root() {
                let f = self.get_brother(index);
                self.set_new_child(p, index, l);
                self.data[l].parent = p;
                if is_black!(self.data[l]) {
                    self.remove_modify_tree(p, f);
                } else {
                    set_black!(self.data[l]);
                }
            } else {
                self.set_as_root(l);
            }
        } else if !left_child_present() && right_child_present() {
            if !is_root() {
                let f = self.get_brother(index);
                self.set_new_child(p, index, r);
                self.data[r].parent = p;
                if is_black!(self.data[r]) {
                    self.remove_modify_tree(p, f);
                } else {
                    set_black!(self.data[r]);
                }
            } else {
                self.set_as_root(r);
            }
        } else {
            #[cfg(debug_assertions)]
            debug_assert_ne!(recursive_call, true);
            let mut foreign_index = r; // Find right [left...] node
            if self.data[foreign_index].left != NO_ENTRY {
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
            if is_root() {
                self.root = Some(foreign_index);
            } else {
                self.set_new_child(p, index, foreign_index);
            }

            self.data[foreign_index].left = l;
            if left_child_present() {
                self.data[l].parent = foreign_index;
            }
            self.data[index].left = NO_ENTRY;

            let index_color = self.data[index].color;
            self.data[index].color = self.data[foreign_index].color;
            self.data[foreign_index].color = index_color;

            let foreign_right_child = self.data[foreign_index].right;
            self.data[index].right = foreign_right_child;
            if foreign_right_child != NO_ENTRY {
                self.data[foreign_right_child].parent = index;
            }
            self.data[foreign_index].right = if r != foreign_index {
                if right_child_present() {
                    self.data[r].parent = foreign_index;
                }
                r
            } else {
                index
            };
            self.remove_find_case(
                index,
                #[cfg(debug_assertions)]
                true,
            );
        };
    }
    #[cfg(any(debug_assertions, test))]
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
}

impl<'a, T, A: Allocator> Iterator for RBBTreeIterator<'a, T, A> {
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

impl<T: std::cmp::PartialOrd, A: Allocator> Drop for RBBTree<T, A> {
    fn drop(&mut self) {}
}

/**
Commands to test the entire crate with all memory check on x86_64-unknown-linux-gnu
DEBUG
RUST_BACKTRACE=1 RUSTFLAGS=-Zsanitizer=address cargo test -Zbuild-std --target x86_64-unknown-linux-gnu
RELEASE
RUST_BACKTRACE=1 RUSTFLAGS=-Zsanitizer=address cargo test --release -Zbuild-std --target x86_64-unknown-linux-gnu
*/
#[cfg(test)]
mod test {
    use super::RBBTree;
    use rand::distributions::Standard;
    use rand::prelude::*;
    #[test]
    fn simple() {
        let v = vec![
            0.6604497006826313,
            0.4802799059433479,
            0.41722104248437,
            0.009563578859236865,
            0.8728550074374297,
            0.13379267290393926,
            0.009863098457087216,
            0.2927782076332135,
            0.4034453299328443,
            0.39366634150555624,
        ];

        let mut rnb = RBBTree::new();
        for val in v.iter() {
            println!("inserting {}", val);
            rnb.insert(*val);
            rnb.prefix_dump();
            rnb.check_nodes();
        }
        let iter = rnb.iter();
        for elem in iter {
            println!("{}", elem);
        }
        dbg!(rnb.remove(&0.1927782076332135));
        for e in v.iter() {
            println!("Removing entry {}", e);
            rnb.prefix_dump();
            dbg!(rnb.remove(e));
            rnb.prefix_dump();
            rnb.check_nodes();
            println!("Checked");
        }
    }
    #[test]
    fn multiple() {
        make_multiple_test(|| RBBTree::new(), &12.43);
    }
    #[cfg(nightly)]
    #[test]
    fn multiple_custom_alloc() {
        use std::alloc::{AllocError, Allocator, Layout};
        use std::ffi::c_void;
        use std::ptr::NonNull;

        struct CustomAllocator {}

        unsafe impl Allocator for CustomAllocator {
            fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
                let ptr = unsafe { libc::memalign(layout.align(), layout.size()) };
                Ok(NonNull::slice_from_raw_parts(
                    NonNull::new(ptr as *mut u8).unwrap(),
                    layout.size(),
                ))
            }
            unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
                libc::free(ptr.as_ptr() as *mut c_void);
            }
        }
        make_multiple_test(|| RBBTree::new_in(&CustomAllocator {}), &12.43);
    }
    fn make_multiple_test<F, T, A>(gen: F, bad_value: &T)
    where
        F: Fn() -> RBBTree<T, A>,
        T: std::cmp::PartialOrd + std::fmt::Debug + std::fmt::Display + Copy + Clone,
        Standard: Distribution<T>,
        A: super::Allocator,
    {
        let mut rng = rand::thread_rng();
        for _i in 0..256 {
            let mut rnb = gen();
            let mut v = Vec::new();
            for _j in 0..64 {
                v.push(rng.gen::<T>());
            }
            for val in v.iter() {
                println!("inserting {}", val);
                rnb.insert(*val);
                rnb.prefix_dump();
                rnb.check_nodes();
            }
            assert_eq!(rnb.remove(bad_value), false);
            let mut v_acc = v.len();
            v.shuffle(&mut thread_rng());
            for e in v.iter() {
                v_acc -= 1;
                let b = rnb.remove(e);
                assert_eq!(b, true);
                rnb.check_nodes();
                let mut max = None;
                let mut acc = v_acc;
                for g in rnb.iter() {
                    acc -= 1;
                    if let Some(max) = max {
                        if g < max {
                            panic!("Error for {}", g);
                        }
                    }
                    max = Some(g);
                }
                assert_eq!(acc, 0);
            }
            assert_eq!(rnb.iter().next(), None);
        }
    }
}
