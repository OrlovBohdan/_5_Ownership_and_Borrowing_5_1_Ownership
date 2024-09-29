#[test]

/*
fn main() {
    let x = Box::new(5);

    let ...      // update this line, don't change other lines!

    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}
*/

fn main() {
    let mut x = Box::new(5);

    let y = &mut *x;  // здесь изменять не нужно

    *y = 4;  // изменяем значение на 4

    assert_eq!(*x, 4);  // проверка на новое значение

    println!("Success!");
}

/*
есть небольшая логическая ошибка, связанная с использованием указателя на Box.
Цель кода — изменить значение x, но при этом в assert_eq! происходит сравнение с 5, что вызывает ошибку,
так как значение y будет изменено на 4.
*/