<div align="center">
<h1> 🍙 Mockimbap </h1>
Mockimbap is a macro for mocking Rust functions

  [![Crates.io](https://img.shields.io/crates/v/mockimbap?color=green)](https://crates.io/crates/mockimbap)
  [![Rust Version](https://img.shields.io/badge/Language-Rust-orange)](https://www.rust-lang.org)
</div>

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
mockimbap = "0.1.0"
```

## Example

```rust
#[mockimbap::mockable(foo = 1)]
trait Foo {
    fn foo(&self) -> i32;
}

// Generates: struct MockFoo; impl Foo for MockFoo { ... }
```

## ❗️ Limitation
Currently, the macro only supports mocking functions that return a value.
The macro does not support mocking functions that take arguments.

Mock structs are generated next to the trait to avoid expansion-order issues.
You can optionally name the mock struct:
```rust
#[mockimbap::mockable(MyMock, foo = 1)]
trait Foo {
    fn foo(&self) -> i32;
}
```
