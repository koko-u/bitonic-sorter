use crate::sorting::Sorting;

mod sorting;

pub fn sort<T>(data: &mut [T]) -> Sorting<T> {
    Sorting { target: data }
}
