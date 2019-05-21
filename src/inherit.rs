trait Foo {
    fn foo(&self);
}

trait FooBar : Foo {
    fn foo_bar(&self);
}

struct Baz;

impl FooBar for Baz {
    fn foo_bar(&self) {
        println!("call baz.foo_bar()");
    }
}

impl Foo for Baz {
    fn foo(&self) {
        println!("call baz.foo()");
    }
}

fn main() {
    let baz = Baz;

    baz.foo();
    baz.foo_bar();
}
