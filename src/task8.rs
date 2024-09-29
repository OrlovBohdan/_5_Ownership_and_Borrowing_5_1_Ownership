#[test]

/*

fn main() {
   let t = (String::from("hello"), String::from("world"));

   let _s = t.0;

   // Modify this line only, don't use `_s`
   println!("{:?}", t);
}
*/


fn main() {
    let t = (String::from("hello"), String::from("world"));

    let _s = t.0.clone();

    // Modify this line only, don't use `_s`
    println!("{:?}", t);
}

/*
В коде на Rust используется кортеж, содержащий два String.
Когда присвоение t.0 переменной _s, это перемещение значения приводит к тому, что t.0 больше недоступно.
Если затем попытаеться напечатать t, программа вызовет ошибку компиляции.
*/


/*fn main() {
    let t = (String::from("hello"), String::from("world"));

    let _s = &t.0;

    // Modify this line only, don't use `_s`
    println!("{:?}", t);
}*/