use std::collections::HashMap;
struct SmallMsg<T> {
    message: T,
}
fn main() {
    let mut hash_map: HashMap<&str, &str> = HashMap::new();
    hash_map.insert("hi", "there");

    let key: &str = "nice";
    let value: &str = "cool";
    hash_map.insert(key, value);

    let msg = SmallMsg {
        message: "hi there",
    };
    let msg2: SmallMsg<i32> = SmallMsg { message: 43 };

    println!("{}", msg.message);
    println!("{:#?}", hash_map);

    let mut overflow: u32 = 4294967295;

    let mut user_input = String::new();

    println!("Enter a number:");
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading line");

    let user_num: u32 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    overflow += user_num;

    println!("Total -> {}", overflow);
}
