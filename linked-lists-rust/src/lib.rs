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
                /* next = self.tail cannot be written because enumerations 
                   are moved by default; the value of self.tail cannot be moved
                   as it would let self in an incomplete state after movement;
                   replace() replaces the value at a mutable location with the new value,
                   returns the old one, so no moves occure */
                next: mem::replace(&mut self.tail, Link::Nothing)
            });

            self.tail = Link::Something(node);
        }

        pub fn pop(&mut self) -> Option<u32> {
            /* take the value inside self.tail without borrowing,
               insert Link::Nothing inside */
            match mem::replace(&mut self.tail, Link::Nothing) {
                Link::Something(value) => {

                    /* we have to move the whole node out of the box */
                    let node = *value;
                    self.tail = node.next;
                    Some(node.data)
                },
                Link::Nothing => {
                    None
                }
            }
        }

        pub fn drop(&mut self) {

            /* get the value of the tail node and replace it by Nothing at self.tail */
            let mut current = mem::replace(&mut self.tail, Link::Nothing);

            /* we get the wrapped node for every iteration;
               create a scope for the wrapped node to delete */
            while let Link::Something(mut to_delete) = current {

                /* the current value used for iteration now contains
                   the next node of the current node to delete;
                   the next parameter of the current node to delete
                   is now equal to Nothing */
                current = mem::replace(&mut to_delete.next, Link::Nothing);

                /* to_delete goes out of the scope and is deleted */
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_create() {

        use ll;

        let mut list = ll::LinkedList::new();
        list.insert(10);
        list.insert(20);
        list.insert(30);
        list.insert(40);

        assert_eq!(
            list.pop(),
            Some(40),
            "40 expected !"
        );

        assert_eq!(
            list.pop(),
            Some(30),
            "30 expected !"
        );

        list.drop();

        assert_eq!(
            list.pop(),
            None,
            "None is expected !"
        );
    }
}
