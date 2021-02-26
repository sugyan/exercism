use std::iter::FromIterator;

type Node<T> = Option<Box<ListNode<T>>>;

#[derive(Default)]
pub struct SimpleLinkedList<T> {
    head: Node<T>,
    len: usize,
}

struct ListNode<T> {
    data: T,
    next: Node<T>,
}

impl<T: Copy> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(ListNode {
            data: element,
            next: self.head.take(),
        }));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(n) = self.head.take() {
            self.head = n.next;
            self.len -= 1;
            return Some(n.data);
        }
        None
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(n) = self.head.as_ref() {
            return Some(&n.data);
        }
        None
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let v: Vec<_> = self.into();
        v.into_iter().rev().collect()
    }
}

impl<T: Copy> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for item in iter.into_iter() {
            list.push(item);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T: Copy> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vec = Vec::new();
        let mut node = &self.head;
        while let Some(n) = node {
            vec.push(n.data);
            node = &n.next;
        }
        vec.reverse();
        vec
    }
}
