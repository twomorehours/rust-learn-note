use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

#[derive(Default)]
struct Store1 {
    set1: HashSet<i32>,
    set2: HashSet<i32>,
}

impl Store1 {
    fn save1(&mut self, val: i32) {
        self.set1.insert(val);
    }

    fn save2(&mut self, val: i32) {
        self.set2.insert(val);
    }
}

#[derive(Default)]
struct Store2 {
    set1: Mutex<HashSet<i32>>,
    set2: Mutex<HashSet<i32>>,
}

impl Store2 {
    fn save1(&self, val: i32) {
        let mut set1 = self.set1.lock().unwrap();
        set1.insert(val);
    }

    fn save2(&self, val: i32) {
        let mut set2 = self.set2.lock().unwrap();
        set2.insert(val);
    }
}

fn main() {
    t1();
    // t2();
}

fn t1() {
    let start = Instant::now();

    let store1 = Arc::new(Mutex::new(Store1::default()));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let store_c = store1.clone();
        let handle = thread::spawn(move || {
            for j in 0..1000000 {
                let mut g = store_c.lock().unwrap();
                if j % 2 == 0 {
                    g.save1(j);
                } else {
                    g.save2(j);
                }
            }
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());

    let duration = Instant::now().duration_since(start);
    println!("{:?}", duration);
}

fn t2() {
    let start = Instant::now();

    let store2 = Arc::new(Store2::default());
    let mut handles = Vec::new();

    for _ in 0..10 {
        let store_c = store2.clone();
        let handle = thread::spawn(move || {
            for j in 0..1000000 {
                if j % 2 == 0 {
                    store_c.save1(j);
                } else {
                    store_c.save2(j);
                }
            }
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());

    let duration = Instant::now().duration_since(start);
    println!("{:?}", duration);
}
