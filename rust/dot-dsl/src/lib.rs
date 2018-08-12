#[macro_use] extern crate maplit;

pub mod graph {
    use std::collections;

    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: collections::HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: hashmap!{},
            }
        }

        pub fn with_nodes(mut self, nodes: &Vec<graph_items::node::Node>) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, edges: &Vec<graph_items::edge::Edge>) -> Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for attr_pair in attrs.iter() {
                let (key, value) = attr_pair;
                self.attrs.insert(key.to_string(), value.to_string());
            }
            
            self
        }

        
    }

    pub mod graph_items {

        pub mod edge {
            use std::collections;

            #[derive(Clone)]
            #[derive(PartialEq)]
            #[derive(Debug)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: collections::HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Edge {
                    Edge {
                        from: String::from(from),
                        to: String::from(to),
                        attrs: hashmap!{},
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for attr_pair in attrs.iter() {
                        let (key, value) = attr_pair;
                        self.attrs.insert(key.to_string(), value.to_string());
                    }
            
                    self
                }

            }

        }

        pub mod node {
            use std::collections;

            #[derive(Clone)]
            #[derive(PartialEq)]
            #[derive(Debug)]
            pub struct Node {
                name: String,
                attrs: collections::HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Node {
                    Node {
                        name: String::from(name),
                        attrs: hashmap!{},
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for attr_pair in attrs.iter() {
                        let (key, value) = attr_pair;
                        self.attrs.insert(key.to_string(), value.to_string());
                    }
            
                    self
                }
            }

        }

    }

}
