pub struct Lookup {
    arr: Vec<u32>,
}

impl Lookup {
    pub fn new(size: usize) -> Self {
        let max = size.max(1);
        let len = (max / 2 + 1) as usize;
        Lookup { arr: vec![0; len] }
    }

    pub fn get(&self, x: u64) -> Option<u32> {
        let len = self.arr.len() as u64;
        if x < len && (x & 1 == 1) {
            let idx = (x >> 1) as usize;
            let val = self.arr[idx];
            if val == 0 { None } else { Some(val) }
        } else {
            None
        }
    }

    pub fn merge(&mut self, new_items: &[(u64, u32)], total_len: u32) {
        let len = self.arr.len() as u64;
        for (item, len_so_far) in new_items.iter().copied() {
            let length = total_len - len_so_far;
            debug_assert!(item & 1 == 1);
            if item < len {
                let idx = (item >> 1) as usize;
                if self.arr[idx] == 0 {
                    self.arr[idx] = length;
                }
            }
        }
    }
}
