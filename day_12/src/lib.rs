use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub name: String,
    max_visits: u32,
}

#[derive(Debug, Clone)]
pub struct Graph {
    nodes: Vec<Rc<Node>>,
    edges: Vec<(Rc<Node>, Rc<Node>)>,
}

impl Graph {
    pub fn from_edges(edges: Vec<(String, String)>) -> Graph {
        let mut graph: Graph = Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        };

        for (left, right) in &edges {
            if graph.get_node_by_name(&left).is_none() {
                graph.nodes.push(Rc::new(Node {
                    max_visits: calc_max_visits(&left),
                    name: left.clone(),
                }));
            }
            if graph.get_node_by_name(&right).is_none() {
                graph.nodes.push(Rc::new(Node {
                    max_visits: calc_max_visits(&right),
                    name: right.clone(),
                }));
            }

            let left = graph.get_node_by_name(&left).unwrap().clone();
            let right = graph.get_node_by_name(&right).unwrap().clone();
            graph.edges.push((left.clone(), right.clone()));
            graph.edges.push((right, left));
        }

        graph
    }

    pub fn find_all_paths_part_1(&mut self) -> Vec<Vec<Rc<Node>>> {
        let starting_node: &Rc<Node> = self.get_node_by_name("start").unwrap();
        let mut visits = vec![Rc::clone(starting_node)];
        let mut path = vec![];
        self.depth_first_traversal(&mut visits, &mut path, "");
        path
    }

    pub fn find_all_paths_part_2(&mut self) -> Vec<String> {
        let mut paths = vec![];

        let excepts: Vec<String> = self
            .nodes
            .iter()
            .filter(|n| n.max_visits == 1 && n.name != "start" && n.name != "end")
            .map(|n| n.name.to_string())
            .collect();

        for except in &excepts {
            let starting_node: &Rc<Node> = self.get_node_by_name("start").unwrap();
            let mut visits = vec![Rc::clone(starting_node)];
            let mut path = vec![];
            self.depth_first_traversal(&mut visits, &mut path, except);
            for p in path {
                paths.push(to_string(&p))
            }
        }

        paths.sort();
        paths.dedup();
        paths
    }

    fn depth_first_traversal(
        &mut self,
        visits: &mut Vec<Rc<Node>>,
        paths: &mut Vec<Vec<Rc<Node>>>,
        except: &str,
    ) {
        let current_node = visits.last().unwrap();
        let adjacent_nodes = self.get_next_nodes(&current_node.name);

        for next_node in adjacent_nodes {
            let already_visited = Self::search_in_visited(visits, &next_node.name);
            if except == next_node.name {
                if already_visited >= next_node.max_visits + 1 {
                    continue;
                }
            } else {
                if already_visited >= next_node.max_visits {
                    continue;
                }
            }

            visits.push(Rc::clone(&next_node));
            self.depth_first_traversal(visits, paths, except);
        }

        // If we reached the end we have a complete path.
        if visits.last().unwrap().name == "end" {
            paths.push(visits.clone());
        }
        visits.pop();
    }

    fn get_node_by_name(&self, name: &str) -> Option<&Rc<Node>> {
        for n in &self.nodes {
            if n.name == name {
                return Some(n);
            }
        }
        None
    }

    fn get_next_nodes(&self, name: &str) -> Vec<Rc<Node>> {
        let mut next = Vec::new();
        for edge in &self.edges {
            if edge.0.name == name {
                next.push(Rc::clone(&edge.1))
            }
        }
        next
    }

    fn search_in_visited(visits: &Vec<Rc<Node>>, node_name: &str) -> u32 {
        let mut count = 0;
        for p in visits {
            if p.name == node_name {
                count += 1;
            }
        }
        count
    }
}

fn calc_max_visits(name: &str) -> u32 {
    if name.chars().any(|c| c.is_lowercase()) {
        1
    } else {
        u32::MAX
    }
}

fn to_string(path: &Vec<Rc<Node>>) -> String {
    path.iter()
        .map(|p| p.name.to_string())
        .collect::<Vec<String>>()
        .join(",")
}
