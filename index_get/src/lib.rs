use std::ops::Index;

pub trait IndexGet<C> : Sized 
    where C: Index<Self>
{
    fn get(self, collection: &C) -> &C::Output  { collection.index(self) }
}
impl<I, C> IndexGet<C> for I where I: core::slice::SliceIndex<C>, C: Index<I> {}
