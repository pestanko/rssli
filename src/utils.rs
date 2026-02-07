use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

pub type HierCellMapRef<K, V> = Rc<RefCell<HierCellMap<K, V>>>;

#[derive(Clone)]
pub struct HierCellMap<K, V>
where
    K: Clone,
    V: Clone,
{
    parent: Option<HierCellMapRef<K, V>>,
    data: HashMap<K, V>,
}

impl<K, V> Default for HierCellMap<K, V>
where
    K: Clone,
    V: Clone,
{
    fn default() -> Self {
        Self {
            parent: Default::default(),
            data: Default::default(),
        }
    }
}

impl<K, V> HierCellMap<K, V>
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    pub fn get(&self, name: &K) -> Option<V> {
        if let Some(data) = self.data.get(name) {
            return Some(data.clone());
        }

        if let Some(parent) = self.parent.as_ref() {
            return parent.borrow().get(name);
        }

        None
    }

    pub fn update(&mut self, name: &K, value: &V) {
        if let Some(data) = self.data.get_mut(name) {
            *data = value.clone();
        }

        if let Some(parent) = self.parent.as_ref() {
            parent.borrow_mut().update(name, value)
        }
    }

    pub fn set(&mut self, name: &K, value: &V) {
        self.data.insert(name.clone(), value.clone());
    }

    pub fn set_or_update(&mut self, name: &K, value: &V) -> bool {
        if self.data.contains_key(name) {
            self.data.insert(name.clone(), value.clone());
            return true;
        }
        if let Some(parent) = self.parent.as_ref() {
            if parent.borrow_mut().set_or_update(name, value) {
                return true;
            }
        }
        self.data.insert(name.clone(), value.clone());
        true
    }

    pub fn unset(&mut self, name: &K) {
        self.data.remove(name);
    }

    pub fn keys(&self) -> Vec<K> {
        let mut keys: Vec<K> = self.data.keys().cloned().collect();

        if let Some(parent) = self.parent.as_ref() {
            let parent_keys: Vec<K> = parent.borrow().keys();
            keys.extend(parent_keys);
        }

        keys
    }

    pub fn all(&self) -> HashMap<K, V> {
        let mut data = self.data.clone();

        if let Some(parent) = self.parent.as_ref() {
            let parent_keys = parent.borrow().all();
            data.extend(parent_keys);
        }

        data
    }
}

#[derive(Clone)]
pub struct HierCellMapWrap<K, V>(HierCellMapRef<K, V>)
where
    K: Clone,
    V: Clone;

impl<K, V> HierCellMapWrap<K, V>
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    pub fn new(parent: Option<HierCellMapRef<K, V>>) -> Self {
        Self(Rc::new(RefCell::new(HierCellMap {
            parent,
            data: Default::default(),
        })))
    }

    pub fn new_child(&self) -> Self {
        Self::new(Some(self.0.clone()))
    }

    pub fn new_root() -> Self {
        Self::new(None)
    }

    pub fn get(&self, name: &K) -> Option<V> {
        self.0.borrow().get(name)
    }

    pub fn update(&mut self, name: &K, value: &V) {
        self.0.borrow_mut().update(name, value)
    }

    pub fn set(&mut self, name: &K, value: &V) {
        self.0.borrow_mut().set(name, value)
    }

    pub fn set_or_update(&mut self, name: &K, value: &V) {
        self.0.borrow_mut().set_or_update(name, value);
    }

    pub fn unset(&mut self, name: &K) {
        self.0.borrow_mut().unset(name)
    }

    pub fn keys(&self) -> Vec<K> {
        self.0.borrow().keys()
    }

    pub fn data(&self) -> HashMap<K, V> {
        self.0.borrow().data.clone()
    }

    pub fn all(&self) -> HashMap<K, V> {
        self.0.borrow().all()
    }

    pub fn curr_is_empty(&self) -> bool {
        self.0.borrow().data.is_empty()
    }
}
