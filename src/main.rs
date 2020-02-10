use std::collections::HashMap;
use std::io;

struct LNum {
    ds: HashMap<usize, usize>,
}

impl LNum {
    fn new(mut n: usize) -> LNum {
        let mut obj = LNum { ds: HashMap::new() };
        let mut i = 0;
        while n > 0 {
            obj.ds.insert(i, n % 10);
            i = i + 1;
            n = n / 10;
        }
        return obj;
    }

    fn mm(&mut self, m: usize) -> LNum {
        let mut f = LNum::new(0);
        for i in self.ds.keys() {
            let md = match self.ds.get(i) {
                Some(d) => &d,
                None => &0,
            };
            let mut res = md * m;
            let mut pp = 0;
            while res > 0 {
                let d = match f.ds.get(&(i + pp)) {
                    Some(di) => &di,
                    None => &0,
                };
                res = res + d;
                f.ds.insert(i + pp, res % 10);
                res = res / 10;
                pp = pp + 1;
            }
        }
        return f;
    }

    fn pri(&mut self) -> String {
        let max = match self.ds.keys().max() {
            Some(v) => &v,
            None => &0,
        } + 1;
        let p: String = (0..max)
            .rev()
            .map(|i| match self.ds.get(&i) {
                Some(d) => d.to_string(),
                None => "0".to_string(),
            })
            .collect();
        return p;
    }
}

fn ri() -> String {
    let mut i: String = String::new();
    io::stdin().read_line(&mut i).expect("");
    return i.trim().parse().expect("");
}

fn fac(mut n: usize) -> LNum {
    let mut num = LNum::new(1);
    while n > 0 {
        num = num.mm(n);
        n = n - 1;
    }
    return num;
}

fn main() {
    let l: usize = ri().parse().expect("");

    for _index in 0..l {
        let n = ri().parse().expect("");
        println!("{}", fac(n).pri());
    }
}
