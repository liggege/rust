use std::error::Error;
use std::io::{self,Write};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

///标准输入输出和文件输入输出
fn read_input() -> io::Result<()>{
    let mut input = String::new();

    try!(io::stdin().read_line(&mut input));
    println!("You typed: {}",input.trim());
    Ok(())
}


fn stdfn() {
    // read_input(); 
    //标准输出也叫控制台输出，标准化的输出是行缓冲的，这就导致标准化
    // 的输出在遇到一个新行之前并不会被隐式刷新

   print!("请输入一个字符串：");
   io::stdout().flush().unwrap();
   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("读取失败");
   print!("您输入的字符串是：{}\n",input);
}

fn fileIn(){
    //文件输入输出
    //创建一个文件路径
    let path = Path::new("hello.txt");
    let display = path.display();

    //打开文件只读模式，返回一个`io::Result<File>类型`；
    
    let mut file = match File::open(&path){
        //处理打开文件可能潜在的错误
        Err(why) => panic!("coulddn`t open{}:{}",display,Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();

    match file.read_to_string(&mut s) {

        Err(why) => panic!("couldn`t read{}:{}",display,Error::description(&why)),
        Ok(_) => print!("{} contains:\n{}", display, s),

    }
}

static LOREM_IPSUM: &'static str = "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed d
o eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad m
inim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate v
elit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est
laborum.
";

fn fileOut(){

    let path = Path::new("out/lorem_ipsum.txt");
    let display = path.display();

    //用只写模式打开一个文件，并返回`io::Result<File>` 类型

    let mut file = match File::create(&path){
        Err(why) => panic!("couldn`t create {}:{}",
            display,Error::description(&why)),
        Ok(file) => file,
    };

    //写入字符串到文件中，并返回`io::Result<()>类型`
    
    match file.write_all(LOREM_IPSUM.as_bytes()) {

        Err(why) => {
            panic!("couldn`t write to {}:{}",display,Error::description(&why))
        },
        Ok(_) => println!("successfully wrote to {}",display),

    }
}