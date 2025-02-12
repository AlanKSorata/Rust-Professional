/*
    bfs
    This problem requires you to implement a basic BFS algorithm
*/

use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src);
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        // TODO: 实现广度优先搜索（BFS）并返回访问顺序

        // 用于存储访问顺序的向量
        let mut visit_order = vec![];

        // 用于存储待访问节点的队列
        let mut wait_vec = vec![];

        // 将起始节点加入访问顺序和待访问队列
        visit_order.push(start);
        wait_vec.push(start);

        // 临时变量，用于存储当前正在处理的节点
        let mut temp = start;

        // 当待访问队列不为空时，继续处理
        while !wait_vec.is_empty() {
            // 遍历当前节点的所有邻接节点
            for i in 0..self.adj[temp].len() {
                // 如果邻接节点未被访问过
                if !visit_order.contains(&self.adj[temp][i]) {
                    // 将邻接节点加入访问顺序
                    visit_order.push(self.adj[temp][i]);

                    // 如果邻接节点不在待访问队列中，则加入队列
                    if !wait_vec.contains(&self.adj[temp][i]) {
                        wait_vec.push(self.adj[temp][i]);
                    }
                }
            }

            // 移除队列中的第一个节点（当前节点）
            wait_vec.remove(0);

            // 如果队列不为空，更新临时变量为队列中的第一个节点
            if !wait_vec.is_empty() {
                temp = wait_vec[0];
            }
        }

        // 返回访问顺序
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}
