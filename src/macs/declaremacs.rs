macro_rules! say_hello {
    // 可以使用多个模式来匹配, 适配不同的情况
    () => {
        println!("Hello");
    };
    // 可以捕获变量, 可以限定捕获的变量的类型
    // 常见的类型有 
    // expr,           // 表达式
    // ident,          // 标识符
    // ty(type),       // 类型
    // tt(token tree), // token 树
    // path,           // 路径
    // literal         // 字面
    ($name: expr) => {
        println!("Hello {}", $name);
    };
}

//
// 定制一个可以获取变量名的宏
//
macro_rules! nameof {
    // ident 代表一个标识符
    ($name: ident) => {
        stringify!($name)
    }
}

// 
// 创建函数的宏
// 
macro_rules! create_fn { 
    ($name: ident, $body: expr) => {
        fn $name() -> u32 {
            $body
        }
    }
}

// 
// 创建结构体 (代码在编译期会被展开)
//
macro_rules! create_struct{  
    ($name: ident, $($field_name: ident: $field_type: ty),*) => {
        #[derive(Debug)]
        struct $name {
            // * 表示模式重复多次
            $($field_name: $field_type),*
        }
        impl $name {
            fn new($($field_name: $field_type),*) -> Self {
                Self {
                    $($field_name),*
                }
            }
        }
    };
}


//
// 用来模仿 vec![] 初始化一个 HashMap
//
macro_rules! hashmap {
    ($($key:expr => $value:expr),*$(,)*) => {
        // 使用一个作用域, 防止变量被遮蔽
        {
            use std::collections::HashMap;
            let mut map = HashMap::new();
            $(map.insert($key, $value);)*
            map
        }
    }
}


struct Person {}


pub fn macs_demo() {

    say_hello!();
    say_hello!("venus");

    // 动态打印变量名
    println!("struct name is {}", nameof!(Person));

    create_fn!(get_one, 1);
    create_fn!(get_ten, 10);
    println!("get_one: {}, get_ten: {}", get_one(), get_ten());

    // 创建结构体
    create_struct!(Student, name: String, age: u8);
    println!("{:#?}", Student::new(String::from("venus"), 18));

    // 创建 HashMap
    let map = hashmap!(
        1 => "one",
        2 => "two",
        3 => "three",
    );
    println!("{:#?}", map);

}
