# StringEnum Derive

This is a procedural macro for deriving a `str_match` method on enums, allowing
you to match string representations to enum variants. This is going to be used
on a future project to allow for better integration of gpt into code without
the use of function calls.

## Usage

To use this procedural macro, simply apply it to your enum:

```rust 

use string_enum::StringEnum;

#[derive(StringEnum)] 
enum MyEnum { 
    Variant1, 
    Variant2, 
    Variant3,
}

fn main() { 
    let variant = MyEnum::str_match("Variant2").unwrap();
    // also adds a to_str method
    println!("Matched variant: {}", variant.to_str()); 
} 


```

The `str_match` method takes a string as an argument and returns an
`Option<Self>`, where `Self` is the enum type. If the string matches one of the
enum variants, it returns `Some(variant)`, otherwise, it returns `None`.

## License

This procedural macro is licensed under the terms of the MIT license. See the
[LICENSE](LICENSE) file for details.
