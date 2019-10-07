#[derive(Clone)]
pub struct TriangularMatrix<T: Copy> {
    count: usize,
    table: Vec<T>,
    invertor: fn(T) -> T
}

impl <T: Copy> TriangularMatrix<T> {
    pub fn new (count: usize, default: T, invertor: fn(T) -> T) -> Result<TriangularMatrix<T>, &'static str> {
        let result = if count > 0 {
            let limit = std::usize::MAX / count;
            if count - 1 >= limit {
                Err("Count too high")
            } else {
                Ok(vec![default; count * (count - 1) / 2])
            }
        } else {
            Ok(Vec::new())
        };
        match result {
            Ok(table) => Ok(TriangularMatrix { count, table, invertor }),
            Err(string) => Err(string)
        }
    }
    #[cfg(test)]
    pub fn num_combinations(&self) -> usize {
        self.table.len()
    }
    pub fn get(&self, i: usize, j: usize) -> Option<T> {
        match self.resolve(i, j) {
            None => None,
            Some((index, inverted)) => {
                let option = self.table.get(index);
                let value = *option.unwrap();
                if inverted {
                    let inverted = (self.invertor)(value);
                    Some(inverted)
                } else {
                    Some(value)
                }
            }
        }
    }
    pub fn set(&mut self, i: usize, j: usize, value: T) -> bool {
        match self.resolve(i, j) {
            None => false,
            Some((index, false)) => {
                self.table[index] = value;
                true
            },
            Some((index, true)) => {
                let inverted = (self.invertor)(value);
                self.table[index] = inverted;
                true
            }
        }
    }
    fn resolve(&self, i: usize, j: usize) -> Option<(usize, bool)> {
        if i >= self.count { return None }
        if j >= self.count { return None }
        if i == j { return None }

        let (lo, hi, inverted) = if i < j {
            (i, j, false)
        } else {
            (j, i, true)
        };
        Some(((hi * (hi - 1)) / 2 + lo, inverted))
    }
}
#[test]
fn triangular_matrix_test() {
    let nc = 0i64;
    let tie = 1i64;
    let win = 2i64;
    let loss = 3i64;
    fn invertor(x: i64) -> i64 {
        if x == 0i64 || x == 1i64 {
            x
        } else if x == 2i64 {
            3i64
        } else {
            2i64
        }
    };
    let mut t = TriangularMatrix::<i64>::new(5, nc, invertor).unwrap();

    assert_eq!(10, t.num_combinations());
    assert_eq!(t.get(0, 1), Some(0));
    assert_eq!(t.get(3, 4), Some(0));
    assert_eq!(t.get(0, 0), None);
    assert_eq!(t.get(0, 5), None);
    assert_eq!(t.get(5, 0), None);

    let ars = 0;
    let ast = 1;
    let che = 2;
    let mnc = 3;
    let liv = 4;

    t.set(ars, ast, win);
    t.set(che, ars, win);
    t.set(mnc, ars, loss);
    t.set(liv, ars, loss);
    t.set(che, ast, tie);
    t.set(ast, mnc, win);
    t.set(ast, liv, win);
    t.set(mnc, che, tie);
    t.set(che, liv, tie);
    t.set(liv, mnc, win);

    assert_eq!(t.resolve(ars, ast), Some((0, false)));
    assert_eq!(t.resolve(ast, ars), Some((0, true)));
    assert_eq!(t.resolve(ars, che), Some((1, false)));
    assert_eq!(t.resolve(che, ars), Some((1, true)));
    assert_eq!(t.resolve(ars, mnc), Some((3, false)));
    assert_eq!(t.resolve(mnc, ars), Some((3, true)));
    assert_eq!(t.resolve(ars, liv), Some((6, false)));
    assert_eq!(t.resolve(liv, ars), Some((6, true)));

    assert_eq!(t.resolve(ast, che), Some((2, false)));
    assert_eq!(t.resolve(che, ast), Some((2, true)));
    assert_eq!(t.resolve(ast, mnc), Some((4, false)));
    assert_eq!(t.resolve(mnc, ast), Some((4, true)));
    assert_eq!(t.resolve(ast, liv), Some((7, false)));
    assert_eq!(t.resolve(liv, ast), Some((7, true)));

    assert_eq!(t.resolve(che, mnc), Some((5, false)));
    assert_eq!(t.resolve(mnc, che), Some((5, true)));
    assert_eq!(t.resolve(che, liv), Some((8, false)));
    assert_eq!(t.resolve(liv, che), Some((8, true)));

    assert_eq!(t.resolve(mnc, liv), Some((9, false)));
    assert_eq!(t.resolve(liv, mnc), Some((9, true)));

    assert_eq!(t.get(ars, ast), Some(win));
    assert_eq!(t.get(ast, ars), Some(loss));
    assert_eq!(t.get(ars, che), Some(loss));
    assert_eq!(t.get(che, ars), Some(win));
    assert_eq!(t.get(ars, mnc), Some(win));
    assert_eq!(t.get(mnc, ars), Some(loss));
    assert_eq!(t.get(ars, liv), Some(win));
    assert_eq!(t.get(liv, ars), Some(loss));
    assert_eq!(t.get(ast, che), Some(tie));
    assert_eq!(t.get(che, ast), Some(tie));
    assert_eq!(t.get(ast, mnc), Some(win));
    assert_eq!(t.get(mnc, ast), Some(loss));
    assert_eq!(t.get(ast, liv), Some(win));
    assert_eq!(t.get(liv, ast), Some(loss));
    assert_eq!(t.get(che, mnc), Some(tie));
    assert_eq!(t.get(mnc, che), Some(tie));
    assert_eq!(t.get(che, liv), Some(tie));
    assert_eq!(t.get(liv, che), Some(tie));
    assert_eq!(t.get(mnc, liv), Some(loss));
    assert_eq!(t.get(liv, mnc), Some(win));
}
