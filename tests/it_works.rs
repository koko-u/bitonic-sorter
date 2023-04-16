use crate::students::Student;
use assert2::check;
use bitonic_sorter::sort;

mod students;

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

#[test]
fn students_sort() {
    let mut students = vec![
        Student::new("山田", "太郎", 15),
        Student::new("加藤", "祐介", 13),
        Student::new("清水", "香菜", 17),
        Student::new("大田", "しのぶ", 21),
        Student::new("安元", "希", 10),
        Student::new("橘", "芳樹", 14),
        Student::new("木村", "肇", 20),
        Student::new("船場", "貴美子", 19),
    ];

    sort(&mut students).by(|s1, s2| s1.age().cmp(&s2.age()));

    check!(
        students
            == vec![
                Student::new("安元", "希", 10),
                Student::new("加藤", "祐介", 13),
                Student::new("橘", "芳樹", 14),
                Student::new("山田", "太郎", 15),
                Student::new("清水", "香菜", 17),
                Student::new("船場", "貴美子", 19),
                Student::new("木村", "肇", 20),
                Student::new("大田", "しのぶ", 21),
            ]
    );
}

#[test]
fn students_sort_by_key() {
    let mut students = vec![
        Student::new("山田", "太郎", 15),
        Student::new("加藤", "祐介", 13),
        Student::new("大田", "しのぶ", 21),
        Student::new("清水", "香菜", 17),
    ];

    sort(&mut students).by_key(Student::age);

    check!(
        students
            == vec![
                Student::new("加藤", "祐介", 13),
                Student::new("山田", "太郎", 15),
                Student::new("清水", "香菜", 17),
                Student::new("大田", "しのぶ", 21),
            ]
    );
}
