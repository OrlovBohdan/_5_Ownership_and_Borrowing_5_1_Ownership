#[test]

/*
// Fix the error without removing any code
fn main() {
    let s = String::from("Hello World");

    print_str(s);

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}
*/

//1
/*// Fix the error without removing any code
fn main() {
    let s = String::from("Hello World");

    print_str(s.clone());

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}*/

//2
// Fix the error without removing any code
fn main() {
    let s = String::from("Hello World");

    print_str(&s); // Передаем ссылку на строку

    println!("{}", s); // Теперь строка доступна здесь
}

fn print_str(s: &String)  { // Изменяем тип параметра на ссылку
    println!("{}", s)
}

/*
 print_str(s) на print_str(&s), передавая ссылку на строку вместо её значения.
 параметр функции print_str с String на &String, чтобы принимать ссылку на строку.
*/
