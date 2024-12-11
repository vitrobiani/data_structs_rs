use ordered_float::OrderedFloat;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Edge {
    node_id: i32,
    weight: OrderedFloat<f64>,
    min_flow: OrderedFloat<f64>,
    max_flow: OrderedFloat<f64>,
}

pub struct GraphNode<T> {
    node: (T, HashSet<Edge>),
    outdegree: i32,
    indegree: i32,
}

pub struct Graph<T> {
    nodes: HashMap<i32, GraphNode<T>>,
    count: i32,
}

impl Edge {
    pub fn new(node_id_v: i32, weight_v: f64, min_flow_v: f64, max_flow_v: f64) -> Self {
        Edge {
            node_id: node_id_v,
            weight: OrderedFloat(weight_v),
            min_flow: OrderedFloat(min_flow_v),
            max_flow: OrderedFloat(max_flow_v),
        }
    }
}

impl<T: Eq + Hash> GraphNode<T> {
    pub fn new(data: T) -> Self {
        GraphNode {
            node: (data, HashSet::new()),
            outdegree: 0,
            indegree: 0,
        }
    }
}

impl<T: Eq + Hash> Graph<T> {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            count: 0,
        }
    }

    pub fn add_node(&mut self, node: T) {
        self.count += 1;
        self.nodes.insert(self.count, GraphNode::new(node));
    }

    pub fn add_vertice(&mut self, from: i32, to: i32, weight: f64, min_flow: f64, max_flow: f64) {
        if self.nodes.contains_key(&from) && self.nodes.contains_key(&to) {
            let from_node = self.nodes.get_mut(&from).unwrap();
            from_node
                .node
                .1
                .insert(Edge::new(to, weight, min_flow, max_flow));
            from_node.outdegree += 1;

            let to_node = self.nodes.get_mut(&to).unwrap();
            to_node.indegree += 1;
        } else {
            print!("both nodes must exist!");
        }
    }

    pub fn print(&self, print_func: fn(&T)) {
        for node in self.nodes.iter() {
            let tup = &node.1.node;
            print!("# id: {},   data: ", node.0);
            print_func(&tup.0);
            print!(
                "\nthe neigbours: <amount: {}, out: {}, in: {}> \n",
                tup.1.len(),
                node.1.outdegree,
                node.1.indegree
            );
            for n in tup.1.iter() {
                print!(
                    "\t- id: {} | weight: {} | min_flow: {} | max_flow: {} | data: ",
                    n.node_id, n.weight, n.min_flow, n.max_flow
                );
                print_func(&self.nodes.get_key_value(&n.node_id).unwrap().1.node.0);
                print!("\n");
            }
        }
    }
}
