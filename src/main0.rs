//!rust 基础

fn ma(){
// fn add_one(x:i32) -> i32{
//     x+1
// }

    // fn diverges() -> ! {
    //     panic!("This function never returns!");
    // }
    // diverges();

    let mut num = 5;
    // let plus_num = |x: i32| x+num;
    // plus_num(1);
    {
        let mut add_num = move  |x: i32| num +=x;
        add_num(5);
    }
    // assert_eq!(5,num);
    println!("{}",num);

    //高阶函数 high order function

    fn add_one(x: i32) -> i32 { x+1}

    fn apply<F>(f:F,y:i32) ->i32 where F:Fn(i32) ->i32
    {
        f(y)*y
    }


    let transform: fn(i32) -> i32 = add_one;
    let f0 = add_one(2i32)*2;
    let f1 = apply(add_one,2);
    let f2 = apply(transform,2);
    println!("{},{},{}",f0,f1,f2);
    let closure = |x: i32| x+1;
    let c0 = closure(2i32)*2;
    let c1 = apply(closure,2);
    let c2 = apply(|x| x+1,2);
    println!("{},{},{}",c0,c1,c2);

    fn factory(x: i32) -> Box<Fn(i32) -> i32> {
        Box::new(move |y| x + y)
    }

    let box_fn = factory(1i32);
    let b0 = box_fn(2i32)*2;
    let b1 = (*box_fn)(2i32)*2;
    let b2 = (&box_fn)(2i32)*2;
    println!("{}, {}, {}", b0, b1, b2);

    let add_num = &(*box_fn);
    let translate: &Fn(i32) -> i32 = add_num;

    /*
    方法
    impl struct enum trait method call syntax
    associated function self 
    self FnOnce 修改移动
    &self Fn 
    &mut self FnMut 修改
    static method 不含self参数的关联函数
    **/

    struct Circle{
        x:f64,
        y:f64,
        radius:f64,
    }

    impl Circle{
        fn new(x:  f64,y:f64,radius: f64) -> Circle {
            Circle{
                x:x,
                y:y,
                radius:radius,
            }
        }

        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    let c = Circle{x: 0.0,y: 0.0, radius: 2.0};
    println!("{}",c.area());
    println!("{}",Circle::new(0.0,0.0,2.0).area());

    /*
    特性与接口
    为了描述类型可以实现abstract interface,引入 tarit
    定义函数类型签名 function type signature
    **/

    trait HasArea {
        // add code here Fn
        fn area(&self) -> f64;
    }

    struct Circle1{
            x:f64,
            y:f64,
            radius:f64,
        }

    impl HasArea for Circle1 {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
        //rust是基于表达式的语言，有且只有两种语句。
        //let bing 语句和表达式语句
        //表达式语句通过在表达式末尾加上分号将表达式变成语句，丢弃该表达式的值，一律返回unit()
        //表达式如果返回，总返回一个值,但是语句不返回或返回()
        // fn area(&self) -> f64 {
        //     std::f64::consts::PI * (self.radius * self.radius);
        // }
    }

    struct Square {
        x: f64,
        y: f64,
        side: f64,
    }

    impl HasArea for Square {
        fn area(&self) -> f64 {
        self.side * self.side
        }
    }

    //特性约束
    fn print_area<T: HasArea>(shape: T){
        println!("{}",shape.area());
    }

    /*
    泛型（generics）参数多态(parametrice polymorphism)
    对给定参数可以有多种形式的函数或类型
    */
    //use generic parameters

    /*
    & 在 Rust 里一定是借用，没有别的意思。
    self 表示 move 掉自己
    &self 表示借用自己。
    */
    /*
    属于对象的函数，只是一个语法糖。
    看上去属于struct，实际上是独立的。
    所以如果把self看做一个普通参数，就很好理解了
    */
    /*
    &* value 表示 Deref and borrow  deref 然后借用
    Rust不区分左值右值。左右值只是类型系统不完善搞出来的鬼。不过如果出现在等于号右边表示 Deref
    */

    /*非rust
    等号左边的就是左值，是一个内存位置，右值是等号右边的值，是表达式
    */

    //use generic parameters
    // trait Graph<N,E> {
    //     fn has_edge(&self,&Self::N, &Self::N) -> bool;
    //     fn edges(&self,&Self::N) -> Vec<Self::E>;
    // }

    // fn distance<N,E,G: Graph<N,E>>(grpah: &G,start : &N, end: &N)
    //     -> u32 {
           
    //     }
    // use associated types
    trait Graph {
        type N;
        type E;
        fn has_edge(&self, &Self::N, &Self::N) -> bool;
        fn edges(&self, &Self::N) -> Vec<Self::E>;
        }

    
    
    struct Node;
    struct Edge;
    struct SimpleGraph;

    impl Graph for SimpleGraph{
        type N = Node;
        type E = Edge;

        fn has_edge(&self, n1: &Node, n2: &Node) -> bool {
            false
        }
        fn edges(&self, n: &Node) -> Vec<Edge> {
            vec![]
        }

    }
    let grpah = SimpleGraph;
    let object = Box::new(grpah) as Box<Graph<N=Node,E=Edge>>;
    

}