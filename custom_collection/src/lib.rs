use std::ops::Index;
use index_get::IndexGet;

pub struct CustomIndex(usize);

impl<C> IndexGet<C> for CustomIndex where C: Index<CustomIndex> {}