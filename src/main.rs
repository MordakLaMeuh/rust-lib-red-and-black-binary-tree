use std::ptr;

pub struct BinaryTree<T: std::fmt::Debug> {
    root: *mut Node<T>,
    n: usize,
}

pub struct BinaryTreeIterator<'a, T: std::fmt::Debug> {
    current: *mut Node<T>,
    stack: Vec<*mut Node<T>>,
    phantom: std::marker::PhantomData<&'a T>,
}

impl<T: std::cmp::PartialOrd + std::fmt::Debug> BinaryTree<T> {
    pub fn new() -> Self {
        Self {
            root: ptr::null_mut(),
            n: 0,
        }
    }
    pub fn iter<'a>(&'a self) -> BinaryTreeIterator<'a, T> {
        BinaryTreeIterator {
            current: self.root,
            stack: Vec::new(),
            phantom: std::marker::PhantomData,
        }
    }
    pub fn prefix_dump(&self) {
        if !self.root.is_null() {
            unsafe {
                self.root.as_ref().unwrap().prefix_dump();
            }
        }
    }
    pub fn insert_content(&mut self, content: T) {
        self.n += 1;
        if self.root.is_null() {
            self.root = Box::into_raw(Box::new(Node::new(content)));
            unsafe {
                (*(self.root)).color = Color::Black;
            }
        } else {
            unsafe {
                self.root = self.root.as_mut().unwrap().insert_content(content);
            }
        }
    }
    pub fn check_nodes(&self) {
        unsafe fn recurse<T: std::fmt::Debug>(
            node: *mut Node<T>,
            mut black_nodes: usize,
            acc: &mut usize,
            color: Color,
        ) -> usize {
            if !(*node).parent.is_null() {
                if node != (*(*node).parent).left && node != (*(*node).parent).right {
                    panic!("Orphelan Node");
                }
            }
            if color as u64 == Color::Red as u64 && (*node).color as u64 == Color::Red as u64 {
                panic!("A red node follow a red node: bl_lvl {}", acc);
            }
            let color = (*node).color;
            if let Color::Black = color {
                black_nodes += 1;
            }
            *acc += 1;
            let black_left = if !(*node).left.is_null() {
                recurse((*node).left, black_nodes, acc, color)
            } else {
                black_nodes
            };
            let black_right = if !(*node).right.is_null() {
                recurse((*node).right, black_nodes, acc, color)
            } else {
                black_nodes
            };
            assert_eq!(black_left, black_right);
            black_left
        }
        if !self.root.is_null() {
            unsafe {
                assert_eq!((*self.root).color as u64, Color::Black as u64);
                let mut total_nodes = 0;
                recurse(self.root, 0, &mut total_nodes, Color::Black);
                assert_eq!(total_nodes, self.n);
            }
        } else {
            assert_eq!(0, self.n);
        }
    }
}

impl<'a, T: std::fmt::Debug> Iterator for BinaryTreeIterator<'a, T> {
    // we will be counting with usize
    type Item = &'a T;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        while !self.stack.is_empty() || !self.current.is_null() {
            if !self.current.is_null() {
                self.stack.push(self.current);
                self.current = unsafe { (*self.current).left };
            } else {
                self.current = self.stack.pop().unwrap();
                let content = unsafe { &(*self.current).content };
                self.current = unsafe { (*self.current).right };
                return Some(content);
            }
        }
        return None;
    }
}
impl<T: std::fmt::Debug> Drop for BinaryTree<T> {
    fn drop(&mut self) {
        unsafe fn recurse<T: std::fmt::Debug>(node: *mut Node<T>) {
            if !(*node).left.is_null() {
                recurse((*node).left);
            }
            if !(*node).right.is_null() {
                recurse((*node).right);
            }
            drop(Box::from_raw(node));
        }
        if !self.root.is_null() {
            unsafe {
                recurse(self.root);
            }
        }
    }
}

#[repr(u64)]
#[derive(Debug, Copy, Clone)]
enum Color {
    Red,
    Black,
}

