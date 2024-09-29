#[test]

/*
// Don't use clone ,use copy instead
fn main() {
    let x = (1, 2, (), "hello".to_string());
    let y = x.clone();
    println!("{:?}, {:?}", x, y);
}
*/

fn main() {
    let x = (1, 2, (), "hello"); // Используем &str вместо String
    let y = x; // Перемещаем x в y
    println!("{:?}, {:?}", x, y); // Теперь x больше недоступен
}

/*
String на строковый срез &str, который поддерживает трейт Copy.
После этой замены можно переместить кортеж x в y, и код будет работать без ошибок.
*/
