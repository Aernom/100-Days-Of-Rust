fn main() {
    assert_eq!(next_prime(12), 13);
    assert_eq!(next_prime(24), 29);
    assert_eq!(next_prime(0), 2);
    assert_eq!(next_prime(11), 11);
}

fn next_prime(number: u32) -> u32 {
    let mut current = number;

    while !is_prime(current) {
        current += 1;
    }

    current
}

fn is_prime(number: u32) -> bool {
    if number <= 1 {
        return false;
    }

    for i in 2..number {
        if number % i == 0 {
            return false;
        }
    }

    true
}
