pub struct Lookup {
    arr: Vec<u32>,
}

impl Lookup {
    pub fn new(size: usize) -> Self {
        let max = size.max(1);
        let len = (max / 2 + 1) as usize;
        Lookup { arr: vec![0; len] }
    }

    pub fn get(&self, x: u64) -> Option<&u32> {
        if x < self.arr.len() as u64 {
            let idx = (x / 2) as usize;
            match self.arr.get(idx) {
                Some(0) => None,
                val => val,
            }
        } else {
            None
        }
    }

    pub fn merge(&mut self, new_items: &[(u64, u32)], total_len: u32) {
        for (item, len_so_far) in new_items.iter().copied() {
            let length = total_len - len_so_far;
            // Save only odd values (even values would collide on the same index)
            if item < self.arr.len() as u64 && item & 1 == 1 {
                let idx = (item / 2) as usize;
                self.arr[idx] = length;
            }
        }
    }
}
