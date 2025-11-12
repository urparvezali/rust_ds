#![allow(dead_code)]

use std::collections::BTreeSet;
pub fn dijkstra(source: usize, g: &Vec<Vec<(usize, usize)>>) -> Vec<usize> {
    let mut dist = vec![u32::MAX as usize; g.len()];
    let mut pq = BTreeSet::new();
    dist[source] = 0;
    pq.insert((0, source));

    while let Some((d, v)) = pq.pop_first() {
        if dist[v] < d {
            continue;
        }
        for &(u, wt) in &g[v] {
            if dist[v] + wt < dist[u] {
                dist[u] = dist[v] + wt;
                pq.insert((dist[u], u));
            }
        }
    }
    dist
}
