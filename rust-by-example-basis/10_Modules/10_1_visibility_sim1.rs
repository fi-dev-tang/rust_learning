/* 10_1_visibility 的模拟，精简并总结所有的情况 */
mod my_mod{
    fn private_function(){
        println!("called `my_mod::private_function()`");
    }

    pub fn function(){
        println!("called `my_mod::function()`");
    }

    pub fn indirect_access(){
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    pub mod nested{
        pub fn function(){
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function(){
            println!("called `my_mod::nested::function()`");
        }

        pub(in crate::my_mod) fn public_function_in_my_mod(){
            print!("called `my_mod::nested::public_function_in_my_mod()`, that \n> ");
            public_function_in_nested();
        }

        pub(self) fn public_function_in_nested(){ // pub(self) == private
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

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

    pub(crate) fn public_function_in_crate(){
        println!("called `my_mod::public_function_in_crate()`");
    }

    mod private_nested{
        #[allow(dead_code)]
        pub fn function(){
            println!("called `my_mod::private_nested::function()`");
        }

        #[allow(dead_code)]
        pub(crate) fn restricted_function(){  // mod private_nested is private, so it won't be allowed in main.rs's main(), even declared pub(crate)
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

    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    my_mod::public_function_in_crate();
}