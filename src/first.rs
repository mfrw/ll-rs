use std::mem;

pub struct List {
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
        List{head: Link::Empty}
    }

    pub fn push(&mut self, elem: i32) {
        let n = Box::new(Node{
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(n);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            },
        }
    }
}

mod test {
    use super::*;
    #[test]
    fn basics() {
        let mut l = List::new();
        assert_eq!(l.pop(), None);

        l.push(1);
        l.push(2);
        l.push(3);

        assert_eq!(l.pop(), Some(3));
        assert_eq!(l.pop(), Some(2));

        l.push(4);
        l.push(5);

        // Check normal removal
        assert_eq!(l.pop(), Some(5));
        assert_eq!(l.pop(), Some(4));

        // Check exhaustion
        assert_eq!(l.pop(), Some(1));
        assert_eq!(l.pop(), None);
    }
}
