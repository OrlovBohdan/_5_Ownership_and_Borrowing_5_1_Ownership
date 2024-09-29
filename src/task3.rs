#[test]

/*

fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.into_bytes();
    s
}
*/



fn main() {
    let s = give_ownership();
    println!("{}", s);
}

fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Преобразуем строку в Vec без перемещения оригинальной строки
    let _s = s.clone().into_bytes(); // Клонируем строку для преобразования в байты
     s // Возвращаем оригинальную строку
}
/*
let _s = s.clone().into_bytes(); создаёт клон оригинальной строки, что позволяет преобразовать её в Vec<u8>, не теряя оригинал.
*/
