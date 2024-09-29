#[test]

/*
// Don't modify code in main!
fn main() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) {
    println!("{}", s);
}
*/

// Don't modify code in main!
fn main() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

/*
Мы изменили сигнатуру функции take_ownership так, чтобы она возвращала тип String (то есть строку).
После вывода строки s в функции, мы возвращаем её, что позволяет s2 в main хранить значение.
*/