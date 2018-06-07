use std::hash::Hash;
use std::fmt;
use std::ops::Add;
use std::collections::{HashMap, HashSet};
use graph::Edge;

struct UnionFind {
    pars : HashMap<String, String>,
    sizes : HashMap<String, usize>,
}

impl UnionFind {
    fn new(nodes : HashSet<String>) -> UnionFind {
        let sizes = nodes.iter().map(|x| (x.clone(), 1)).collect();
        let pars = nodes.into_iter().map(|x| (x.clone(), x)).collect();
        UnionFind{pars, sizes}
    }

    fn find(&self, key : &str) -> Option<String>{
        if let Some(val) = Some(key.to_string()) {
            match self.pars.get(&val) {
                Some(par) =>
                    if &val == par {
                        Some(val)
                    } else {
                        self.find(par)
                    },
                None => None,
            }
        } else {
            None
        }
    }

    fn size(&self, key : &str) -> Option<usize> {
        if let Some(par) = self.find(key) {
            self.sizes.get(&par).map(|x| *x)
        } else {
            None
        }
    }

    fn unite(&mut self, x : &str, y : &str) {
        if let (Some(x), Some(y)) = (self.find(x), self.find(y)) {
            if x != y {
                if let (Some(sx), Some(sy)) = (self.size(&x), self.size(&y)) {
                    let (x, y) = if sx > sy {(x, y)} else {(y, x)};

                    self.pars.insert(y.clone(), x.clone());
                    
                    self.sizes.insert(x, sx + sy);
                    self.sizes.remove(&y);
                }
            }
        }
    }

    fn same_tree(&mut self, x : &str, y : &str) -> Option<bool> {
        if let (Some(x), Some(y)) = (self.find(x), self.find(y)) {
            Some(x == y)
        } else {
            None
        }
    }
}

fn id<T>(v: T) -> T { v }

pub fn run<T>(edges : HashSet<Edge<T>>) -> HashSet<Edge<T>>
    where T : PartialEq + Hash + Ord + Clone + fmt::Debug + Add
{
    
    let edges = {
        let mut edges : Vec<Edge<T>> = edges.into_iter().collect();
        edges.sort_by_key(|e| e.weight.clone());
        edges.into_iter()
    };

    let nodes : HashSet<String> = edges.clone()
        .flat_map(|e| {
            let (n1, n2) = e.node;
            vec![n1, n2].into_iter()
        }).collect();

    let mut uf = UnionFind::new(nodes);

    let result : HashSet<Edge<T>> = edges.map(|e| {
        let (n1, n2) = &e.node.clone();
        if let Some(b) = uf.same_tree(n1, n2) {
            if !b {
                uf.unite(n1, n2);
                Some(e)
            } else {
                None
            }
        } else {
            None
        }
    }).filter_map(id).collect();

    result
}