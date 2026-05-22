pub struct Lookup {
    arr: Vec<i32>,
}

impl Lookup {
    pub fn new(size: i64) -> Self {
        let size = size.max(0) as usize;
        let len = size / 2 + 1;
        Lookup { arr: vec![0; len] }
    }

    pub fn get(&self, x: i64) -> Option<&i32> {
        if x < 0 || (x & 1) != 0 {
            return None;
        }
        let index = (x / 2) as usize;
        if index < self.arr.len() {
            match self.arr.get(index) {
                Some(0) => None,
                val => val,
            }
        } else {
            None
        }
    }

    pub fn merge(&mut self, new_items: &[(i64, i32)], total_len: i32) {
        for (item, step) in new_items {
            if *item < 0 || (*item & 1) != 0 {
                continue;
            }
            let index = (*item / 2) as usize;
            if index < self.arr.len() {
                self.arr[index] = total_len - *step;
            }
        }
    }
}
