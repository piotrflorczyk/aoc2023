use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet, VecDeque},
};

fn parse_input() -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    include_str!("../../input/day25").lines().for_each(|line| {
        let (name, n_list) = line.split_once(": ").unwrap();
        n_list.split(' ').for_each(|n| {
            graph
                .entry(n.to_string())
                .or_default()
                .push(name.to_string());
        });
        graph
            .entry(name.to_string())
            .or_default()
            .extend(n_list.split(' ').map(|x| x.to_string()));
    });
    graph
}

fn count_subgraphs(graph: &HashMap<String, Vec<String>>) -> Vec<usize> {
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue = Vec::from_iter(graph.keys().map(|k| (k, false)));
    let mut count = 0;
    let mut counts = vec![];
    while let Some((v, connected)) = queue.pop() {
        if visited.contains(v) {
            continue;
        }
        if !connected && count != 0 {
            counts.push(count);
            count = 0
        }
        count += 1;
        visited.insert(v.clone());
        for neighbour in &graph[v] {
            if !visited.contains(neighbour) {
                queue.push((neighbour, true))
            }
        }
    }
    counts.push(count);
    counts
}

fn remove_edge(edge: (&String, &String), graph: &mut HashMap<String, Vec<String>>) {
    let (v1, v2) = edge;
    let v1_idx = graph[v1].iter().position(|v| *v == *v2);
    let v2_idx = graph[v2].iter().position(|v| *v == *v1);
    graph.get_mut(v1).unwrap().remove(v1_idx.unwrap());
    graph.get_mut(v2).unwrap().remove(v2_idx.unwrap());
}

fn edge_frequencies(graph: &HashMap<String, Vec<String>>) -> HashMap<(String, String), usize> {
    let mut freq = HashMap::new();
    for v in graph.keys() {
        let mut queue = VecDeque::from([v]);
        let mut visited = HashSet::from([v]);

        while let Some(vi) = queue.pop_front() {
            graph[vi].iter().for_each(|n| {
                if !visited.contains(n) {
                    visited.insert(n);
                    let key = if n < vi {
                        (n.clone(), vi.clone())
                    } else {
                        (vi.clone(), n.clone())
                    };
                    freq.entry(key)
                        .and_modify(|counter| *counter += 1)
                        .or_insert(1);
                    queue.push_back(n);
                }
            })
        }
    }
    freq
}

fn main() {
    let mut graph = parse_input();
    let freq = edge_frequencies(&graph);
    let mut freq_vec = freq.iter().collect::<Vec<_>>();
    freq_vec.sort_by_key(|(_, fr)| Reverse(*fr));

    freq_vec.iter().take(3).for_each(|((v1, v2), _)| {
        remove_edge((&v1, &v2), &mut graph);
    });
    let counts = count_subgraphs(&graph);
    let res = counts.iter().product::<usize>();
    println!("p1: {res:?}");
}
