pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;


struct Node<T> {
    elem: T,
    next: Link<T>,
}


impl<T> List<T> {
    pub fn new() -> Self {
        List{head: None}
    }

    pub fn push(&mut self, elem: T) {
        let n = Box::new(Node{
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(n);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

mod test {
    use super::List;
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
