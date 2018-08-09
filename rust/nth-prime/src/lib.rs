pub fn nth(n: u32) -> Option<u32> {
    if n == 0 {
        return None
    }

    let mut primes: Vec<u32> = Vec::new();
    primes.push(2);
    primes.push(3);
    primes.push(5);
    primes.push(7);
    primes.push(11);
    primes.push(13);

    let mut counter = 14;

    while primes.len() <= n as usize {
        if counter % 2 != 0 {
            let mut temp: u32 = counter / 2;
            
            while temp >= 3 {
                if counter % temp == 0 {
                    break
                }
                temp -= 1
            }

            if temp <= 3 {
                primes.push(counter)
            }
        }

        counter += 1;
    }

    Option::from(primes[(n - 1) as usize])

}
