use crate::triangulate::*;

#[derive(Debug, Clone)]
pub struct NodeList<'a> {
    pub head: usize,
    pub nodes: Vec<Node>,
    pub points: &'a Polygon,
    len: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub index: usize,
    pub prev: usize,
    pub next: usize
}


impl<'a> Into<NodeList<'a>> for &'a Polygon {
    fn into(self) -> NodeList<'a> {
        NodeList::new(self)
    }
}

impl<'a> NodeList<'a> {

    pub fn new(poly: &'a Polygon) -> Self {

        let mut list = NodeList{ head: 0, nodes: vec![], points: poly, len: poly.len() };

        for _ in poly {
            list.push_end();
        }

        list
    }

    pub fn remove_at(&mut self, index: usize) {

        self.len -= 1;

        if index == self.head {
            self.head = self.nodes[index].next;
        }

        let mut n = &mut self.nodes[index];
        let n_prev = n.prev;
        let n_next = n.next;

        n.prev = index;
        n.next = index;

        self.nodes[n_prev].next = n_next;
        self.nodes[n_next].prev = n_prev;
    }

    pub fn len(&self) -> usize {
        self.len
    }


    fn push_end(&mut self) {

        if self.nodes.len() == 0 {
            self.nodes.push( Node {
                index: 0,
                prev: 0,
                next: 0
            });
        }
        else {
            let prev = self.nodes.len() - 1;
            let index = self.nodes.len();
            self.nodes.push( Node {
                index,
                prev,
                next: 0
            });

            self.nodes[prev].next = index;
            self.nodes[0].prev = index;

        }
    }

    // used for debugging
    #[allow(dead_code)]
    fn print(&self) {
        for (i, node) in self.nodes.iter().enumerate() {
            println!("{} <- {:?} -> {}", node.prev, i, node.next);
        }
    }
}


impl<'a> std::iter::IntoIterator for &'a NodeList<'a> {

    type Item = usize;
    type IntoIter = NodeListIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let head = self.head;
        NodeListIterator {
            list: self,
            cur: head,
            moved: false
        }
    }

}


pub struct NodeListIterator<'a> {
    list: &'a NodeList<'a>,
    cur: usize,
    moved: bool
}


impl<'a> Iterator for NodeListIterator<'a> {

    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.moved {
            let node = Some(self.cur);

            self.cur = self.list.nodes[self.cur].next;
            self.moved = true;
            return node;

        }
        if self.list.nodes[self.cur].index == self.list.head {
            return None;
        }
        let node = Some(self.cur);
        self.cur = self.list.nodes[self.cur].next;

        node
    }
}



#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn create_list() {

         let square = vec![
                vector![0.0, 0.0],
                vector![0.0, 1.0],
                vector![1.0, 1.0],
                vector![1.0, 0.0],
            ];

        let list = NodeList::new(&square);

        assert_eq!(4, list.nodes.len());
        assert_eq!(2, list.nodes[3].prev);
        assert_eq!(0, list.nodes[3].next);
        check_list(&list);
    }



    #[test]
    fn modify_list() {

         let square = vec![
                vector![0.0, 0.0],
                vector![0.0, 1.0],
                vector![1.0, 1.0],
                vector![1.0, 0.0],
            ];

        let mut list = NodeList::new(&square);

        list.print();

        list.remove_at(1);
        println!("");

        list.print();

        assert_eq!(4, list.nodes.len());
        assert_eq!(2, list.nodes[3].prev);
        assert_eq!(0, list.nodes[3].next);

        assert_eq!(3, list.nodes[0].prev);
        assert_eq!(2, list.nodes[0].next);
        check_list(&list);



    }

    fn check_list(list: &NodeList) {

        for n in &list.nodes {

            assert_eq!(n.prev, list.nodes[n.prev].index);
            assert_eq!(n.index, list.nodes[n.prev].next);

            assert_eq!(n.next, list.nodes[n.next].index);
            assert_eq!(n.index, list.nodes[n.next].prev);

        }
    }
}
