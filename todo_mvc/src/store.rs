use js_sys::JSON;
use wasm_bindgen::prelude::*;


pub enum ItemUpdate {
    Title { id: String, title: String },
    Completed { id: String, completed: bool },
}

impl ItemUpdate {
    fn id(&self) -> String {
        match self {
            Self::Completed {id, .. } => id.clone(),
            Self::Title {id, .. } => id.clone(),
        }
    }
}

/// Represents a todo item
pub struct Item {
    pub id: String,
    pub title: String,
    pub completed: bool,
}

impl Item {
    pub fn update(&mut self, update: &ItemUpdate) {
        match update {
            ItemUpdate::Title {title, .. } => { 
                self.title = title.to_string(); },
            ItemUpdate::Completed {completed, ..} => {
                self.completed = *completed
            },
        };
    }
}


/// List of the todo items
pub struct ItemList {
    list: Vec<Item>,
}

impl ItemList {
    fn retain<F>(&mut self, f: F) where F: FnMut(&Item) -> bool {
        self.list.retain(f)
    }

    fn iter_mut(&mut self) -> std::slice::IterMut<'_, Item> {
        self.list.iter_mut()
    }
}


pub trait ItemListTrait<T> {
    fn new() -> Self;
    fn get(&self, i: usize) -> Option<&T>;
    fn length(&self) -> usize;
    fn push(&mut self, item: T);
    fn iter(&self) -> std::slice::Iter<'_, T>;
}


impl ItemListTrait<Item> for ItemList {
    fn new() -> ItemList {
        ItemList { list: Vec::new() }
    }
    fn get(&self, i: usize) -> Option<&Item> {
        self.list.get(i)
    }
    fn length(&self) -> usize {
        self.list.len()
    }
    fn push(&mut self, item: Item) {
        self.list.push(item);
    }
    fn iter(&self) -> std::slice::Iter<'_, Item> {
        self.list.iter()
    }
}

use std::iter::FromIterator;
impl<'a> FromIterator<Item> for ItemList {
    fn from_iter<T: IntoIterator<Item = Item>>(iter: T) -> Self {
        let mut c = ItemList::new();
        for i in iter { c.push(i) };
        
        c
    }
}

/// A borrowed set of Items filtered from the store
pub struct ItemListSlice<'a> {
    list: Vec<&'a Item>,
}


impl<'a> ItemListTrait<&'a Item> for ItemListSlice<'a> {
    fn new() -> ItemListSlice<'a> {
        ItemListSlice { list: Vec::new() }
    }
    fn get(&self, i: usize) -> Option<&&'a Item> {
        self.list.get(i)
    }
    fn length(&self) -> usize {
        self.list.len()
    }
    fn push(&mut self, item: &'a Item) {
        self.list.push(item)
    }
    fn iter(&self) -> std::slice::Iter<'_, &'a Item> {
        self.list.iter()
    }
}

pub struct Storage {
    local_storage: web_sys::Storage,
    data: ItemList,
    name: String,
}