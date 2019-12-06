use aoc::graph::Graph;

fn main() {
    let input = include_str!("../../input/day06.in");
    let orbits = input.lines().map(|line| {
        let mut bodies = line.split(')');
        (bodies.next().unwrap(), bodies.next().unwrap())
    });

    let graph = orbits.collect::<Graph<_>>();
    part1(&graph);
    part2(&graph);
}

fn part1(graph: &Graph<&str>) {
    let orbits = count_orbits(graph, &"COM");
    println!("{}", orbits);
}

fn part2(graph: &Graph<&str>) {
    let len = transfer_len(graph, &"YOU", &"SAN");
    println!("{}", len);
}

fn count_orbits<T: PartialEq>(graph: &Graph<T>, start: &T) -> usize {
    let start_index = graph.find_node(start).unwrap();
    graph.bfs(start_index).map(|(_, depth)| depth).sum()
}

fn transfer_len<T: PartialEq>(graph: &Graph<T>, start: &T, end: &T) -> usize {
    let start_index = graph.find_node(start).unwrap();
    let end_index = graph.find_node(end).unwrap();
    graph
        .bfs(start_index)
        .filter_map(|(node, depth)| {
            if node == end_index {
                Some(depth - 2)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_orbits_small() {
        let graph = [(0, 1), (0, 2)].iter().cloned().collect::<Graph<_>>();
        assert_eq!(count_orbits(&graph, &0), 2);

        let graph = [(0, 1), (1, 2)].iter().cloned().collect::<Graph<_>>();
        assert_eq!(count_orbits(&graph, &0), 3);
    }

    #[test]
    fn test_count_orbits_big() {
        let graph = [
            ("COM", "B"),
            ("B", "C"),
            ("C", "D"),
            ("D", "E"),
            ("E", "F"),
            ("B", "G"),
            ("G", "H"),
            ("D", "I"),
            ("E", "J"),
            ("J", "K"),
            ("K", "L"),
        ]
        .iter()
        .cloned()
        .collect::<Graph<_>>();
        assert_eq!(count_orbits(&graph, &"COM"), 42);
    }

    #[test]
    fn test_transfer_len_small() {
        let graph = [(0, 1), (1, 2), (2, 3)]
            .iter()
            .cloned()
            .collect::<Graph<_>>();
        assert_eq!(transfer_len(&graph, &0, &3), 1);
    }

    #[test]
    fn test_transfer_len_big() {
        let graph = [
            ("COM", "B"),
            ("B", "C"),
            ("C", "D"),
            ("D", "E"),
            ("E", "F"),
            ("B", "G"),
            ("G", "H"),
            ("D", "I"),
            ("E", "J"),
            ("J", "K"),
            ("K", "L"),
            ("K", "YOU"),
            ("I", "SAN"),
        ]
        .iter()
        .cloned()
        .collect::<Graph<_>>();
        assert_eq!(transfer_len(&graph, &"YOU", &"SAN"), 4);
    }
}
