use std::{
	cell::{Ref, RefCell, RefMut},
	collections::HashMap,
	hash::Hash,
};

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexError {
	#[error("item not found")]
	NotFound,
	#[error("item is referenced")]
	RefBefore,
}
use IndexError::*;

#[derive(Debug, Default, Clone)]
pub struct CellMap<K, V> {
	map: HashMap<K, RefCell<V>>,
}

impl<K: Eq + Hash, V> CellMap<K, V> {
	pub fn new() -> Self {
		Self { map: HashMap::new() }
	}
	pub fn with_capacity(capacity: usize) -> Self {
		Self { map: HashMap::with_capacity(capacity) }
	}
	pub fn capacity(&self) -> usize {
		self.map.capacity()
	}
	pub fn len(&self) -> usize {
		self.map.len()
	}
	pub fn is_empty(&self) -> bool {
		self.map.is_empty()
	}
	pub fn has_mut(&self) -> bool {
		self.map.values().any(|v| v.try_borrow_mut().is_ok())
	}
	pub fn contains_key(&self, key: &K) -> bool {
		self.map.contains_key(key)
	}
	pub fn is_mut(&self, key: &K) -> bool {
		match self.map.get(key) {
			Some(v) => v.try_borrow_mut().is_ok(),
			_ => false,
		}
	}
	pub fn get(&self, key: &K) -> Result<Ref<'_, V>, IndexError> {
		match self.map.get(key) {
			Some(v) => v.try_borrow().or(Err(RefBefore)),
			_ => Err(NotFound),
		}
	}
	pub fn get_mut(&self, key: &K) -> Result<RefMut<'_, V>, IndexError> {
		match self.map.get(key) {
			Some(v) => v.try_borrow_mut().or(Err(RefBefore)),
			_ => Err(NotFound),
		}
	}
	pub fn insert(&mut self, key: K, value: V) {
		self.map.insert(key, RefCell::new(value));
	}
	pub fn remove(&mut self, key: &K) -> Option<V> {
		self.map.remove(key).map(|v| v.into_inner())
	}
	pub fn clear(&mut self) {
		self.map.clear();
	}
	pub fn keys(&self) -> impl Iterator<Item = &K> {
		self.map.keys()
	}
	pub fn values(&self) -> impl Iterator<Item = Ref<'_, V>> {
		self.map.values().filter_map(|v| v.try_borrow().ok())
	}
	pub fn values_mut(&self) -> impl Iterator<Item = RefMut<'_, V>> {
		self.map.values().filter_map(|v| v.try_borrow_mut().ok())
	}
	pub fn iter(&self) -> impl Iterator<Item = (&K, Ref<'_, V>)> {
		self.map.iter().filter_map(|(k, v)| v.try_borrow().ok().map(|v| (k, v)))
	}
	pub fn iter_mut(&self) -> impl Iterator<Item = (&K, RefMut<'_, V>)> {
		self.map.iter().filter_map(|(k, v)| v.try_borrow_mut().ok().map(|v| (k, v)))
	}
	pub fn iter_all(&self) -> impl Iterator<Item = (&K, Result<Ref<'_, V>, IndexError>)> {
		self.map.iter().map(|(k, v)| (k, v.try_borrow().or(Err(RefBefore))))
	}
	pub fn iter_all_mut(&self) -> impl Iterator<Item = (&K, Result<RefMut<'_, V>, IndexError>)> {
		self.map.iter().map(|(k, v)| (k, v.try_borrow_mut().or(Err(RefBefore))))
	}
}

impl<K: Eq + Hash, V> Into<HashMap<K, RefCell<V>>> for CellMap<K, V> {
	fn into(self) -> HashMap<K, RefCell<V>> {
		self.map
	}
}
impl<K: Eq + Hash, V> FromIterator<(K, V)> for CellMap<K, V> {
	fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
		Self { map: HashMap::from_iter(iter.into_iter().map(|(k, v)| (k, RefCell::new(v)))) }
	}
}
