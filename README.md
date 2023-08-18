# input-utils
Small package with console input utils.

# Example
```Rust
use input_utils;

fn main() {
  let age: u32 = input_utils::read_input("Enter your age: ", "Invalid input.\n");
  println!("Your age is {}", age);
}
```
