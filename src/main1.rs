use std::fmt::Debug
fn foo<T: Clone,K: Clone + Debug>(x: T,y: K){
    x.clone();
    y.clone();{{
    println!("{:?}",y);
}

fn bar<T,K>(x: T,y: K)
    where T: Clone,K: Clone + Debug
{
    x.clone();
    y.clone();{{
    println!("{:?}",y);
}

trait Foo {
    fn foo(&self);

    //defalut method
    fn bar(&self) { println!("We called bar.");}
}

//inheritance
trait  FooBar : Foo {
    // add code here
    fn foobar(&self);
}

struct Baz;

impl Foo for Baz {
    // add code here
    fn foo(&self){println!("foo");}
}
impl FooBar for Baz{
    fn foobar(&self){println!("foobar");}
}

