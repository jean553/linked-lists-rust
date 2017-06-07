use std::mem;

pub struct LinkedList {
    tail: Link
}

struct Node {
    data: u32,
    next: Link,
}

type Link = Option<Box<Node>>;

impl LinkedList {

    pub fn new() -> Self {
        LinkedList {
            tail: None,
        }
    }

    pub fn insert(
        &mut self,
        data: u32
    ) {

        let node = Box::new(Node {
            data: data,
            next: self.tail.take()
        });

        self.tail = Some(node);
    }

    pub fn pop(&mut self) -> Option<u32> {

        self.tail.take().map(|value| {
            let node = *value;
            self.tail = node.next;
            node.data
        })
    }

    pub fn drop(&mut self) {

        /* get the value of the tail node and replace it by None at self.tail */
        let mut current = self.tail.take();

        /* we get the wrapped node for every iteration;
           create a scope for the wrapped node to delete */
        while let Some(mut to_delete) = current {

            /* the current value used for iteration now contains
               the next node of the current node to delete;
               the next parameter of the current node to delete
               is now equal to None */
            current = to_delete.next.take();

            /* to_delete goes out of the scope and is deleted */
        }
    }
}

#[cfg(test)]
mod tests {

    use super::LinkedList;

    #[test]
    fn test_create() {

        let mut list = LinkedList::new();
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
