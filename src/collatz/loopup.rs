pub struct Lookup {
    arr: Vec<i64>,
}

impl Lookup {
    pub fn new(size: i64) -> Self {
        let size = size.max(0) as usize;
        Lookup { arr: vec![0; size] }
    }

    pub fn get(&self, x: i64) -> Option<&i64> {
        if x >= 0 && x < self.arr.len() as i64 {
            match self.arr.get(x as usize) {
                Some(0) => None,
                val => val,
            }
        } else {
            None
        }
    }

    pub fn merge(&mut self, new_items: Vec<i64>, length: i64) {
        for (i, item) in new_items.iter().enumerate() {
            let length = length - i as i64;
            if *item >= 0 && *item < self.arr.len() as i64 {
                self.arr[*item as usize] = length;
            }
        }
    }
}
