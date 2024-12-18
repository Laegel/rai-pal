use std::{
	borrow::Borrow,
	collections::HashMap,
	fmt::Display,
	hash::{BuildHasher, Hash},
};

use crate::result::{Error, Result};

pub trait TryGettable<K, V> {
	fn try_get<Q>(&self, k: &Q) -> Result<&V>
	where
		K: Borrow<Q> + Display,
		Q: Hash + Eq + Display + ?Sized;
}

impl<K, V, S: BuildHasher> TryGettable<K, V> for HashMap<K, V, S>
where
	K: Hash + Eq + Display,
{
	fn try_get<Q>(&self, key: &Q) -> Result<&V>
	where
		K: Borrow<Q> + Display,
		Q: Hash + Eq + Display + ?Sized,
	{
		self.get(key)
			.ok_or_else(|| Error::DataEntryNotFound(key.to_string()))
	}
}
