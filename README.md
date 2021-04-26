# find_prime

## Explaining this code:

find_prime is a function that takes the nth index to be calculated. <br>
Note that this nth does not follow the array convention of starting at 0: 
- it is first received as an ordinal number;
- checked if is between 1 and 10001;
- then passed to lower functions, "converted" to array index.

Then we have _find_prime (which in a private/public context shoudn't be called by a programmer): <br>
It does all the hardlifting, and what it's meant to do, which is calculate the desirable prime number.<br>
We know that primes start at 2, going to 3, 5, 7... and so on; so primes are some sort of set/interval, mathematically represented by <br>
**[2, +inf)**<br>
Thus, it must be represented on code as a range, which purpose fits correctly at this situation - it starts at a well-defined point,
and goes to the end, not inclusive (and if you dont specify, the compiler will treat this as an infinite loop, unless you give some 
function that will serve as a stop condition)<br>

And there it goes. To iterate over this infinite interval, looking only for primes, we must use a filter function. It will take another function
called is_prime to serve as predicate (personal opinion here: I think it's clearer to use functions instead of closures - it's better to read, 
to maintain, to keep track of the program flow; I only use closures when it's needed to capture some environment variable, which it's the obvious 
purpose of using them, it's their function!)<br>

[to be continued]

