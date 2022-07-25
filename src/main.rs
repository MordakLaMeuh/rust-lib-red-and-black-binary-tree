mod btree;
mod raw_vec;

use btree::BinaryTree;
use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();

    // let mut v = RawVec::new();
    // for i in 0..1024 {
    //     v.push(i);
    //     // dbg!(v.get_mut(v.len() - 1));
    // }
    // for _i in 0..1024 {
    //     v.swap_remove(0);
    //     // dbg!(v.swap_remove(0));
    // }
    // dbg!(v.len());

    // let mut v = RawVec::new();
    // let mut max_alloc = 0;
    // for _i in 0..1024 * 1024 {
    //     let b: bool = rng.gen();
    //     if b {
    //         let val: u64 = rng.gen();
    //         v.push(val);
    //         max_alloc = std::cmp::max(max_alloc, v.len());
    //     } else {
    //         let len = v.len();
    //         if len > 0 {
    //             let idx = rng.gen::<usize>() % len;
    //             v.swap_remove(idx);
    //         }
    //     }
    // }
    // println!("Max alloc: {}", max_alloc);

    // println!("sizeof node u64 : {}", std::mem::size_of::<Node<u64>>());

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
    for val in v.iter() {
        // println!("inserting {}", val);
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
        dbg!(rnb.remove(e));
        rnb.prefix_dump();
    }

    for _i in 0..256 {
        let mut v = Vec::new();
        for _j in 0..64 {
            v.push(rng.gen::<f64>());
        }
        let mut rnb = BinaryTree::new();
        for val in v.iter() {
            // println!("inserting {}", val);
            rnb.insert(*val);
            rnb.prefix_dump();
            rnb.check_nodes();
        }
        let iter = rnb.iter();
        for elem in iter {
            println!("{}", elem);
        }
        dbg!(rnb.remove(&0.1927782076332135));
        let mut v_acc = v.len();
        for e in v.iter() {
            v_acc -= 1;
            rnb.prefix_dump();
            let b = dbg!(rnb.remove(e));
            assert_eq!(b, true);
            let mut max = None;
            let mut acc = v_acc;
            for g in rnb.iter() {
                dbg!(g);
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

    drop(rnb);
    // return;

    // loop {
    for _j in 0..1 {
        let mut rnb = BinaryTree::new();
        //let mut rnb = std::collections::BTreeSet::new();
        for _i in 0..4096 {
            let y: u64 = rng.gen(); // generates a float between 0 and 1
                                    // println!("inserting {}", y);
                                    // rnb.insert_content(y);
            rnb.insert(y);
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
