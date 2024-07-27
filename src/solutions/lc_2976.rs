use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use crate::structs::solution::Solution;

#[derive(Debug)]
struct Node {
    value: char,
    to: Vec<(char, i32)>
}

#[derive(Eq, PartialEq)]
 struct State {
     cost: i32,
     position: char,
 }

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl Solution {
    pub fn minimum_cost(source: String, target: String, original: Vec<char>, changed: Vec<char>, cost: Vec<i32>) -> i64 {
        let mut nodes = HashMap::new();
        for i in 0..original.len() {
            nodes.entry(original[i]).and_modify(|node: &mut Node| node.to.push((changed[i], cost[i]))).or_insert(Node { value: original[i], to: vec![(changed[i], cost[i])]});
        }
        let mut total_cost = 0;
        let mut cost_cache: HashMap<(char, char), i64> = HashMap::new();
        for idx in 0..source.len() {
            let from = source.as_bytes()[idx] as char;
            let to = target.as_bytes()[idx] as char;
            if let Some(cached) = cost_cache.get(&(from, to)) {
                total_cost += cached;
            } else {
                let cost = calc_change_cost(&nodes, from, to);
                if cost == -1 {
                    return -1;
                } else {
                    total_cost += cost;
                    cost_cache.insert((from, to), cost);
                }
            }
        }
        total_cost
    }
}

fn calc_change_cost(nodes: &HashMap<char, Node>, start: char, target: char) -> i64 {
    let mut distances = HashMap::new();
    let mut heap = BinaryHeap::new();

    for key in nodes.keys() {
        distances.insert(*key, i32::MAX);
    }

    distances.insert(start, 0);
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if position == target { return cost as i64 }

        if cost > *distances.get(&position).unwrap_or(&i32::MAX) {
            continue;
        }

        if let Some(node) = nodes.get(&position) {
            for &(neighbor, weight) in &node.to {
                let next = State { cost: cost + weight, position: neighbor };

                if next.cost < *distances.get(&neighbor).unwrap_or(&i32::MAX) {
                    distances.insert(neighbor, next.cost);
                    heap.push(next);
                }
            }
        }
    }
    -1
}