#[repr(C, align(64))]
#[derive(Debug)]
struct Node<T: std::fmt::Debug> {
    content: T,
    color: Color,
    parent: *mut Node<T>,
    left: *mut Node<T>,
    right: *mut Node<T>,
}

impl<T: std::cmp::PartialOrd + std::fmt::Debug> Node<T> {
    fn new(content: T) -> Self {
        Self {
            color: Color::Red,
            content,
            parent: ptr::null_mut(),
            left: ptr::null_mut(),
            right: ptr::null_mut(),
        }
    }
    unsafe fn prefix_dump(&self) {
        unsafe fn recurse<T: std::fmt::Debug>(node: *const Node<T>, level: u32) {
            if !(*node).left.is_null() {
                recurse((*node).left, level + 1);
            }
            println!("lvl {} {:?}", level, (*node));
            if !(*node).right.is_null() {
                recurse((*node).right, level + 1);
            }
        }
        recurse(self, 0);
    }
    unsafe fn insert_content(&mut self, content: T) -> *mut Self {
        // println!("insert stage 1");
        let mut current = self as *mut Self;
        let current = loop {
            if content < (*current).content {
                if !(*current).left.is_null() {
                    current = (*current).left;
                } else {
                    (*current).left = Box::into_raw(Box::new(Node::new(content)));
                    (*(*current).left).parent = current;
                    break (*current).left;
                }
            } else {
                if !(*current).right.is_null() {
                    current = (*current).right;
                } else {
                    (*current).right = Box::into_raw(Box::new(Node::new(content)));
                    (*(*current).right).parent = current;
                    break (*current).right;
                }
            }
        };
        unsafe fn insert_strategy<T: std::fmt::Debug>(
            root: *mut Node<T>,
            current: *mut Node<T>,
        ) -> *mut Node<T> {
            // println!("insert stage 2");
            let p = (*current).parent;
            // assert_ne!(p, ptr::null_mut());
            if let Color::Red = (*p).color {
                if p == root {
                    // println!("case root");
                } else {
                    let pp = (*p).parent;
                    // assert_ne!(pp, ptr::null_mut());
                    let f = if p == (*pp).left {
                        (*pp).right
                    } else {
                        (*pp).left
                    };

                    if f.is_null() || Color::Black as u64 == (*f).color as u64 {
                        if p == (*pp).left {
                            if current == (*p).left {
                                // println!("case left-left");
                                (*p).parent = (*pp).parent; // Assign new parents
                                (*pp).parent = p;
                                (*pp).left = (*p).right; // Move values
                                if !(*pp).left.is_null() {
                                    // tell the son that I am his father
                                    (*(*pp).left).parent = pp;
                                }
                                (*p).right = pp;
                                (*p).color = Color::Black;
                                (*pp).color = Color::Red;
                                if (*p).parent.is_null() {
                                    // Root may be changed
                                    return p;
                                } else {
                                    if pp == (*(*p).parent).left {
                                        // Or juste change parent ref
                                        (*(*p).parent).left = p;
                                    } else {
                                        (*(*p).parent).right = p;
                                    }
                                }
                            } else {
                                // println!("case left-right");
                                (*current).parent = pp; // Assign new parents
                                (*p).parent = current;
                                (*pp).left = current; // move values
                                (*p).right = (*current).left;
                                if !(*p).right.is_null() {
                                    // tell the son that I am his father
                                    (*(*p).right).parent = p;
                                }
                                (*current).left = p;
                                return insert_strategy(root, p);
                            }
                        } else {
                            if current == (*p).right {
                                // println!("case right-right");
                                (*p).parent = (*pp).parent; // Assign new parents
                                (*pp).parent = p;
                                (*pp).right = (*p).left; // Move values
                                if !(*pp).right.is_null() {
                                    // tell the son that I am his father
                                    (*(*pp).right).parent = pp;
                                }
                                (*p).left = pp;
                                (*p).color = Color::Black;
                                (*pp).color = Color::Red;
                                // Root may be changed
                                if (*p).parent.is_null() {
                                    return p;
                                } else {
                                    // Or juste change parent ref
                                    if pp == (*(*p).parent).left {
                                        (*(*p).parent).left = p;
                                    } else {
                                        (*(*p).parent).right = p;
                                    }
                                }
                            } else {
                                // println!("case right-left");
                                (*current).parent = pp; // Assign new parents
                                (*p).parent = current;
                                (*pp).right = current; // Move values
                                (*p).left = (*current).right;
                                if !(*p).left.is_null() {
                                    // tell the son that I am his father
                                    (*(*p).left).parent = p;
                                }
                                (*current).right = p;
                                return insert_strategy(root, p);
                            }
                        }
                    } else {
                        // println!("Recolorize");
                        (*p).color = Color::Black;
                        (*f).color = Color::Black;
                        (*pp).color = Color::Red;
                        (*root).color = Color::Black;
                        if pp != root {
                            return insert_strategy(root, pp);
                        }
                    }

                    // Cas 0 : le nœud père p est la racine de l'arbre
                    // Le nœud père devient alors noir. La propriété (2)
                    // est maintenant vérifiée et la propriété (3) le reste.
                    // C'est le seul cas où la hauteur noire de l'arbre augmente.

                    // CAS le frère f de p est rouge
                    // Les nœuds p et f deviennent noirs et leur père pp devient
                    // rouge. La propriété (3) reste vérifiée mais la propriété ne
                    // l'est pas nécessairement. Si le père de pp est aussi rouge.
                    // Par contre, l'emplacement des deux nœuds rouges consécutifs
                    // s'est déplacé vers la racine.

                    // Cas 2 : le frère f de p est noir
                    // Par symétrie on suppose que p est le fils gauche de son père.
                    // L'algorithme distingue à nouveau deux cas suivant que x est le
                    // fils gauche ou le fils droit de p.
                    // Cas 2a : x est le fils gauche de p.
                    // L'algorithme effectue une rotation droite entre p et pp. Ensuite
                    // le nœud p devient noir et le nœud pp devient rouge. L'algorithme
                    // s'arrête alors puisque les propriétés (2) et (3) sont maintenant vérifiées.
                    // Cas 2b : x est le fils droit de p.
                    // L'algorithme effectue une rotation gauche entre x et p de sorte
                    // que p deviennent le fils gauche de x. On est ramené au cas précédent
                    // et l'algorithme effectue une rotation droite entre x et pp. Ensuite
                    // le nœud x devient noir et le nœud pp devient rouge. L'algorithme
                    // s'arrête alors puisque les propriétés (2) et (3) sont maintenant vérifiées.
                }
            } else {
                // println!("Parent is black");
                // Parent is Black, do nothing
            }
            return root;
        }
        insert_strategy(self, current)
    }
}
impl<T: std::fmt::Debug> Drop for Node<T> {
    fn drop(&mut self) {
        // println!("Node droped for value {:?}", self.content);
    }
}

use rand::prelude::*;

fn main() {
    println!("sizeof node u64 : {}", std::mem::size_of::<Node<u64>>());

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

    let mut rnb = BinaryTree::new();
    for val in v.into_iter() {
        // println!("inserting {}", val);
        rnb.insert_content(val);
        // rnb.prefix_dump();
        rnb.check_nodes();
    }
    let iter = rnb.iter();
    for elem in iter {
        println!("{}", elem);
    }
    drop(rnb);

    let mut rng = rand::thread_rng();

    // loop {
    for _j in 0..4096 {
        let mut rnb = BinaryTree::new();
        // let mut rnb = std::collections::BTreeSet::new();
        for _i in 0..4096 {
            let y: u64 = rng.gen(); // generates a float between 0 and 1
                                    // println!("inserting {}", y);
                                    // rnb.insert_content(y);
            rnb.insert_content(y);
            rnb.check_nodes();
            // let mut max = None;
            // for val in rnb.iter() {
            //     if let Some(max) = max {
            //         if val < max {
            //             panic!("Error");
            //         }
            //     }
            //     max = Some(val);
            // }
        }
        // }
    }
}
