use std::{collections::VecDeque, fmt::Debug};

mod test;

const MAX_SIZE: usize = 3; // NOTE: Must be odd

pub struct BTree<T: PartialOrd + Copy + std::fmt::Debug> {
    root: BTreeNode<T>
}

impl<T: PartialOrd + Copy + Debug> BTree<T> {
    pub fn insert(&mut self, val: T) {
        let res = self.root.insert(val);
        if let Some(inserted) = res {
            self.root = BTreeNode{
                keys: VecDeque::from(vec![inserted.0]),
                children: Some(VecDeque::from(vec![inserted.1, inserted.2])),
            }
        }
    }

    pub fn new() -> Self {
        Self {
            root: BTreeNode { keys: VecDeque::new(), children: None }
        }
    }
}

impl<T: PartialOrd + Copy + std::fmt::Debug> Debug for BTree<T>  {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.root.fmt(f)
    }
}

struct BTreeNode<T: PartialOrd + Copy + Debug> {
    keys: VecDeque<T>,
    children: Option<VecDeque<Box<BTreeNode<T>>>>,
}

impl<T: PartialOrd + Copy + std::fmt::Debug> BTreeNode<T> {
    fn insert(&mut self, val: T) -> Option<(T, Box<BTreeNode<T>>, Box<BTreeNode<T>>)> {
        // Binary search keys to find where to insert
        /* let mut l = 0;
        let mut h = self.keys.len();
        while (h - l) > 1 {
            let mid = (l + h) / 2;
            let v = self.keys[mid];
            if v < val {
                l = mid;
            } else if val < v {
                h = mid;
            } else {
                panic!("Value already exists");
            }
        }*/
        
        // Linear search until i can get binary search working
        let mut h = 0;
        while self.keys.len() > 0 && h < self.keys.len() && self.keys[h] < val {
            h += 1;
        }
        
        match &mut self.children {
            Some(children) => { // Inner node
                // Insert into that
                if let Some(insertion) = children[h].insert(val) {
                    self.keys.insert(h, insertion.0);
                    let c = self.children.as_mut().unwrap();
                    c.remove(h);
                    c.insert(h, insertion.1);
                    c.insert(h + 1, insertion.2);

                    if self.keys.len() >= MAX_SIZE {
                        Some(self.split())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            None => { // Leaf node
                // See if need to split
                self.keys.insert(h, val);

                if self.keys.len() >= MAX_SIZE {
                    Some(self.split())
                } else {
                    None
                }
            }
        }
    }

    fn split(&mut self) -> (T, Box<BTreeNode<T>>, Box<BTreeNode<T>>) {
        let median = self.keys[self.keys.len()/2];
        let mut l = VecDeque::with_capacity(MAX_SIZE/2);
        for _ in 0..MAX_SIZE/2 {
            l.push_back(self.keys.pop_front().unwrap());
        }
        self.keys.pop_front();

        // Copy R for Rust to be happy
        let mut new_r = VecDeque::new();
        new_r.append(&mut self.keys);

        let mut lval = BTreeNode{
            keys: l,
            children: None
        };
        let mut rval = BTreeNode{
            keys: new_r,
            children: None,
        };

        // Split children if available
        match &mut self.children {
            Some(children) => {
                let point = MAX_SIZE/2 + 1;
                let mut l = VecDeque::with_capacity(point);
                for _ in 0..point  {
                    l.push_back(children.pop_front().unwrap());
                }
                lval.children = Some(l);

                let mut r = VecDeque::new(); // To make rust happy
                r.append(children);
                rval.children = Some(r);
            }
            None => ()
        }

        (median, Box::new(lval), Box::new(rval))
    }
}

impl<T: PartialOrd + Copy + std::fmt::Debug> Debug for BTreeNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(children) = &self.children {
            for child in children.iter() {
                write!(f, "{:?}", child)?;
            }
        }
        write!(f, "{:?}", self.keys)?;
        Ok(())
    }
}
