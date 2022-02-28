use colored::*;
use std::collections::HashMap;
use std::ops::Index;

pub struct Graph(Vec<Vec<u32>>);

impl Graph {
    pub fn from_lines<I>(lines: I) -> Self
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let nodes = lines
            .into_iter()
            .map(|line| {
                line.as_ref()
                    .chars()
                    .map(|ch| ch.to_digit(10).unwrap())
                    .collect()
            })
            .collect();
        Graph(nodes)
    }

    pub fn neighbors(&self, current: (usize, usize)) -> [Option<(usize, usize)>; 4] {
        let mut neighbors = [None, None, None, None];
        if current.0 > 0 {
            neighbors[0] = Some((current.0 - 1, current.1));
        }
        if current.1 <= self.0[0].len() - 2 {
            neighbors[1] = Some((current.0, current.1 + 1));
        }
        if current.0 <= self.0.len() - 2 {
            neighbors[2] = Some((current.0 + 1, current.1));
        }
        if current.1 > 0 {
            neighbors[3] = Some((current.0, current.1 - 1));
        }
        neighbors
    }

    pub fn enlarge(&mut self) {
        let initial_clone = self.0.clone();
        for i in 1..=4 {
            for row in &initial_clone {
                self.0.push(
                    row.iter()
                        .map(|v| if v + i >= 10 { (v + i) % 9 } else { v + i })
                        .collect(),
                );
            }
        }

        for i in 0..self.0.len() {
            let row_values = self.0[i].clone();
            for t in 1..=4 {
                for val in &row_values {
                    let mut new_val = val + t;
                    if new_val >= 10 {
                        new_val %= 9;
                    }
                    self.0[i].push(new_val);
                }
            }
        }
    }

    pub fn len(&self) -> (usize, usize) {
        let rows = self.0.len();
        let cols = self.0[0].len();
        (rows, cols)
    }

    pub fn print(&self, path: &Vec<(usize, usize)>) {
        let rows = self.0.iter().enumerate();
        for (r, row) in rows {
            for (c, val) in row.iter().enumerate() {
                if path.contains(&(r, c)) {
                    print!("{} ", val.to_string().red());
                } else {
                    print!("{} ", val.to_string());
                }
            }
            print!("\n");
        }
    }
}

impl Index<(usize, usize)> for Graph {
    type Output = u32;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

#[derive(Debug)]
pub struct Labels {
    stable: HashMap<(usize, usize), Label>,
    frontier: HashMap<(usize, usize), Label>,
}

#[derive(Debug)]
pub struct Label {
    pub distance: u32,
    pub next: (usize, usize),
}

impl Labels {
    pub fn new_from_graph(graph: &Graph) -> Labels {
        let mut labels = Labels {
            stable: HashMap::new(),
            frontier: HashMap::new(),
        };
        labels.stable.insert(
            (0, 0),
            Label {
                distance: 0,
                next: (0, 0),
            },
        );
        labels.frontier.insert(
            (0, 1),
            Label {
                distance: graph.0[0][1],
                next: (0, 0),
            },
        );
        labels.frontier.insert(
            (1, 0),
            Label {
                distance: graph.0[1][0],
                next: (0, 0),
            },
        );
        labels
    }

    pub fn find_nearest_in_frontier(&self) -> Option<((usize, usize), &Label)> {
        match self.frontier.iter().min_by_key(|(_, v)| v.distance) {
            Some(((row, col), label)) => Some(((*row, *col), label)),
            None => None,
        }
    }

    pub fn get_shortest_path(&self, end: (usize, usize)) -> (Vec<(usize, usize)>, u32) {
        let total_distance = self.stable.get(&end).unwrap().distance;
        let mut path = vec![end];
        loop {
            let current = self.stable.get(path.last().unwrap()).unwrap();
            path.push(current.next);
            if current.next == (0, 0) {
                break;
            }
        }
        path.reverse();
        (path, total_distance)
    }

    // Stable labels.
    pub fn move_to_stable(&mut self, label_key: (usize, usize)) {
        let label = self.frontier.remove(&label_key).unwrap();
        self.stable.insert(label_key, label);
        self.frontier.remove(&label_key);
    }

    pub fn is_in_stable(&self, label_key: (usize, usize)) -> bool {
        self.stable.contains_key(&label_key)
    }

    // Frontier labels.
    pub fn get_from_frontier(&mut self, label_key: (usize, usize)) -> Option<&mut Label> {
        self.frontier.get_mut(&label_key)
    }

    pub fn insert_in_frontier(&mut self, key: (usize, usize), label: Label) {
        self.frontier.insert(key, label);
    }

    pub fn is_frontier_ended(&self) -> bool {
        self.frontier.len() == 0
    }
}

impl Index<(usize, usize)> for Labels {
    type Output = Label;
    fn index(&self, index: (usize, usize)) -> &Label {
        self.stable.get(&index).unwrap()
    }
}
