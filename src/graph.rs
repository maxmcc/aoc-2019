//! A graph data structure, inspired by [Niko Matsakis's blog post][blog].
//!
//! [blog]: http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/

use std::collections::{HashSet, VecDeque};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Graph<T> {
    nodes: Vec<Node<T>>,
    edges: Vec<Edge>,
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph {
            nodes: vec![],
            edges: vec![],
        }
    }

    pub fn add_node(&mut self, value: T) -> NodeIndex {
        let index = NodeIndex(self.nodes.len());
        self.nodes.push(Node {
            value: value,
            first_outgoing_edge: None,
        });
        index
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) -> EdgeIndex {
        let edge_index = EdgeIndex(self.edges.len());
        let node_data = &mut self.nodes[source.0];
        self.edges.push(Edge {
            target: target,
            next_outgoing_edge: node_data.first_outgoing_edge,
        });
        node_data.first_outgoing_edge = Some(edge_index);
        edge_index
    }

    pub fn successors(&self, source: NodeIndex) -> Successors<T> {
        let first_outgoing_edge = self.nodes[source.0].first_outgoing_edge;
        Successors {
            graph: self,
            current_edge_index: first_outgoing_edge,
        }
    }

    pub fn bfs(&self, source: NodeIndex) -> Bfs<T> {
        let mut queue = VecDeque::new();
        queue.push_back((source, 0));
        Bfs {
            graph: self,
            queue: queue,
            visited: HashSet::new(),
        }
    }
}

impl<T: PartialEq> Graph<T> {
    pub fn find_node(&self, value: &T) -> Option<NodeIndex> {
        self.nodes.iter().enumerate().find_map(|(index, node)| {
            if node.value == *value {
                Some(NodeIndex(index))
            } else {
                None
            }
        })
    }
}

impl<T: PartialEq> std::iter::FromIterator<(T, T)> for Graph<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (T, T)>,
    {
        let mut graph = Graph::new();
        for (source, target) in iter {
            let source = graph.find_node(&source).unwrap_or(graph.add_node(source));
            let target = graph.find_node(&target).unwrap_or(graph.add_node(target));
            graph.add_edge(source, target);
            graph.add_edge(target, source);
        }
        graph
    }
}

impl<T> std::ops::Index<NodeIndex> for Graph<T> {
    type Output = T;

    fn index(&self, index: NodeIndex) -> &Self::Output {
        &self.nodes[index.0].value
    }
}

impl<T> std::ops::IndexMut<NodeIndex> for Graph<T> {
    fn index_mut(&mut self, index: NodeIndex) -> &mut Self::Output {
        &mut self.nodes[index.0].value
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct NodeIndex(usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Node<T> {
    value: T,
    first_outgoing_edge: Option<EdgeIndex>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct EdgeIndex(usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Edge {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
}

pub struct Successors<'g, T> {
    graph: &'g Graph<T>,
    current_edge_index: Option<EdgeIndex>,
}

impl<'g, T> Iterator for Successors<'g, T> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.graph.edges[edge_index.0];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge.target)
            }
        }
    }
}

pub struct Bfs<'g, T> {
    graph: &'g Graph<T>,
    visited: HashSet<NodeIndex>,
    queue: VecDeque<(NodeIndex, usize)>,
}

impl<'g, T> Iterator for Bfs<'g, T> {
    type Item = (NodeIndex, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.queue.pop_front() {
            None => None,
            Some((node, depth)) => {
                self.visited.insert(node);
                let visited = &self.visited;
                let next = self.graph.successors(node);
                self.queue.extend(next.filter_map(|next| {
                    if visited.contains(&next) {
                        None
                    } else {
                        Some((next, depth + 1))
                    }
                }));
                Some((node, depth))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_successors() {
        let mut graph = Graph::new();
        let zero = graph.add_node(0);
        let one = graph.add_node(1);

        assert_eq!(graph.successors(zero).count(), 0);
        assert_eq!(graph.successors(one).count(), 0);

        graph.add_edge(zero, one);
        assert_eq!(graph.successors(zero).collect::<Vec<_>>(), [one]);
        assert_eq!(graph.successors(one).count(), 0);

        graph.add_edge(one, zero);
        assert_eq!(graph.successors(one).collect::<Vec<_>>(), [zero]);

        let two = graph.add_node(2);
        graph.add_edge(zero, two);
        assert_eq!(
            graph.successors(zero).collect::<HashSet<_>>(),
            [one, two].iter().cloned().collect()
        );
    }

    #[test]
    fn test_bfs() {
        let mut graph = Graph::new();
        let (zero, one, two) = (graph.add_node(0), graph.add_node(1), graph.add_node(2));
        graph.add_edge(zero, one);
        graph.add_edge(one, zero);
        assert_eq!(graph.bfs(zero).collect::<Vec<_>>(), [(zero, 0), (one, 1)]);
        assert_eq!(graph.bfs(one).collect::<Vec<_>>(), [(one, 0), (zero, 1)]);

        graph.add_edge(one, two);
        assert_eq!(
            graph.bfs(zero).collect::<Vec<_>>(),
            [(zero, 0), (one, 1), (two, 2)]
        );

        assert_eq!(graph.bfs(two).collect::<Vec<_>>(), [(two, 0)]);
    }
}
