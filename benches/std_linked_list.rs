/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#![cfg_attr(feature = "fatal-warnings", deny(warnings))]

#[macro_use]
extern crate bencher;

mod utils;

use bencher::{black_box, Bencher};
use std::collections::LinkedList;
use utils::iterations;
use utils::BencherNoDrop;

fn std_linked_list_push_front(bench: &mut Bencher) {
    let limit = iterations(100_000);

    bench.iter_no_drop(|| {
        let mut linked_list: LinkedList<usize> = LinkedList::new();

        for i in 0..limit {
            linked_list.push_front(i);
        }

        linked_list
    });
}

fn std_linked_list_push_back(bench: &mut Bencher) {
    let limit = iterations(100_000);

    bench.iter_no_drop(|| {
        let mut linked_list: LinkedList<usize> = LinkedList::new();

        for i in 0..limit {
            linked_list.push_back(i);
        }

        linked_list
    });
}

fn std_linked_list_iterate(bench: &mut Bencher) {
    let limit = iterations(100_000);
    let mut linked_list: LinkedList<usize> = LinkedList::new();

    for i in 0..limit {
        linked_list.push_back(i);
    }

    bench.iter(|| {
        for i in &linked_list {
            black_box(i);
        }
    });
}

benchmark_group!(
    benches,
    std_linked_list_push_front,
    std_linked_list_push_back,
    std_linked_list_iterate
);
benchmark_main!(benches);
