use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

use dashmap::DashMap;
use rayon::prelude::*;


// item insert tests
pub fn hashmap_simple(r_max: usize) {
    let mut benchmap = HashMap::new();
    for num in 0..r_max {
        benchmap.insert(num, num);
    }
}

pub fn hashmap_prealloc(r_max: usize) {
    let mut benchmap = HashMap::with_capacity(r_max);
    for num in 0..r_max {
        benchmap.insert(num, num);
    }
}

pub fn dashmap_simple(r_max: usize) {
    let benchmap = DashMap::new();
    for num in 0..r_max {
        benchmap.insert(num, num);
    }
}

pub fn dashmap_prealloc(r_max: usize) {
    let benchmap = DashMap::with_capacity(r_max);
    for num in 0..r_max {
        benchmap.insert(num, num);
    }
}

pub fn hashmap_threaded(r_max: usize) {
    let benchmap = Arc::new(Mutex::new(HashMap::new()));

    (0..r_max).into_par_iter().for_each(|i| {
        benchmap.lock().unwrap().insert(i, i);
    });
}

pub fn hashmap_threaded_rwlock(r_max: usize) {
    let benchmap = RwLock::new(HashMap::new());

    (0..r_max).into_par_iter().for_each(|i| {
        benchmap.write().unwrap().insert(i, i);
    });
}

pub fn dashmap_threaded(r_max: usize) {
    let benchmap = DashMap::new();

    (0..r_max).into_par_iter().for_each(|i| {
        benchmap.insert(i, i);
    });
}

pub fn hashmap_range_query_simple(r_max: usize) {
    let mut benchmap = HashMap::new();
    for num in 0..r_max {
        benchmap.insert(num, num);
    }
    let _ = benchmap.iter().filter(|(k, _)| *k > &(r_max/2)).map(|(_, v)| *v).sum::<usize>();
}