struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,  // The problem is that at compile time the size of next must be known. Since next is recursive ("a node has a node has a node..."), the compiler does not know how much memory is to be allocated. In contrast, Box is a heap pointer with a defined size.
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: Option::None
        }
    }

    pub fn len(&self) -> usize {
        let mut counter = 0;
        let mut optional = &self.head;
        while let Some(b) = optional { // while this pattern matches to this optional
            counter = counter + 1;
            optional = &b.next;
        }
        counter
    }

    pub fn push(&mut self, _element: T) {
        self.head = Option::Some(Box::from(Node {
            data: _element,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        // ~~~~~ Alternative implementation ~~~~
        
        // match self.head.take() {
        //     Some(b) => {
        //         self.head = b.next;
        //         Some(b.data)
        //     },
        //     None => None,
        // }
        if let Some(b) = self.head.take() {
            self.head = b.next;
            Some(b.data)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(b) = &self.head {
            Some(&b.data)
        } else {
            None
        }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut optional = &self.head;
        let mut reverse_linked_list: SimpleLinkedList<T> = SimpleLinkedList { 
            head: Option::Some(Box::from(Node {
                data: self.peek().cloned().unwrap(),
                next: None
            }))
        };
        while let Some(b) = optional {
            if let Some(b) = &b.next {
                reverse_linked_list.push(b.data.clone())
            }
            optional = &b.next;
        }
        reverse_linked_list
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_item: &[T]) -> Self {
        let mut linked_list = SimpleLinkedList::new();
        for entry in _item.into_iter() {
            linked_list.push(entry.clone());
        }
        linked_list
    }
}

impl<T: Clone> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut new_vector: Vec<T> = vec!();
        let mut optional = &self.head;
        while let Some(b) = optional {
            new_vector.push(b.data.clone());
            optional = &b.next;
        }
        new_vector.reverse();
        new_vector
    }
}
