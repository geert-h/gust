pub enum Tree<T> {
    Node {
        data: T,
        children: Vec<Tree<T>>,
    },
    Leaf {
        data: T,
    },
}

impl<T: Default> Tree<T> {
    pub fn new() -> Self {
        Tree::Node {
            data: Default::default(),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Tree<T>) {
        match self {
            Tree::Node { data, children } => {
                children.push(child);
            }
            Tree::Leaf { .. } => {
                panic!("Cannot add child to leaf");
            }
        }
    }

    pub fn add_data(&mut self, data: T) {
        match self {
            Tree::Node { .. } => {
                panic!("Cannot add data to node");
            }
            Tree::Leaf { data: existing_data } => {
                *existing_data = data;
            }
        }
    }
}
