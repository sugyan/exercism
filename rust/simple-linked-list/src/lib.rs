use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<ListNode<T>>>,
}

#[derive(Default)]
struct ListNode<T> {
    value: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

impl<T: Default> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        Self {
            head: Some(Box::new(ListNode::default())),
        }
    }
}

impl<T: Copy + Default> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Default::default()
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.as_ref().unwrap().next.is_none()
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut node = &self.head;
        while let Some(n) = node {
            node = &n.next;
            len += 1;
        }
        len - 1
    }

    pub fn push(&mut self, element: T) {
        let mut node = &mut self.head;
        while let Some(n) = node {
            if n.next.is_none() {
                return n.next = Some(Box::new(ListNode::new(element)));
            }
            node = &mut n.next;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut node = &mut self.head;
        while let Some(n) = node {
            if n.next.is_some() {
                if let Some(next) = n.next.as_ref() {
                    if next.next.is_none() {
                        let value = next.value;
                        n.next = None;
                        return Some(value);
                    }
                }
            }
            node = &mut n.next;
        }
        None
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        let mut node = &self.head;
        while let Some(n) = node {
            if n.next.is_none() {
                return Some(&n.value);
            }
            node = &n.next;
        }
        unreachable!()
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let v: Vec<_> = self.into();
        v.into_iter().rev().collect()
    }
}

impl<T: Copy + Default> FromIterator<T> for SimpleLinkedList<T> {
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
        let mut node = &self.head.unwrap().next;
        while let Some(n) = node {
            vec.push(n.value);
            node = &n.next;
        }
        vec
    }
}
