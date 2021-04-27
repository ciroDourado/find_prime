fn main() {
    println!("{:?}", find_prime(500));
} // end main 


fn find_prime(nth: usize) -> Option<u32> {
    match (1..=10001).contains(&nth) {
        false => None,
        true  => _find_prime(nth - 1)}
} // end find_prime


fn _find_prime(index: usize) -> Option<u32> {
    (2..).filter(is_prime).nth(index)
} // end _find_prime


fn is_prime(number: &u32) -> bool {
    let remainder_zero = move |dividend: &u32| { 
        (*number % dividend) == 0
    };
    (1..= *number).filter(remainder_zero).count() == 2
} // end is_prime
