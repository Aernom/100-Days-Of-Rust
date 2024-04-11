fn main() {
    PHRASES
        .map(|p| find_nemo(p))
        .iter()
        .for_each(|r| println!("{:?}", r));
}

fn find_nemo(text: &str) -> Option<usize> {
    for (i, slice) in text.split_whitespace().enumerate() {
        if slice == "Nemo" {
            return Some(i + 1);
        }
    }

    None
}

const PHRASES: [&str; 4] = [
    "I am finding Nemo !",
    "Nemo is me",
    "I Nemo am  ",
    "My name is Ea. Nice to meet you.",
];

/*
```text
findNemo("I am finding Nemo !") ➞ "I found Nemo at 4!"

findNemo("Nemo is me") ➞ "I found Nemo at 1!"

findNemo("I Nemo am") ➞ "I found Nemo at 2!"
```
*/
