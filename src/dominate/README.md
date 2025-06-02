# Chapter 6: Dominate

Dominate is a library for building DOM trees in Rust.
(This library also uses StyleMan for styling, TailwindCSS -> CSS.)

## Usage

```rust
use dominate::prelude::*;

fn main() {
    let mut doc = html! {
        <html>
            <head>
                <title>"Dominate"</title>
            </head>
            <body>
                <h1>{1 + 1}</h1>
                <p>{"A library for building DOM trees in Rust."}</p>
            </body>
        </html>
    };

    println!("{:?}", doc);
}
```
