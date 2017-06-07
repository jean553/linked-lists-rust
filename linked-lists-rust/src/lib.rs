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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
