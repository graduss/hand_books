pub mod run;
use std::fmt;

#[derive(Debug)]
pub enum SinglyLinkedListError {
    IndexOutOfBounds { index: usize, len: usize },
}

impl fmt::Display for SinglyLinkedListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IndexOutOfBounds { index, len } => {
                write!(f, "Index {index} out of bounds the list (len: {len})")
            }
        }
    }
}

impl std::error::Error for SinglyLinkedListError {}

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Self>>,
}

#[derive(Debug)]
pub struct SinglyLinkedList<T> {
    root: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }

    pub fn insert(&mut self, ind: usize, value: T) -> Result<(), SinglyLinkedListError> {
        if ind == 0 {
            self.root = Some(Box::new(Node {
                value,
                next: self.root.take(),
            }))
        } else {
            let len = self.len;
            let node = self
                .get_node_mut(ind - 1)
                .ok_or_else(|| SinglyLinkedListError::IndexOutOfBounds { index: ind, len })?;
            node.next = Some(Box::new(Node {
                value,
                next: node.next.take(),
            }))
        }

        self.len += 1;

        Ok(())
    }

    pub fn remove(&mut self, ind: usize) -> Result<T, SinglyLinkedListError> {
        let len = self.len;
        let err = || SinglyLinkedListError::IndexOutOfBounds { index: ind, len };

        let removed = if ind == 0 {
            let mut removed = self.root.take().ok_or_else(err)?;
            self.root = removed.next.take();
            removed
        } else {
            let previos_node = self.get_node_mut(ind - 1).ok_or_else(err)?;
            let mut removed = previos_node.next.take().ok_or_else(err)?;
            previos_node.next = removed.next.take();
            removed
        };

        self.len -= 1;

        Ok(removed.value)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, ind: usize) -> Option<&T> {
        self.get_node(ind).map(|n| &n.value)
    }

    fn get_node(&self, ind: usize) -> Option<&Node<T>> {
        let mut node = self.root.as_deref()?;
        for _ in 0..ind {
            node = node.next.as_deref()?;
        }
        Some(node)
    }

    fn get_node_mut(&mut self, ind: usize) -> Option<&mut Node<T>> {
        let mut node = self.root.as_deref_mut()?;
        for _ in 0..ind {
            node = node.next.as_deref_mut()?;
        }
        Some(node)
    }
}
