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
        
		//TODO
        let mut visit_order = vec![];
        visit_order.push(start);
        let mut visit_queue: VecDeque<usize>  = VecDeque::new();         // 定义一个双端队列，前进后出
        visit_queue.push_front(start);
        while visit_queue.len() > 0 {
            let cur_node = visit_queue.pop_back().unwrap();       // while循环条件判断了queue非空，所以unwrap()是安全的
            for &side in &self.adj[cur_node] {                    // 遍历存储矩阵的每一行的每个边
                let mut have_visited = false;                      // 标记当前边是否已经被遍历过了
                for &visited_side in &visit_order{                // 查找已经遍历的边，看当前查找的边是否已经存在
                    if side == visited_side{
                        have_visited = true;                             // 存在，则标记为已经遍历过
                        break;                                           // 直接结束循环 
                    }
                }
                if have_visited == false{                                // 将没有遍历过的边放入队列中、放入遍历顺序表中 
                    visit_queue.push_front(side);
                    visit_order.push(side);
                }
            }
        }
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

