fn main() {}

fn trapped_water(section: &[u32]) -> u32 {
    let start = *section.first().unwrap_or(&0);
    let end = *section.last().unwrap_or(&0);

    if start == 0 || end == 0 {
        return 0;
    }

    let peak = start.min(end);
    let max_area = peak * (section.len() as u32 - 1);
}

const EL_1: [u32; 12] = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
const EL_2: [u32; 12] = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
