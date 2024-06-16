use num::complex::Complex;

fn main(){
    let a = Complex {re: 3.1, im: 4.2};
    let b = Complex::new(11.1, 2.2);
    let result = a + b;

    println!("result = {} + {}i", result.re, result.im);
}