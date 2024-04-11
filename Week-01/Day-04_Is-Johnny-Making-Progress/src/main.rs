fn main() {
    assert_eq!(progress_days(&[3, 4, 1, 2]), 2);
    assert_eq!(progress_days(&[10, 11, 12, 9, 10]), 3);
    assert_eq!(progress_days(&[6, 5, 4, 3, 2, 9]), 1);
    assert_eq!(progress_days(&[9, 9]), 0);
    assert_eq!(progress_days(&[9]), 0);
    println!("Test passed");
}

fn progress_days(days: &[u32]) -> u32 {
    let mut prog = 0;

    for i in 1..days.len() {
        if days[i] > days[i - 1] {
            prog += 1;
        }
    }

    prog
}
