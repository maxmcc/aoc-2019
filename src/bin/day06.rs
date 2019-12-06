use std::borrow::Borrow;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::iter::FromIterator;
use std::rc::Rc;

fn main() {
    let input = include_str!("../../input/day06.in");
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let graph = input
        .lines()
        .map(|line| {
            let mut bodies = line.split(')');
            (bodies.next().unwrap(), bodies.next().unwrap())
        })
        .collect::<Graph<_>>();
    println!("{}", graph.count_connections(&"COM"));
}

fn part2(input: &str) {
    let graph = input
        .lines()
        .map(|line| {
            let mut bodies = line.split(')');
            (bodies.next().unwrap(), bodies.next().unwrap())
        })
        .collect::<Graph<_>>();
    println!("{}", graph.distance(&"YOU", &"SAN"));
}

#[derive(Debug)]
struct Graph<T: Hash + Eq> {
    nodes: HashMap<Rc<T>, HashSet<Rc<T>>>,
}

impl<T: Hash + Eq> Graph<T> {
    fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    fn add_edge(&mut self, (a, b): (T, T)) {
        let a = Rc::new(a);
        let b = Rc::new(b);
        self.nodes
            .entry(a.clone())
            .or_insert(HashSet::new())
            .insert(b.clone());
        self.nodes
            .entry(b.clone())
            .or_insert(HashSet::new())
            .insert(a.clone());
    }

    fn count_connections(&self, start: &T) -> usize {
        self.bfs(&start).map(|(_, depth)| depth).sum()
    }

    fn distance(&self, start: &T, end: &T) -> usize {
        self.bfs(&start)
            .filter_map(|(node, depth)| if node == end { Some(depth - 2) } else { None })
            .next()
            .unwrap()
    }

    fn bfs<'short, 'long: 'short>(&'long self, start: &'long T) -> Bfs<'short, T> {
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));
        Bfs {
            graph: self,
            queue: queue,
            seen: HashSet::new(),
        }
    }
}

impl<T: Hash + Eq> FromIterator<(T, T)> for Graph<T> {
    fn from_iter<I: IntoIterator<Item = (T, T)>>(iter: I) -> Self {
        let mut graph = Graph::new();
        for item in iter {
            graph.add_edge(item);
        }
        graph
    }
}

struct Bfs<'a, T: Hash + Eq> {
    graph: &'a Graph<T>,
    queue: VecDeque<(&'a T, usize)>,
    seen: HashSet<&'a T>,
}

impl<'a, T: Hash + Eq> Iterator for Bfs<'a, T> {
    type Item = (&'a T, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((node, depth)) = self.queue.pop_front() {
            self.seen.insert(node);
            let downstream = self.graph.nodes.get(node).into_iter().flatten();
            let seen = &self.seen;
            let unvisited = downstream.filter_map(|new_node| {
                let b: &T = new_node.borrow();
                if seen.contains(b) {
                    None
                } else {
                    Some((new_node.borrow(), depth + 1))
                }
            });
            self.queue.extend(unvisited);
            Some((node, depth))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_connections() {
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
        assert_eq!(graph.count_connections(&"COM"), 42);
    }

    #[test]
    fn test_distance() {
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
        assert_eq!(graph.distance(&"YOU", &"SAN"), 4);
    }
}
