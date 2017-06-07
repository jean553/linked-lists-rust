mod ll {

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
