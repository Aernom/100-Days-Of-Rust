fn main() {
    for bbq in BBQS {
        println!("{:?}", skewer_types(&bbq));
    }
}

fn skewer_types(bbq: &[&str]) -> (u32, u32) {
    bbq.iter().fold((0, 0), |(veg, meat), v| match v.find('x') {
        Some(_) => (veg, meat + 1),
        None => (veg + 1, meat),
    })
}

const BBQS: [[&str; 5]; 3] = [
    [
        "--xo--x--ox--",
        "--xx--x--xx--",
        "--oo--o--oo--",
        "--xx--x--ox--",
        "--xx--x--ox--",
    ],
    [
        "--oooo-ooo--",
        "--xx--x--xx--",
        "--o---o--oo--",
        "--xx--x--ox--",
        "--xx--x--ox--",
    ],
    [
        "--oooo-ooo--",
        "--xxxxxxxx--",
        "--o---",
        "-o-----o---x--",
        "--o---o-----",
    ],
];
