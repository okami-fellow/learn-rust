fn main() {
    let name = "дружбан".to_string();
    println!("Привет, мой {name} (^_^)/\n Вот и пример релиза.");

    let mut x = 1;
    {
        let mut x = x;
        x += 2;
    }
    println!("{x}");
}
