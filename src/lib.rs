use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

use dashmap::DashMap;
use rayon::prelude::*;

pub fn hashmap_simple(r_max: usize) {
    let mut benchmap = HashMap::new();
    let _ = (0..r_max).map(|num| benchmap.insert(num, num));
}

pub fn hashmap_prealloc(r_max: usize) {
    let mut benchmap = HashMap::with_capacity(r_max);
    let _ = (0..r_max).map(|num| benchmap.insert(num, num));
}

pub fn dashmap_simple(r_max: usize) {
    let benchmap = DashMap::new();
    let _ = (0..r_max).map(|num| benchmap.insert(num, num));
}

pub fn dashmap_prealloc(r_max: usize) {
    let benchmap = DashMap::with_capacity(r_max);
    let _ = (0..r_max).map(|num| benchmap.insert(num, num));
}

pub fn hashmap_threaded(r_max: usize) {
    let benchmap = Arc::new(Mutex::new(HashMap::new()));

    let _ = (0..r_max).into_par_iter().for_each(|i| {
        benchmap.lock().unwrap().insert(i, i);
    });
}

pub fn hashmap_threaded_rwlock(r_max: usize) {
    let benchmap = RwLock::new(HashMap::new());

    let _ = (0..r_max).into_par_iter().for_each(|i| {
        benchmap.write().unwrap().insert(i, i);
    });
}

pub fn dashmap_threaded(r_max: usize) {
    let benchmap = DashMap::new();

    let _ = (0..r_max).into_par_iter().for_each(|i| {
        benchmap.insert(i, i);
    });
}
