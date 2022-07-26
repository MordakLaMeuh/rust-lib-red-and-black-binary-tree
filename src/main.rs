mod btree;
mod raw_vec;

use btree::RBBTree;
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

    let mut rnb = RBBTree::new();
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
        println!("Removing entry {}", e);
        rnb.prefix_dump();
        dbg!(rnb.remove(e));
        rnb.prefix_dump();
        rnb.check_nodes();
        println!("Checked");
    }

    for _i in 0..256 {
        let mut v = Vec::new();
        for _j in 0..64 {
            v.push(rng.gen::<f64>());
        }
        let mut rnb = RBBTree::new();
        for val in v.iter() {
            // println!("inserting {}", val);
            rnb.insert(*val);
            // rnb.prefix_dump();
            rnb.check_nodes();
        }
        assert_eq!(rnb.remove(&13.1927782076332135), false);
        let mut v_acc = v.len();
        v.shuffle(&mut thread_rng());
        for e in v.iter() {
            v_acc -= 1;
            // rnb.prefix_dump();
            let b = rnb.remove(e);
            assert_eq!(b, true);
            rnb.check_nodes();
            let mut max = None;
            let mut acc = v_acc;
            for g in rnb.iter() {
                // dbg!(g);
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
    for _j in 0..4096 {
        let mut rnb = RBBTree::new();
        // let mut rnb = std::collections::BTreeSet::new();

        let mut v = Vec::new();
        for _i in 0..4096 {
            v.push(rng.gen::<u64>());
        }
        for elem in v.iter() {
            rnb.insert(*elem);
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
        v.shuffle(&mut thread_rng());
        // let mut v_acc = v.len();
        for elem in v.iter() {
            // v_acc -= 1;
            // assert_eq!(rnb.remove(elem), true);
            rnb.remove(elem);
            // rnb.check_nodes();
            // let mut acc = v_acc;
            // let mut max = None;

            // for g in rnb.iter() {
            //     // dbg!(g);
            //     acc -= 1;
            //     if let Some(max) = max {
            //         if g < max {
            //             panic!("Error for {}", g);
            //         }
            //     }
            //     max = Some(g);
            // }
            // assert_eq!(acc, 0);
        }
    }
}
