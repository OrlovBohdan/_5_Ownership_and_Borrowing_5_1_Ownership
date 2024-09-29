#[test]


/*fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("Hello world");
    let y = x;
    println!("{}, {}",x, y);
}*/

//1
fn main() {
    let x = String::from("Hello world");
    let y = x.clone(); // Клонируем строку, чтобы сохранить оригинал в x
    println!("{}, {}", x, y); // Теперь x и y могут использоваться
}
//2
/*
fn main() {
    let x = String::from("Hello world");
    let y = (x, String::from("Another string")); // Создаем кортеж
    println!("{}, {}", y.0, y.1); // Используем элементы кортежа
}
*/
//3
/*
fn print_strings(s1: &String, s2: &String) {
    println!("{}, {}", s1, s2);
}

fn main() {
    let x = String::from("Hello world");
    let y = &x; // Создаем ссылку на x
    print_strings(&x, y); // Передаем ссылки в функцию
}
*/
//4
/*
fn main() {
    let x = String::from("Hello world");
    let y = (x, String::from("Another string")); // Создаем кортеж
    println!("{}, {}", y.0, y.1); // Используем элементы кортежа
}
*/

/*
Подход 1 (clone) создает глубокую копию строки, что может быть затратным с точки зрения производительности.
Подход 2 и Подход 3 позволяют избежать перемещения и использовать ссылки, что более эффективно.
Подход 4 показывает, как можно использовать кортеж для работы с несколькими значениями.
*/