use std::collections::HashMap;

fn main() {
    assert_eq!(sock_pairs("     "), 0);
    assert_eq!(sock_pairs("AA"), 1);
    assert_eq!(sock_pairs("ABABC"), 2);
    assert_eq!(sock_pairs("CABBACCCD"), 4);
}

fn sock_pairs(socks: &str) -> u32 {
    let mut socks_map: HashMap<char, u32> = HashMap::new();

    for sock in socks.trim().chars() {
        socks_map
            .entry(sock)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    socks_map
        .values()
        .fold(0, |acc, socks_of_type| acc + socks_of_type / 2)
}
