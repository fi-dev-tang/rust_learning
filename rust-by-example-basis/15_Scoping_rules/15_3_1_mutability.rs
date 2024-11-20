/*
[psNote]:
使用 &mut T 可以对原本借用的变量实行 read/write access,
&T 只能使用不可变引用，对原来的变量进行 read

这边加了这一行 #[derive(Clone, Copy)], 加上了 Copy trait, 进行了按值复制的机制
*/
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book{
    // `&'static str` is a reference to a string allocated in read only memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book){
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

fn new_edition(book: &mut Book){
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main(){
    let immutabook = Book{
        author: "Douglas Hofstadter",
        title: "Godel, Escher, Bach",
        year: 1979,
    };

    let mut mutabook = immutabook;
    borrow_book(&immutabook);

    borrow_book(&mutabook);

    new_edition(&mut mutabook);
}