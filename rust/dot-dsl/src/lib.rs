#[macro_use] extern crate maplit;

pub mod graph {
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: String,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: String::from(""),
            }
        }

        pub fn with_nodes(mut self, nodes: &Vec<graph_items::node::Node>) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        
    }

    pub mod graph_items {

        pub mod edge {
            pub struct Edge;
        }

        pub mod node {
            use std::cmp;
            use std::fmt;


            #[derive(Clone)]
            pub struct Node {
                name: String,
            }

            impl Node {
                pub fn new(name: &str) -> Node {
                    Node {
                        name: String::from(name)
                    }
                }
            }

            impl cmp::PartialEq for Node {
                fn eq(&self, node: &Node) -> bool {
                    if self.name == node.name {
                        return true
                    }
                    false
                }
            }

            impl fmt::Debug for Node {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "Name: {}", self.name)
                }
            }

        }

    }

}
