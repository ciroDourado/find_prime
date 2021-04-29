// this implementation takes up to aproximatelly
// 3 seconds to calculate the 10001st prime

fn main() {
    let prime: usize = 10001;

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
    let (mut counter, mut dividend) = (0, 2);
    
    while remainder_not_zero(number, dividend) {
        dividend += 1;
        counter  += 1;
    }
    counter as usize
} // end non_zero_remainders


fn remainder_not_zero(num: &u32, div: u32) -> bool {
    (*num % div) != 0
} // end remainder_not_zero
