use crate::sorting::Sorting;

mod sorting;

pub fn sort<T: Ord>(data: &mut [T]) -> Sorting<T> {
    Sorting { target: data }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn ascending_sort() {
        let mut data = vec![10, 30, 11, 20, 4, 330, 21, 110];

        sort(&mut data).ascending();

        check!(data == vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn descending_sort() {
        let mut data = vec![10, 30, 11, 20, 4, 330, 21, 110];

        sort(&mut data).descending();

        check!(data == vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }
}
