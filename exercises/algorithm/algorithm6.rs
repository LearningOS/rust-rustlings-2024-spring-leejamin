/*
	dfs
	This problem requires you to implement a basic DFS traversal
*/

// I AM NOT DONE
use std::collections::HashSet;

struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src); 
    }

    fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {
        //TODO
        if !visited.contains(&v) {
            visited.insert(v);
            visit_order.push(v);
        }
        let mut nodes = Vec::<Vec<usize>>::new();
        nodes.push(self.adj[v].clone());

        let mut i = 0;
        loop {
            if nodes[i].is_empty() {
                if i == 0 {
                    break;
                } else {
                    i -= 1;
                    continue;
                }
            }
            let ele = nodes[i][0];
            nodes[i].remove(0);
            if visited.contains(&ele) {
                continue;
            }
            
            visit_order.push(ele);
            visited.insert(ele);
            let mut tmp = vec![];
            for ele in &self.adj[ele] {
                if !visited.contains(ele) {
                    tmp.push(*ele);
                }
            }
            if !tmp.is_empty() {
                nodes.push(tmp);
                i += 1;
            }
        }

    }

    // Perform a depth-first search on the graph, return the order of visited nodes
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut visit_order = Vec::new(); 
        self.dfs_util(start, &mut visited, &mut visit_order);
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_simple() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_dfs_with_cycle() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_dfs_disconnected_graph() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(3, 4); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]); 
        let visit_order_disconnected = graph.dfs(3);
        assert_eq!(visit_order_disconnected, vec![3, 4]); 
    }
}
