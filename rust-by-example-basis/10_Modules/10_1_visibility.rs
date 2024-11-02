/*
写在最前: 在写 rustlings 的 41 题之后关于 Modules 的部分卡壳，所以先补一下这部分的知识。
参考的内容是 rust-by-example 的 Modules

目前对于 Modules 的理解，从 C 的经验来看，C这部分的做法是 专门写一个头文件 head.h, 然后把所有定义的结构体，函数都写在这个 head.h 里面
之后使用到的 .c 文件，引用 #include"head.h"
rust 里的做法或许类同，但更灵活?

Modules （RBE) 章节提要:
Rust provides a powerful module system that can be used to hierarchically split code in logical units(modules),
and manage visibility(public/private) between them.
A module is a collection of items: functions, structs, trait, impl blocks, and other modules.

1. 发现了第一个类同 Java 的点:
类似 protected，某些功能设置成 public, 但不是对所有外部的模块都 public, 只能对 parent 或者 ancestor module 可见
语法 
pub(in path)
类似这里的 
mod my_mod{
    mod nested{
        pub(in crate::my_mod) fn public_function_in_my_mod(){
        }
    }
} 
*/

/*
[visibility]
default: private visibility, overriden with the pub modifier.
Only the public items of a module can be accessed from outside the module scope.
*/
mod my_mod {  // 这里不需要加 pub 的原因，和 10_1_visibility.rs 的 main.rs 中定义了
    fn private_function(){
        println!("called `my_mod::private_function()`");
    }

    pub fn function(){
        println!("called `my_mod::function()`");
    }

    // 类似，在同一个模块内，item 可以访问另一个 item
    pub fn indirect_access(){
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // Modules can also be nested, 模块可以嵌套
    // 看到这里感觉跟 class 非常像
    pub mod nested{
        pub fn function(){
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function(){
            println!("called `my_mod::nested::private_function()`");
        }

        /*
        pub(in crate::my_mod) fn public_function_in_my_mod(){}
        使用 pub(in path) 语法声明的函数，pub(in path)语法允许你指定函数的可见性范围。crate::my_mod 表示该函数只在 my_mod 模块内可见

        顶层模块 crate, 其中包含一个子模块 my_mod, my_mod 又包含一个嵌套模块 nested。
        在这种结构下， public_function_in_my_mod 只能在 my_mod 模块及其子模块中被访问
        如果尝试从 crate 或其他外部模块直接调用 public_function_in_my_mod, 编译会报错，因为该函数的可见性被限制在了 my_mod 内
        */
        /*
        Functions declared using `pub(in path)` syntax are only visible within the given path.
        `path` must be a parent or ancestor module
        */
        pub(in crate::my_mod) fn public_function_in_my_mod(){
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }

        /*
        Functions declared using `pub(self)` syntax are only visible within
        the current module, which is the same as leaving them private
        */
        pub(self) fn public_function_in_nested(){
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        /*
        Functions declared using `pub(super)` syntax are only visible with the parent module
        */
        pub(super) fn public_function_in_super_mod(){
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod(){
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate) makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate(){
        println!("called `my_mod::public_function_in_crate()`");
    }

    // Nested modules follow the same rules for visibility
    mod private_nested{
        #[allow(dead_code)]
        pub fn function(){
            println!("called `my_mod::private_nested::function()`");
        }

        /*
        Private parent items will still restrict the visibility of a child item,
        even if it is declared as visible within a bigger scope.
        
        即使在子模块 private_nested 中，把函数声明为 pub(crate) fn restricted_function 看似对于所有的模块都可见
        但由于这个子模块属于 mod my_mod{} ,受到父模块的限制
        */
        #[allow(dead_code)]
        pub(crate) fn restricted_function(){
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn function(){
    println!("called `function()`");
}

fn main(){
    function();

    my_mod::function();

    /*
    第一部分，声明为 public 的 item, 包括那些嵌套的模块，可以从外部通过父模块访问
    */
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) items can be called from anywhere in the same crate
    my_mod::public_function_in_crate();    

    // my_mod::nested::public_function_in_my_mod();  // Error1: main 在 crate 的顶层，而不是在 my_mod 模块内部，可以修改成 pub（crate)
                                                     // 从而可以在 main 中调用
}