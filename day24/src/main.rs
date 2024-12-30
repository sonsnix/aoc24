use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let graph = build_graph(&input);
    
    let sets_of_three = find_sets_of_three(&graph);
    println!("Part 1: {}", sets_of_three.len());
    
    let parties = find_parties(&graph);
    let password = get_largest_party_password(&parties);
    println!("Part 2: {}", password);
}

fn build_graph(input: &str) -> HashMap<String, HashSet<String>> {
    let mut graph = HashMap::new();
    
    for line in input.lines() {
        let (computer_a, computer_b) = line.split('-')
            .map(String::from)
            .collect_tuple()
            .unwrap();
        
        // Add bidirectional connections
        graph.entry(computer_a.clone())
            .or_insert_with(HashSet::new)
            .insert(computer_b.clone());
            
        graph.entry(computer_b)
            .or_insert_with(HashSet::new)
            .insert(computer_a);
    }
    
    graph
}

fn find_sets_of_three(graph: &HashMap<String, HashSet<String>>) -> HashSet<Vec<String>> {
    let mut sets_of_three = HashSet::new();
    
    for (computer_a, connections) in graph {
        let valid_combinations = connections
            .iter()
            .combinations(2)
            .filter(|pair| {
                graph[pair[0]].contains(pair[1]) &&
                (computer_a.starts_with('t') || 
                 pair[0].starts_with('t') || 
                 pair[1].starts_with('t'))
            })
            .map(|pair| {
                [computer_a.clone(), pair[0].clone(), pair[1].clone()]
                    .into_iter()
                    .sorted()
                    .collect_vec()
            });
            
        sets_of_three.extend(valid_combinations);
    }
    
    sets_of_three
}

fn find_parties(graph: &HashMap<String, HashSet<String>>) -> Vec<HashSet<String>> {
    let mut parties: Vec<HashSet<String>> = Vec::new();
    
    for (computer_a, connections) in graph {
        // Try to add computer to existing parties
        for party in &mut parties {
            if party
                .iter()
                .all(|computer_b| connections.contains(computer_b))
            {
                party.insert(computer_a.clone());
            }
        }
        
        // Create new party with just this computer
        parties.push(HashSet::from([computer_a.clone()]));
    }
    
    parties
}

fn get_largest_party_password(parties: &[HashSet<String>]) -> String {
    parties
        .iter()
        .max_by_key(|party| party.len())
        .unwrap()
        .iter()
        .sorted()
        .join(",")
}