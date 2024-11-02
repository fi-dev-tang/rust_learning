/*
The use declaration can be used to bind a full path to a new name, for easier access.
It is ofen used like this
*/
use crate::deeply::nested::function as other_function;

mod deeply{
    pub mod nested{
        pub fn function(){
            println!("Called by deeply::nested::function()");
        }
    }
}

fn function(){
    println!("Called by function()");
}

fn main(){
    other_function();

    println!("Entering block"); // shadowing
    {
        use crate::deeply::nested::function as function;  // Equivalent to `use deeply::nested::function;`
        function();
    }
    println!("Leaving block");
    function(); 
}