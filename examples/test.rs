#[derive(Debug)]
struct Foo {
    a: i32,
    b: i32,
    c: i32,
}

impl Default for Foo {
    fn default() -> Self {
        Self { a: 1, b: 2, c: 3 }
    }
}

fn main() {
    let foo = Foo {
        b: 10,
        ..Default::default()
    };

    println!("{:?}", foo);
}
