pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;


struct Node {
    elem: i32,
    next: Link,
}


impl List {
    pub fn new() -> Self {
        List{head: None}
    }

    pub fn push(&mut self, elem: i32) {
        let n = Box::new(Node{
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(n);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            },
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        // `while let` == "do this thing until this pattern doesn't match"
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
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
