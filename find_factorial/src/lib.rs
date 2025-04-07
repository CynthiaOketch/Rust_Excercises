pub fn factorial(num: u64) -> u64 {
    let mut fact: u64 = 1;

    for i in 2..=num {
        fact *= i;
    }

    fact
}
