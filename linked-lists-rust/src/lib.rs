mod ll {

    use std::mem;

    pub struct LinkedList {
        tail: Link
    }

    struct Node {
        data: u32,
        next: Link,
    }

    enum Link {
        Nothing,
        Something(Box<Node>),
    }

    impl LinkedList {

        pub fn new() -> Self {
            LinkedList {
                tail: Link::Nothing,
            }
        }

        pub fn insert(
            &mut self,
            data: u32
        ) {

            let node = Box::new(Node {
                data: data,
                next: mem::replace(&mut self.tail, Link::Nothing)
            });

            self.tail = Link::Something(node);
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_create() {

        use ll;

        let list = ll::LinkedList::new();
    }
}
