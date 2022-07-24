pub struct BinaryTree<T: std::fmt::Debug> {
    data: Vec<Node<T>>,
    root: Option<usize>,
    n: usize,
}

pub struct BinaryTreeIterator<'a, T: std::fmt::Debug> {
    data: &'a Vec<Node<T>>,
    x: usize,
    stack: Vec<usize>,
}

impl<T: std::cmp::PartialOrd + std::fmt::Debug> BinaryTree<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            root: None,
            n: 0,
        }
    }
    pub fn iter<'a>(&'a self) -> BinaryTreeIterator<'a, T> {
        BinaryTreeIterator {
            data: &self.data,
            x: self.root.unwrap_or(usize::MAX),
            stack: Vec::new(),
        }
    }

    fn prefix_dump_recurse(&self, x: usize, level: u32) {
        if self.data[x].left != usize::MAX {
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
        if self.data[x].right != usize::MAX {
            self.prefix_dump_recurse(self.data[x].right, level + 1);
        }
    }

    pub fn prefix_dump(&self) {
        if let Some(index) = self.root {
            self.prefix_dump_recurse(index, 0);
        }
    }

    fn insert_content_recurse(&mut self, x: usize) {
        // println!("insert stage 2");
        let p = self.data[x].parent;
        // assert_ne!(p, ptr::null_mut());
        if let Color::Red = self.data[p].color {
            if p == self.root.unwrap() {
                // println!("case root");
            } else {
                let pp = self.data[p].parent;
                // assert_ne!(pp, ptr::null_mut());
                let f = if p == self.data[pp].left {
                    self.data[pp].right
                } else {
                    self.data[pp].left
                };

                if f == usize::MAX || Color::Black as u64 == self.data[f].color as u64 {
                    if p == self.data[pp].left {
                        if x == self.data[p].left {
                            // println!("case left-left");
                            self.data[p].parent = self.data[pp].parent; // Assign new parents
                            self.data[pp].parent = p;
                            self.data[pp].left = self.data[p].right; // Move values
                            let left_index = self.data[pp].left;
                            if left_index != usize::MAX {
                                // tell the son that I am his father
                                self.data[left_index].parent = pp;
                            }
                            self.data[p].right = pp;
                            self.data[p].color = Color::Black;
                            self.data[pp].color = Color::Red;
                            if self.data[p].parent == usize::MAX {
                                // Root may be changed
                                self.root = Some(p);
                            } else {
                                let new_parent = self.data[p].parent;
                                if pp == self.data[new_parent].left {
                                    // Or juste change parent ref
                                    self.data[new_parent].left = p;
                                } else {
                                    self.data[new_parent].right = p;
                                }
                            }
                        } else {
                            // println!("case left-right");
                            self.data[x].parent = pp; // Assign new parents
                            self.data[p].parent = x;
                            self.data[pp].left = x; // move values
                            self.data[p].right = self.data[x].left;
                            let right_index = self.data[p].right;
                            if right_index != usize::MAX {
                                // tell the son that I am his father
                                self.data[right_index].parent = p;
                            }
                            self.data[x].left = p;
                            return self.insert_content_recurse(p);
                        }
                    } else {
                        if x == self.data[p].right {
                            // println!("case right-right");
                            self.data[p].parent = self.data[pp].parent; // Assign new parents
                            self.data[pp].parent = p;
                            self.data[pp].right = self.data[p].left; // Move values
                            let right_index = self.data[pp].right;
                            if right_index != usize::MAX {
                                // tell the son that I am his father
                                self.data[right_index].parent = pp;
                            }
                            self.data[p].left = pp;
                            self.data[p].color = Color::Black;
                            self.data[pp].color = Color::Red;
                            // Root may be changed
                            if self.data[p].parent == usize::MAX {
                                self.root = Some(p);
                            } else {
                                let new_parent = self.data[p].parent;
                                // Or juste change parent ref
                                if pp == self.data[new_parent].left {
                                    self.data[new_parent].left = p;
                                } else {
                                    self.data[new_parent].right = p;
                                }
                            }
                        } else {
                            // println!("case right-left");
                            self.data[x].parent = pp; // Assign new parents
                            self.data[p].parent = x;
                            self.data[pp].right = x; // Move values
                            self.data[p].left = self.data[x].right;
                            let left_index = self.data[p].left;
                            if left_index != usize::MAX {
                                // tell the son that I am his father
                                self.data[left_index].parent = p;
                            }
                            self.data[x].right = p;
                            return self.insert_content_recurse(p);
                        }
                    }
                } else {
                    // println!("Recolorize");
                    self.data[p].color = Color::Black;
                    self.data[f].color = Color::Black;
                    self.data[pp].color = Color::Red;
                    self.data[self.root.unwrap()].color = Color::Black;
                    if pp != self.root.unwrap() {
                        return self.insert_content_recurse(pp);
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
    }

    pub fn insert_content(&mut self, content: T) {
        self.n += 1;
        match self.root {
            Some(mut index) => {
                // println!("insert stage 1");
                index = loop {
                    if content < self.data[index].content {
                        if self.data[index].left != usize::MAX {
                            index = self.data[index].left;
                        } else {
                            self.data.push(Node::new(content));
                            let new_index = self.data.len() - 1;
                            self.data[index].left = new_index;
                            self.data[new_index].parent = index;
                            break new_index;
                        }
                    } else {
                        if self.data[index].right != usize::MAX {
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
                self.insert_content_recurse(index);
            }
            None => {
                self.data.push(Node::new(content));
                self.data[0].color = Color::Black;
                self.root = Some(0);
            }
        };
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
        if parent != usize::MAX {
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
        let black_left = if x_ref.left != usize::MAX {
            self.check_nodes_recurse(x_ref.left, black_nodes, acc, color)
        } else {
            black_nodes
        };
        let black_right = if x_ref.right != usize::MAX {
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

impl<'a, T: std::fmt::Debug> Iterator for BinaryTreeIterator<'a, T> {
    // we will be counting with usize
    type Item = &'a T;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        while !self.stack.is_empty() || self.x != usize::MAX {
            if self.x != usize::MAX {
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

impl<T: std::fmt::Debug> Drop for BinaryTree<T> {
    fn drop(&mut self) {}
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
    parent: usize,
    left: usize,
    right: usize,
}

impl<T: std::cmp::PartialOrd + std::fmt::Debug> Node<T> {
    fn new(content: T) -> Self {
        Self {
            content,
            color: Color::Red,
            parent: usize::MAX,
            left: usize::MAX,
            right: usize::MAX,
        }
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
        rnb.prefix_dump();
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
            // rnb.prefix_dump();
            // rnb.check_nodes();
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
    }
}
