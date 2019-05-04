use std::mem;

struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::new();
        // Check empty list
        assert_eq!(list.pop(), None);
        list.push(1);
        list.push(2);
        list.push(3);
        // check removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        // add more
        list.push(4);
        list.push(5);
        // check removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        // final check
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
