fn main() {
    let prime: usize = 1500;

    println!("nth({}) prime: {:?}", 
        prime, 
        find_prime(prime) );
} // end main


fn find_prime(nth: usize) -> Option<u32> {
    match (1..=10001).contains(&nth) {
        false => None,
        true  => _find_prime(nth - 1) }
} // end find_prime


fn _find_prime(index: usize) -> Option<u32> {
    (2..).filter(is_prime).nth(index)
} // end _find_prime


fn is_prime(number: &u32) -> bool {
    remainder_zero_divisions(number) == 2
} // end is_prime


fn remainder_zero_divisions(number: &u32) -> usize {
    let maximum_divisions = *number as usize;
    
    maximum_divisions - non_zero_remainders(number)
} // end remainder_zero_divisions


fn non_zero_remainders(number: &u32) -> usize {
    let remainder_not_zero = move |dividend: &u32| {
        (*number % dividend) != 0
    };
    (2.. *number).take_while(remainder_not_zero).count()
} // end non_zero_remainders
