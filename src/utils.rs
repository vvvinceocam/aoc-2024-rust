pub(crate) trait Sorted {
    fn sorted(&self) -> Self;
    fn sorted_unstable(&self) -> Self;
}

impl<T> Sorted for Vec<T>
where
    T: Copy + Ord,
{
    fn sorted(&self) -> Self {
        let mut data = self.clone();
        data.sort();
        data
    }

    fn sorted_unstable(&self) -> Self {
        let mut data = self.clone();
        data.sort_unstable();
        data
    }
}
