/*  关于泛型的练习 */
struct A;           // 具体的类型 'A'.
struct S(A);        // 具体的类型 'S'.
struct SGen<T>(T);  // 泛型 `SGen`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>){}

fn gen_spec_i32(_s: SGen<i32>){}

fn generic<T>(_s: SGen<T>){}

fn exercise_1(){
    // 使用非泛型函数
    reg_fn(S(A));       // 具体的类型
    gen_spec_t(SGen(A));   // 隐式地指定类型参数 'A'
    gen_spec_i32(SGen(0i32)); // 隐式地指定类型参数 `i32`

    // 显式地指定类型参数 `char`
    generic::<char>(SGen('c'));

    // 隐式地指定类型参数 'char'.
    generic(SGen('c'));   
}

fn main(){
    exercise_1();
}