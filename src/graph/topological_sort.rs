type Node = u32;

pub fn topological_sort(graph: &Vec<Vec<Node>>) -> Vec<Node>
{
    let n = graph.len();
    let mut in_degree = vec![0;n];
    for v in 0..n
    {
        for &to in graph[v].iter()
        {
            in_degree[to as usize] += 1;
        }
    }
    
    use crate::data_structure::min_heap::min_heap::MinHeap;
    let mut q = MinHeap::new();
    
    for i in 0..n
    {
        if in_degree[i] == 0
        {
            q.push(i as Node);
        }
    }

    let mut ret = vec![];
    while let Some(v) = q.pop()
    {
        for next in &graph[v as usize]
        {
            in_degree[*next as usize] -= 1;
            if in_degree[*next as usize] == 0
            {
                q.push(*next as Node);
            }
        }
        ret.push(v as Node);
    }
    ret
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topological_sort() {
        let graph = vec![vec![1], vec![2], vec![], vec![1, 4], vec![5], vec![2]];
        let result = topological_sort(&graph);
        assert_eq!(vec![0, 3, 1, 4, 5, 2], result);
    }
}