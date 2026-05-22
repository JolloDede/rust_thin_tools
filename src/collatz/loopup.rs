#[derive(Debug)]
pub struct Lookup {
    arr: Vec<i32>,
}

impl Lookup {
    pub fn new(size: i64) -> Self {
        let size = size.max(0) as usize;
        Lookup { arr: vec![0; size] }
    }

    pub fn get(&self, x: i64) -> Option<&i32> {
        let index = x / 2;
        if index >= 0 && index < self.arr.len() as i64 {
            match self.arr.get(index as usize) {
                Some(0) => None,
                val => val,
            }
        } else {
            None
        }
    }

    pub fn merge(&mut self, new_items: Vec<i64>, length: i32) {
        for (i, item) in new_items.iter().enumerate() {
            if item % 2 != 0 {
                continue;
            }
            let length = length - i as i32;
            let index = *item / 2;
            if index >= 0 && index < self.arr.len() as i64 {
                self.arr[index as usize] = length;
            }
        }
    }
}
