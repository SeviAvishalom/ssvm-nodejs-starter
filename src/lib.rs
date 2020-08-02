use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
    println!("The Rust function say() received {}", s);
    let r = String::from("This is my first Rust program! Hello ");
    return r + s;
}
