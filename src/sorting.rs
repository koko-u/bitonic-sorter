pub struct Sorting<'a, T> {
    pub target: &'a mut [T],
}

impl<'a, T: Ord> Sorting<'a, T> {
    pub fn ascending(self) {
        core::sort_by(self.target, &|a, b| a.cmp(&b), true)
    }
    pub fn descending(self) {
        core::sort_by(self.target, &|a, b| a.cmp(&b), false)
    }
}
impl<'a, T> Sorting<'a, T> {
    pub fn by<F>(self, comparer: F)
    where
        F: Fn(&T, &T) -> std::cmp::Ordering,
    {
        core::sort_by(self.target, &comparer, true);
    }
    pub fn by_key<F, U>(self, selector: F)
    where
        F: Fn(&T) -> U,
        U: Ord,
    {
        core::sort_by(self.target, &|a, b| selector(a).cmp(&selector(b)), true);
    }
}

mod core;
