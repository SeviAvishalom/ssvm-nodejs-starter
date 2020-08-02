<<<<<<< HEAD
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
    println!("The Rust function say() received {}", s);
    let r = String::from("This is my first Rust program! Hello ");
    return r + s;
}
=======
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
  let r = String::from("hello ");
  return r + s;
}
>>>>>>> e2b488f359c8c4944c6ed9f84d7b737d00ce2f82
