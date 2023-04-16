pub struct Sorting<'a, T> {
    pub target: &'a mut [T],
}

impl<'a, T: Ord> Sorting<'a, T> {
    pub fn ascending(self) {
        core::sort(self.target, true)
    }
    pub fn descending(self) {
        core::sort(self.target, false)
    }
}

mod core;
