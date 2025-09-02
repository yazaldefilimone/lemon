use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

pub struct Ref<T> {
	pointer: NonNull<T>,
}

impl<T> Ref<T> {
	pub fn new(data: T) -> Self {
		let boxed = Box::new(data);
		Ref { pointer: Box::leak(boxed).into() }
	}

	pub fn inner(&self) -> &T {
		unsafe { self.pointer.as_ref() }
	}

	pub fn inner_mut(&mut self) -> &mut T {
		unsafe { self.pointer.as_mut() }
	}
}

impl<T> Clone for Ref<T> {
	fn clone(&self) -> Self {
		Self { pointer: self.pointer }
	}
}

impl<T: PartialEq> PartialEq for Ref<T> {
	fn eq(&self, other: &Self) -> bool {
		self.inner().eq(other.inner())
	}
}
impl<T: Eq> Eq for Ref<T> {}
impl<T: Hash> Hash for Ref<T> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.inner().hash(state)
	}
}
impl<T: Debug> Debug for Ref<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.inner().fmt(f)
	}
}
impl<T> Deref for Ref<T> {
	type Target = T;
	fn deref(&self) -> &Self::Target {
		self.inner()
	}
}
impl<T> DerefMut for Ref<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.inner_mut()
	}
}

pub struct SliceRef<T> {
	data: NonNull<[T]>,
}

impl<T> SliceRef<T> {
	pub fn from_vec(vec: Vec<T>) -> Self {
		SliceRef { data: vec.leak().into() }
	}

	pub fn new_empty() -> Self {
		let empty: &mut [T] = &mut [];
		SliceRef { data: empty.into() }
	}
}

impl<T> Clone for SliceRef<T> {
	fn clone(&self) -> Self {
		SliceRef { data: self.data }
	}
}
impl<T: PartialEq> PartialEq for SliceRef<T> {
	fn eq(&self, other: &Self) -> bool {
		unsafe { self.data.as_ref() == other.data.as_ref() }
	}
}
impl<T: Eq> Eq for SliceRef<T> {}
impl<T: Hash> Hash for SliceRef<T> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.data.hash(state)
	}
}
impl<T: Debug> Debug for SliceRef<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		unsafe { self.data.as_ref() }.fmt(f)
	}
}
impl<T> Deref for SliceRef<T> {
	type Target = [T];
	fn deref(&self) -> &Self::Target {
		unsafe { self.data.as_ref() }
	}
}
impl<T> DerefMut for SliceRef<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { self.data.as_mut() }
	}
}
