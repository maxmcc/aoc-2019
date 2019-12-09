use std::collections::HashMap;

fn main() {
    main1();
    main2();
}

fn main1() {
    let input = include_str!("../../input/day08.in");
    let layer_size = 25 * 6;
    let chars = input.trim().chars().collect::<Vec<_>>();
    let layer = chars
        .chunks(layer_size)
        .map(|chunk| {
            let mut map = HashMap::new();
            for x in chunk {
                *map.entry(x).or_insert(0) += 1;
            }
            map
        })
        .min_by_key(|freq| freq[&'0'])
        .unwrap();
    println!("{}", layer[&'1'] * layer[&'2']);
}

fn main2() {
    let input = include_str!("../../input/day08.in");
    let rows = 6;
    let cols = 25;
    let layer_size = rows * cols; //25 * 6;
    let mut image = vec!['2'; layer_size];
    let chars = input.trim().chars().collect::<Vec<_>>();
    for chunk in chars.chunks(layer_size) {
        for (i, c) in (0..).zip(chunk) {
            if image[i] == '2' {
                image[i] = *c;
            }
        }
    }
    let s = image
        .chunks(cols)
        .map(|s| {
            s.iter()
                .map(|c| match c {
                    '1' => ' ',
                    '0' => '*',
                    _ => *c,
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", s);
}
