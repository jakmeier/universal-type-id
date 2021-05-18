# Universal Type ID

A prototype for a type ID that can be shared between binaries.

```rust
#[derive(UniversalType)]
struct Person {
    name: String,
    year: i16,
}

fn main() {
    let uid = UniversalTypeId::of::<Person>();
    println!("Numerical value of universal type ID: {}", uid.as_u128());
}
```

So far, the prototype only supports struct types. Enum and union support could be added easily.

Support for generic parameters is planned but I haven't quite figured out yet how to do it.

Other types (functions, closures, trait objects etc.) are most likely out of scope. I think that because I don't see an obvious way how it could be done and it's also difficult to find real use cases.