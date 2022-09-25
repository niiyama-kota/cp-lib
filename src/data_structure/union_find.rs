pub mod union_find
{
    pub struct UnionFind
    {
        _n: usize,
        parent_or_size: Vec<i32>
    }

    impl UnionFind
    {
        pub fn new(n: usize) -> Self
        {
            Self
            {
                _n: n,
                parent_or_size: vec![-1;n]
            }
        }

        pub fn merge(&mut self, a: i32, b: i32) -> i32
        {
            assert!(0 <= a && a < self._n.try_into().unwrap());
            assert!(0 <= b && b < self._n.try_into().unwrap());
            let mut x = self.leader(a);
            let mut y = self.leader(b);
            if x == y
            {
                return x;
            }
            if -self.parent_or_size[x as usize] < -self.parent_or_size[y as usize]
            {
                std::mem::swap(&mut x, &mut y);
            }
            self.parent_or_size[x as usize] += self.parent_or_size[y as usize];
            self.parent_or_size[y as usize] = x;
            return x;
        }

        pub fn same(&mut self, a: i32, b: i32) -> bool
        {
            assert!(0 <= a && a < self._n.try_into().unwrap());
            assert!(0 <= b && b < self._n.try_into().unwrap());
            return self.leader(a) == self.leader(b);
        }

        pub fn leader(&mut self, a: i32) -> i32
        {
            assert!(0 <= a && a < self._n.try_into().unwrap());
            if self.parent_or_size[a as usize] < 0
            {
                return a;
            }
            self.parent_or_size[a as usize] = self.leader(self.parent_or_size[a as usize]);
            self.parent_or_size[a as usize]
        }

        pub fn size(&mut self, a: i32) -> i32
        {
            assert!(0 <= a && a < self._n.try_into().unwrap());
            let s = self.leader(a);
            -self.parent_or_size[s as usize]
        }

        pub fn groups(&mut self) -> Vec<Vec<usize>>
        {
            let mut leader_buf = vec![0;self._n];
            let mut group_size = vec![0;self._n];
            for i in 0..self._n
            {
                leader_buf[i as usize] = self.leader(i.try_into().unwrap());
                group_size[leader_buf[i] as usize] += 1;
            }
            let mut result = vec![vec![]; self._n];
            for i in 0..self._n
            {
                result[i as usize].reserve(group_size[i]);
            }
            for i in 0..self._n
            {
                result[leader_buf[i as usize] as usize].push(i);
            }
            result.retain(|v| v.len() != 0);
            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::test_helper::Tester;

    /// Solve http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_A
    #[test]
    fn solve_dsl_1_a() {
        let tester = Tester::new("./assets/DSL_1_A/in/", "./assets/DSL_1_A/out/");

        tester.test_solution(|sc| {
            let n = sc.read();
            let q = sc.read();
            let mut uf = union_find::UnionFind::new(n);
            for _ in 0..q {
                let com: usize = sc.read();
                let x = sc.read();
                let y = sc.read();
                if com == 0 {
                    uf.merge(x, y);
                } else {
                    let ans = if uf.same(x, y) { 1 } else { 0 };
                    sc.write(format!("{}\n", ans));
                }
            }
        });
    }
}