use std::borrow::BorrowMut;
use std::env::Args;
use std::time::{SystemTime, UNIX_EPOCH};
use log::trace;
use tokio::sync::Mutex;

mod byte_bit;



#[derive(Debug, Default)]
pub struct IdGenerator {
    machine: u16,
    process: u16,
    counter: u32,
    last_time: u64,
}

impl IdGenerator {
    pub fn new(machine: u16, process: u16) -> IdGenerator {
        IdGenerator {
            machine,
            process,
            counter: 0,
            last_time: 0,
        }
    }

    /* time(0) + counter(64) + machine(96) + process(112) */
    pub fn next_id(&mut self) -> u128 {
        let mut time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        //cut off time to 64 bits (if it would ever go above that)
        time = time & 0xffffffffffffff;
        let mut id: u128 = time;
        id += (self.machine as u128) << 112;
        id += (self.process as u128) << 96;
        if self.last_time > time as u64 {
            panic!("time went backwards");
        } else if self.last_time == time as u64 {
            if self.counter == u32::MAX {
                //TODO: sleep
                panic!("counter overflow");
            }
            self.counter += 1;
            id += (self.counter as u128) << 64;
        } else {
            self.counter = 0;
            self.last_time = time as u64;
        }
        //release mutex
        return id;
    }
}

struct Pool<T> {
    generator_fn: fn() -> T,
    max_size: usize,
    free_pool: Vec<T>,
}

impl<T> Pool<T> {
    fn new(max_size: usize, generator: fn() -> T) -> Pool<T> {
        Pool {
            generator_fn: generator,
            max_size,
            free_pool: Vec::new(),
        }
    }
    fn return_to_pool(&mut self, item: T) {
        if self.free_pool.len() < self.max_size {
            self.free_pool.push(item);
        } else {
            drop(item);
        }
    }
    fn get(&mut self) -> PoolGuard<T> {
        let item = self.free_pool.pop();
        let item = match item {
            Some(item) => item,
            None => (self.generator_fn)(),
        };
        return PoolGuard {
            pool: self,
            item: Some(item),
        };
    }
}

struct PoolGuard<'a, T> {
    pool: &'a mut Pool<T>,
    item: Option<T>,
}
impl <'a,T> PoolGuard<'a,T>{
    fn get_mut(&mut self) -> &mut T {
        self.item.as_mut().unwrap()
    }
    fn get(&self) -> &T {
        self.item.as_ref().unwrap()
    }

}

impl<'a, T> Drop for PoolGuard<'a, T> {
    fn drop(&mut self) {
        let item = self.item.take().expect("item already taken");
        self.pool.free_pool.push(item);
    }
}