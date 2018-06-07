use std::hash::{Hash, Hasher};
use std::fmt;
use std::ops::Add;

#[derive(Clone)]
pub struct Edge<T>
    where T : PartialEq + Hash + Ord + Clone + fmt::Debug + Add
{
    pub node : (String, String),
    pub weight : T,
}

impl<T> Edge<T>
    where T : PartialEq + Hash + Ord + Clone + fmt::Debug + Add
{
    pub fn new(node : (&str, &str), weight : T) -> Edge<T> {
        let node = match node {
            (n1, n2) => (n1.to_string(), n2.to_string())
        };
        Edge{node, weight}
    }
}

impl<T> fmt::Debug for Edge<T>
    where T : PartialEq + Hash + Ord + Clone + fmt::Debug + Add
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Edge({:?}, {:?})", self.node, self.weight)
    }
}

impl<T> PartialEq for Edge<T>
    where T : PartialEq + Hash + Ord + Clone + fmt::Debug + Add
{
    fn eq(&self, other : &Edge<T>) -> bool {
        self.node == other.node && self.weight == other.weight
    }
}

impl<T> Eq for Edge<T> where T : PartialEq + Hash + Ord + Clone + fmt::Debug + Add {}


impl<T> Hash for Edge<T>
    where T : PartialEq + Hash + Ord + Clone + fmt::Debug + Add
{
    fn hash<H : Hasher>(&self, state : &mut H) {
        self.node.hash(state);
        self.weight.hash(state);
    }
}