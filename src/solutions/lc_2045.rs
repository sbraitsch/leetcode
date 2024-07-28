use crate::structs::solution::Solution;
impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let mut adjacent = vec![vec![]; (n + 1) as usize];
        edges.into_iter().map(|edge| (edge[0] as usize, edge[1] as usize)).for_each(|(i, j)| {
            adjacent[i].push(j);
            adjacent[j].push(i);
        });
        let mut visited = vec![None; (n + 1) as usize];
        visited[1] = Some(0);
        let mut open = vec![1usize];
        let mut open_swap = vec![];

        let mut t = 0;
        loop {
            if (t / change) & 1 == 1{ // waiting for green
                t += change - (t % change);
            }
            t += time;

            for next in open.drain(..) {
                for &adj in adjacent[next].iter() {
                    if adj == n as usize && visited[adj].is_some() && visited[adj].unwrap_or(i32::MAX) != t {
                        return t;
                    }

                    match visited[adj] {
                        Some(cost) if cost < t => {
                            visited[adj] = Some(t);
                            open_swap.push(adj);
                        },
                        None => {
                            visited[adj] = Some(t);
                            open_swap.push(adj);
                        }
                        _ => {}
                    }
                }
            }
            std::mem::swap(&mut open, &mut open_swap);
        }
    }
}
