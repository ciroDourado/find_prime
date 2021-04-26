fn main() {
    println!("{:?}", find_prime(500));
} 


fn find_prime(nth: usize) -> Option<u32> {
    match nth < 1 || nth > 10001 {
        true  => None,
        false => _find_prime(nth - 1)}
}


fn _find_prime(index: usize) -> Option<u32> {
    (2..).filter(is_prime).nth(index)
}


fn is_prime(number: &u32) -> bool {
    let remainder_zero = move |dividend: &u32| { 
        (*number % dividend) == 0
    };
    (1..= *number).filter(remainder_zero).count() == 2
}
