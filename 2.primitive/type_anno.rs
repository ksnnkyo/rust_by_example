fn main(){
    // 前置类型说明
    let logical: bool = true;
    let a_float: f64 = 1.0;
    
    // 后置类型说明
    let an_integer = 5i32;
    
    // 自动类型推断
    let default_integer = 7;

    // 根据上下文推断
    let mut inferred_type = 12;
    inferred_type = 4294967296i64;

    // 可变的变量，值可以改变
    let mut mutable = 12;
    mutable = 21;
    // 注意，类型不能改变
    mutable = true;
    // 但是可以遮蔽
    let mutable = true;
}