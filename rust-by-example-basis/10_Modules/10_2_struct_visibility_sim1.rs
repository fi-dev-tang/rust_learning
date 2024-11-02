mod my{
    pub struct OpenBox<T>{
        pub contents: T,
    }

    #[allow(dead_code)]
    pub struct ClosedBox<T>{
        contents: T,
    }

    impl<T> ClosedBox<T>{
        pub fn new(contents: T) -> ClosedBox<T>{
            ClosedBox{
                contents:contents
            }
        }
    }
}

fn main(){
    let openbox = my::OpenBox{contents: "Public struct with public member:contents!"};
    println!("{}", openbox.contents);

    let _closed_box = my::ClosedBox::new("Public struct with private member: contents!");
}