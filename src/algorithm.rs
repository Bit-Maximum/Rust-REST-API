use std::cmp::{min, Reverse};
use std::collections::HashMap;
use priority_queue::PriorityQueue;
use crate::models::{City, Road};


const INF: i32 = 10_i32.pow(9);

pub fn build_graph(nodes: Vec<City>, edges: Vec<Road>) -> HashMap<String, Vec<(i32, String)>> {
    let mut graph: HashMap<String, Vec<(i32, String)>>  = HashMap::new();
    let mut cities: HashMap<i32, String>  = HashMap::new();

    for city in nodes.clone() {
        cities.insert(city.id.expect("REASON"), city.name.clone());
        graph.insert(city.name, Vec::new());
    }
    // for (k, v) in cities.clone() {
    //     println!("{} || {}", k, v);
    // }

    for road in edges.clone() {
        let city_a = cities.get(&road.city_a).unwrap();
        let city_b = cities.get(&road.city_b).unwrap();
        // println!("-------");
        // println!("{} || {}", city_a, city_b);
        let mut roads_a = graph.get(city_a).unwrap().clone();
        let mut roads_b = graph.get(city_b).unwrap().clone();

        roads_a.push((road.length, city_b.clone()));
        roads_b.push((road.length, city_a.clone()));

        graph.insert(city_a.clone(), roads_a.clone());
        graph.insert(city_b.clone(), roads_b.clone());
    }
    graph
}

///
/// # Dijkstra algorithm for dense graphs
/// O(N^2)
/// # Arguments
///
/// * `n`<i32>: count of all nodes
/// * `start`<i32>: starting node
/// * `dist`<Vec<i32>>: adjacency list with Pairs (Connected node, distance to node)
///
/// returns: Vec<i32> => Min distance to every node from 'start'
///
pub fn dijkstra_cost(n: usize, start:usize, dist: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![INF; n ];
    let mut visited: Vec<bool> = vec![false; n];
    result[start] = 0;

    for _ in 0..n {
        let mut curent: i32 = -1;
        for j in 0..n {
            if !visited[j] && (curent == -1 || result[j] < result[curent as usize]) {
                curent = j as i32;
            }
        }
        visited[curent as usize] = true;
        for (index, lenth) in dist.iter().enumerate() {
            result[index] = min(result[index], result[curent as usize] + lenth);
        }
    }
    result
}

// Работа с структурами данных в Rust - это боль :(
// ///
// /// # Dijkstra algorithm for sparse graphs
// /// O(M*log(N))
// /// # Arguments
// ///
// /// * `n`<i32>: count of all nodes
// /// * `start`<i32>: starting node
// /// * `dist`<Vec<i32>>: adjacency list with Pairs (Connected node, distance to node)
// ///
// /// returns: Vec<i32> => Path to every node from 'start'
// ///
// pub fn dijkstra_path(n: usize, start: usize, dist: Vec<i32>) -> Vec<i32> {
//     let mut dijkstra: Vec<i32> = vec![INF; n];
//     let mut path: Vec<i32> = vec![-1; n];
//     dijkstra[start] = 0;
//
//     let mut pq: PriorityQueue<i32, Reverse<i32>> = PriorityQueue::new();
//     pq.push(start as i32, Reverse(0));
//     while !pq.is_empty() {
//         let (current_node, current_distance) = pq.pop().unwrap();
//         if (current_distance > dijkstra[current_node]) { continue; }
//         for (index, lenth) in dist.iter().enumerate() {
//             if dijkstra[index] > dijkstra[current_node] + lenth {
//                 dijkstra[index] = dijkstra[current_node] + lenth;
//                 path[index] = current_node;
//                 pq.push(index as i32, Reverse(dijkstra[index]));
//             }
//         }
//     }
//     path
// }


///
/// Dijkstra on HashMap
pub fn dijkstra(start: String, goal: String, graph: HashMap<String, Vec<(i32, String)>>) -> HashMap<String, (Option<String>, i32)> {
    let mut pq: PriorityQueue<String, Reverse<i32>> = PriorityQueue::new();
    pq.push(start.clone(), Reverse(0));

    let mut cost: HashMap<String, i32> = HashMap::new();
    let mut path: HashMap<String, (Option<String>, i32)> = HashMap::new();
    cost.insert(start.clone(), 0);
    path.insert(start.clone(), (None, 0));

    while !pq.is_empty() {
        let (current_node, _) = pq.pop().unwrap();
        if current_node == goal { break };

        for (next_cost, next_node) in graph[&current_node].clone() {
            let new_cost = cost.get(&current_node.clone()).unwrap() + next_cost;

            if !cost.contains_key(&next_node) || new_cost < cost[&next_node] {
                pq.push(next_node.clone(), Reverse(new_cost));
                cost.insert(next_node.clone(), new_cost);
                path.insert(next_node.clone(), (Some(current_node.clone()), new_cost));
            };
        }
    }
    path
}


pub fn format_path(start: String, end: String, path: &HashMap<String, (Option<String>, i32)>) -> String {
    let mut current_node = end.clone();
    let mut reverse = Vec::new();
    while current_node != start {
        reverse.push(current_node.clone());
        current_node = path.get(&current_node.clone()).unwrap().0.clone().unwrap();
    }

    let mut result = format!("Path from {start} to {end}: \n{start} ", start=start, end=end);
    for node in reverse.iter().rev() {
        result += &format!("---> {node} ", node=node);
    }
    result
}