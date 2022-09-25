// https://atcoder.jp/contests/abc223/tasks/abc223_d
use rust_lib::graph::topological_sort::*;
use proconio::input;
use proconio::marker::Usize1;

fn main()
{
    input!
    {
        n: usize,
        m: usize,
    }

    let mut graph = vec![vec![]; n];
    for _ in 0 .. m
    {
        input!
        {
            u: Usize1,
            v: Usize1
        }
        graph[u].push(v as u32);
    }


    let result = topological_sort(&graph);
    if result.len() != n
    {
        println!("{}", -1);
    }
    else
    {
        for u in result.iter()
        {
            print!("{} ", u + 1)
        }
    }
}