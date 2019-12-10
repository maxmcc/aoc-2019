const COLS: usize = 25;
const ROWS: usize = 6;
const LAYER_SIZE: usize = COLS * ROWS;

fn main() {
    let input = include_str!("../../input/day08.in");
    let nums = input
        .trim()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();
    part1(&nums);
    part2(&nums);
}

fn part1(input: &[u32]) {
    let mut min_freqs = vec![std::usize::MAX; 3];
    'chunks: for chunk in input.chunks(LAYER_SIZE) {
        let mut freqs = vec![0; 3];
        for x in chunk {
            freqs[*x as usize] += 1;
            if freqs[0] >= min_freqs[0] {
                continue 'chunks;
            }
        }
        min_freqs = freqs;
    }
    println!("{}", min_freqs[1] * min_freqs[2]);
}

fn part2(input: &[u32]) {
    let mut image = vec![2; LAYER_SIZE];
    for chunk in input.chunks(LAYER_SIZE) {
        for (i, c) in chunk.iter().enumerate() {
            if image[i] == 2 {
                image[i] = *c;
            }
        }
    }
    for row in image.chunks(COLS) {
        let row = row
            .iter()
            .map(|c| match c {
                0 => ' ', // white
                1 => '*', // black
                _ => unreachable!(),
            })
            .collect::<String>();
        println!("{}", row);
    }
}
