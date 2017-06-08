use std::mem;

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {

    pub fn new() -> Self {
        LinkedList {
            head: None,
        }
    }

    pub fn peek(&self) -> Option<&T> {

        /* without as_ref(), the self.head value is taken;
           so return a reference is impossible because the reference
           would concern the taken value, which does no longer exist
           when the function is terminated; as_ref() let us use
           a reference of the self.head value */
        self.head.as_ref().map( |value| {
            &value.data
        })
    }

    pub fn insert(
        &mut self,
        data: T
    ) {

        let node = Box::new(Node {
            data: data,
            next: self.head.take()
        });

        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {

        self.head.take().map(|value| {
            let node = *value;
            self.head = node.next;
            node.data
        })
    }

    pub fn drop(&mut self) {

        /* get the value of the head node and replace it by None at self.head */
        let mut current = self.head.take();

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

        list.insert(100);
        list.insert(200);

        assert_eq!(
            list.peek(),
            Some(&200),
            "200 is expected"
        );
    }
}
