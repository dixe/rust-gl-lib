use std::collections::HashMap;

pub struct DataMap<T> {
    pub data: HashMap::<usize, T>,
    next_id: usize
}

impl<T> Default for DataMap<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            next_id: 1
        }
    }
}

impl<T> DataMap<T> {
    pub fn insert(&mut self, data: T) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        self.data.insert(id, data);

        id
    }

    pub fn get(&self, id: &usize) -> Option<&T> {
        self.data.get(&id)
    }

    pub fn get_mut(&mut self, id: &usize) -> Option<&mut T> {
        self.data.get_mut(&id)
    }

    pub fn remove(&mut self, id: &usize) -> Option<T> {
        self.data.remove(id)
    }
}
