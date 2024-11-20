/*
[psNote]:
当数据被不可变地借用时，它也会被冻结。
冻结的数据在所有对其的引用超出作用域之前，不能通过原对象进行修改

认为是编译器做了优化，实际上应该会在 13 行报错，因为 12 给了一个不可变借用，暂时冻结了对数据的修改
*/
fn main(){
    let mut _mutabe_integer = 7i32;

    {
        let large_integer = &_mutabe_integer;
        _mutabe_integer = 50;
    }
    _mutabe_integer = 3;
}