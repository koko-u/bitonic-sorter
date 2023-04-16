pub fn sort_by<T, F>(target: &mut [T], comparer: &F, up: bool)
where
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    if target.len() <= 1 {
        return;
    }

    let mid_point = target.len() / 2;
    sort_by(&mut target[..mid_point], comparer, true);
    sort_by(&mut target[mid_point..], comparer, false);
    sub_sort_by(target, comparer, up);
}
fn sub_sort_by<T, F>(target: &mut [T], comparer: &F, up: bool)
where
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    if target.len() <= 1 {
        return;
    }

    compare_and_swap_by(target, comparer, up);

    let mid_point = target.len() / 2;
    sub_sort_by(&mut target[..mid_point], comparer, up);
    sub_sort_by(&mut target[mid_point..], comparer, up);
}
fn compare_and_swap_by<T, F>(target: &mut [T], comparer: &F, up: bool)
where
    F: for<'a> Fn(&'a T, &'a T) -> std::cmp::Ordering,
{
    use std::cmp::Ordering::*;

    let mid_point = target.len() / 2;
    for i in 0..mid_point {
        match (comparer(&target[i], &target[i + mid_point]), up) {
            (Greater, true) | (Less, false) => target.swap(i, i + mid_point),
            _ => (),
        }
    }
}

// pub fn sort<U: Ord>(target: &mut [U], up: bool) {
//     if target.len() <= 1 {
//         return;
//     }
//
//     let mid_point = target.len() / 2;
//     sort(&mut target[..mid_point], true);
//     sort(&mut target[mid_point..], false);
//     sub_sort(target, up);
// }
//
// fn sub_sort<U: Ord>(target: &mut [U], up: bool) {
//     if target.len() <= 1 {
//         return;
//     }
//
//     compare_and_swap(target, up);
//
//     let mid_point = target.len() / 2;
//     sub_sort(&mut target[..mid_point], up);
//     sub_sort(&mut target[mid_point..], up);
// }
//
// fn compare_and_swap<U: Ord>(target: &mut [U], up: bool) {
//     use std::cmp::Ordering::*;
//
//     let mid_point = target.len() / 2;
//     for i in 0..mid_point {
//         match (target[i].cmp(&target[i + mid_point]), up) {
//             (Greater, true) | (Less, false) => {
//                 target.swap(i, i + mid_point);
//             }
//             _ => (),
//         }
//     }
// }
