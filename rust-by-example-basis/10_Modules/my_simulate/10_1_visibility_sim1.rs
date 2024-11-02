mod my_mod{                             // public in crate
    pub fn public_in_my_mod(){          // public
        println!("Call by my_mod::public_in_my_mod()");
    }

    fn private_in_my_mod(){             // private
        println!("Call by my_mod::private_in_my_mod()");
    }

    pub fn indirect_access_in_my_mod(){ // public
        print!("Call by my_mod::indirect_access_in_my_mod(), that \n> ");
        private_in_my_mod();
    }

    pub mod nested{                                         // public in my_mod
        pub fn public_in_nested(){                          // public
            println!("Call by my_mod::nested::public_in_nested()");
        }

        pub(in crate::my_mod) fn public_crate_in_nested(){  // public in my_mod
            print!("Call by my_mod::nested::public_crate_in_nested(), that\n> ");
            private_self_in_nested();
        }

        pub(self) fn private_self_in_nested(){              // private = pub(self)   
            println!("Call by my_mod::nested::private_self_in_nested()");
        }

        pub(super) fn public_super_in_nested(){             // public (super -> father module) in my_mod
            print!("Call by my_mod::nested::public_super_in_nested(), that\n> ");
            private_in_nested();
        }

        fn private_in_nested(){                            // private
            println!("Call by my_mod::nested::private_in_nested()");
        }
    }

    pub fn call_by_nested_function(){                      // public
        print!("Call by my_mod::call_by_nested_function(), that\n> ");
        nested::public_crate_in_nested();
        print!("> ");
        nested::public_super_in_nested();
    }

    pub(crate) fn public_crate_in_my_mod(){              // public
        println!("Call by my_mod::public_crate_in_my_mod()");
    }

    mod private_mod{                                    // private
        #[allow(dead_code)]
        fn private_in_private_mod(){
            println!("Call by my_mod::private_mod::private_in_private_mod()");
        }

        #[allow(dead_code)]
        pub(crate) fn still_private_crate_in_private_mod(){ // private(because module private_mod is private)
            println!("Call by my_mod::private_mod::still_private_crate_in_private_mod()");
        }
    }
}

fn function(){
    println!("Call by function()");
}

fn main(){
    function();
    my_mod::public_in_my_mod();             // 1. pub fn 
    my_mod::indirect_access_in_my_mod();    // 2. pub fn -> [private]
    my_mod::call_by_nested_function();      // 3. pub fn -> [pub mod nested] -> [pub(super) / pub(in crate::my_mod)]
    my_mod::public_crate_in_my_mod();       // 4. pub(crate)

    my_mod::nested::public_in_nested();     // 5. pub mod -> pub fn
}