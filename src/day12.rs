use aoc_runner_derive::{aoc, aoc_generator};
use petgraph::{graph::{self, UnGraph}, Graph, Undirected};
use petgraph::visit::Walker;
use petgraph::algo::connected_components;

#[aoc_generator(day12)]
fn input_generator(input: &str) -> Graph<String, (), Undirected> {
    let mut graph = UnGraph::<String, ()>::new_undirected();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" <-> ").collect();
        let node = parts[0].to_string();
        let neighbors: Vec<&str> = parts[1].split(", ").collect();

        let root_node = graph
            .node_indices()
            .find(|n| graph[*n] == node)
            .unwrap_or_else(|| graph.add_node(node.clone()));

        for neighbor in neighbors {
            let neighbor_str = neighbor.to_string();
            let neighbor_node = graph
                .node_indices()
                .find(|n| graph[*n] == neighbor_str)
                .unwrap_or_else(|| graph.add_node(neighbor_str.clone()));

            graph.add_edge(root_node, neighbor_node, ());
        }
    }
    graph
}


#[aoc(day12, part1)]
fn solve_part1(graph: &Graph<String, (), Undirected>) -> usize {
    let zero_index = graph.node_indices().find(|i| graph[*i] == "0").unwrap();
    let component_size = petgraph::visit::Dfs::new(&graph, zero_index).iter(&graph).count();
    component_size
}

#[aoc(day12, part2)]
fn solve_part2(graph: &Graph<String, (), Undirected>) -> usize {
    let zero_index = graph.node_indices().find(|i| graph[*i] == "0").unwrap();
    let component_size = petgraph::visit::Dfs::new(&graph, zero_index).iter(&graph).count();
    connected_components(&graph)
}
