use std::ops::Index;
use index_get::IndexGet;

pub struct CustomIndex(usize);

impl<T> Index<CustomIndex> for Vec<T>
{
    type Output = T;

    fn index(&self, index: CustomIndex) -> &Self::Output {
        self.index(index.0)
    }
}

impl<C> IndexGet<C> for CustomIndex where C: Index<CustomIndex>
{

}