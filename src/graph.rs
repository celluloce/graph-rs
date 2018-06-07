use std::fmt;

#[derive(Clone)]
pub struct Edge
{
    pub node : (String, String),
    pub weight : usize,
}

impl Edge
{
    pub fn new(node : (&str, &str), weight : usize) -> Edge {
        let node = match node {
            (n1, n2) => (n1.to_string(), n2.to_string())
        };
        Edge{node, weight}
    }
}

impl fmt::Debug for Edge
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Edge({:?}, {:?})", self.node, self.weight)
    }
}