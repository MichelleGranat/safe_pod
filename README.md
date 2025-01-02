# Safe Pod
``safe_pod`` hepls creating types that can be serialized
to a byte array (``&[u8]``) and deserialyzed from one, all
written in safe Rust.

It can particularly useful when parsing binary file formats.

# Getting started
To get started add ``safe_pod`` to your project by running:
```
cargo add safe_pod
```
or by adding the following to your ``Cargo.toml`` file:
```
safe_pod = "0.0.2"
```

# Basic use
The following primitive types implement ``Zeroable`` and ``Pod`` traits:
``bool``, ``u8``, ``u16``, ``u32``, ``u64``, ``u128``, ``i8``, 
``i16``, ``i32``, ``i64``, ``128``, ``f32``, ``f64``.

Any struct where all fields are of types that implement 
``Zeroable`` or ``Pod`` can derive those traits respectively.

```rust
#[derive(Debug, Zeroable, Pod)]
struct Foo {
    a: i8,
    b: bool,
    c: f32,
}

let zeroed_foo = Foo::zeroed();
let foo_from_bytes = Foo::from_le_bytes(&[0, 1, 0, 0, 0, 0])?;
let mut bytes_from_foo = [0u8; Foo::SIZE];
let bytes_written = foo_from_bytes.to_be_bytes(&mut bytes_from_foo)?;

println!("Foo zeroed: {:#?}", zeroed_foo);
println!("Foo from bytes: {:#?}", foo_from_bytes);
println!("Foo wrote {} bytes to byte buffer: {:#?}", bytes_written, bytes_from_foo);
```

All enums with variants where all types implement `Zeroable` can derive it. The "zero variant" must be marked with the `#[zero]` attribute. 

Only (for now) [unit-like enums](https://doc.rust-lang.org/reference/items/enumerations.html#unit-only-enum) may derive the `Pod` trait. The enum must have the `#[pod(...)]` atrribute with the inner attribute `repr($type)` set to a type that implements `Pod`. Every variant must also have the `#[pod(...)]` atrribute with the inner attribute `match_expr($expression)` set to an expression of the type set in `repr($tpye)`.

```rust
#[derive(Debug, Pod, Zeroable)]
#[pod(repr(u32))]
enum UnitLikeEnum {
    #[pod(match_expr(0))]
    #[zero]
    Foo,
    #[pod(match_expr(1))]
    Bar,
}

let zeroed_enum = UnitLikeEnum::zeroed();
let enum_from_bytes = UnitLikeEnum::from_le_bytes(&[1, 0, 0, 0])?;
let mut bytes_from_enum = [0u8; UnitLikeEnum::SIZE];
let bytes_written = enum_from_bytes.to_be_bytes(&mut bytes_from_enum)?;

println!("UnitLikeEnum zeroed: {:#?}", zeroed_enum);
println!("UnitLikeEnum from bytes: {:#?}", enum_from_bytes);
println!("UnitLikeEnum wrote {} bytes to byte buffer: {:#?}", bytes_written, bytes_from_enum);
```

# NOTE
This project is still under heavy development. In the following versions I will be adding more features and documentation leading up to a `0.1.0` release.