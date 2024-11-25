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
#[mockimbap::mockable]
trait Foo {
    fn foo(&self) -> i32;
}

#[return_at(foo, 1)]
#[mock(Foo)]
struct MockFoo;
```

## ❗️ Limitation
Currently, the macro only supports mocking functions that return a value.
The macro does not support mocking functions that take arguments.

And **Always must mock after mockimbap::mockable**

So, you may do like this:
```rust
// mock.rs
#[mock(Foo)]
struct MockFoo;

// main.rs
#[mockimbap::mockable]
trait Foo {
    fn foo(&self) -> i32;
}

fn main() {
    let mock = MockFoo;
    assert_eq!(mock.foo(), 1);
}

// Like this, you must put mock file after mockimbap::mockable
mod mock;
```
