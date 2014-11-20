// parse collada file format

use std::mem;
use std::ptr;

type NNode<T> = Option<Box<Node<T>>>; // normal node
type PNode<T> = RawLink<Node<T>>; // pointer node

struct RawLink<T> {
    p: *mut T
}

pub struct XmlList<T> {
    head: NNode<T>,
    tail: PNode<T>,
    length: uint,
}

pub struct Node<T> {
	child: NNode<T>,
    prev: PNode<T>,
    next: NNode<T>,
    value: T,
}

impl<T> RawLink<T> {
	pub fn none() -> RawLink<T> {
		RawLink{ p: ptr::null_mut() }
	}
	pub fn some(n: &mut T) -> RawLink<T>{
		RawLink{ p: n}
	}
	pub fn resolve<'a>(&mut self) -> Option<&'a mut T>{
		if self.p.is_null() {
			None
		} else {
			Some(unsafe {mem::transmute(self.p) })
		}
	}
}

impl<T> Node<T> {
	pub fn new(v: T) -> Node<T> {
		Node{child: None, prev: RawLink::none(), next: None, value: v}
	}
}

fn link_pre<T>(mut new_node: Box<Node<T>>, prev: PNode<T>) -> NNode<T> {
	new_node.prev = prev;
	Some(new_node)
}

impl<T> XmlList<T> {
	pub fn new() -> XmlList<T> {
		XmlList{head: None, tail: RawLink::none(), length: 0}
	}

	pub fn push_back_node(&mut self, mut new_tail: Box<Node<T>>) {
		match self.tail.resolve() {
			None => {
				self.tail = RawLink::some(&mut *new_tail);
				self.head = Some(new_tail);
			}
			Some(tail) => {
				tail.next = link_pre(new_tail, RawLink::some(tail));
			}
		}
	}

	#[inline]
    #[unstable = "matches collection reform specification, waiting for dust to settle"]
    pub fn iter<'a>(&'a self) -> Items<'a, T> {
        Items{nelem: self.len(), head: &self.head, tail: self.tail}
    }

    #[inline]
    #[unstable = "matches collection reform specification, waiting for dust to settle"]
    pub fn len(&self) -> uint {
        self.length
    }
}

pub struct Items<'a, T:'a> {
    head: &'a NNode<T>,
    tail: PNode<T>,
    nelem: uint,
}

impl<'a, A> Iterator<&'a A> for Items<'a A>{
	fn next(&mut self) -> Option<&'a A> {
		if self.nelem == 0 {
			return None;
		}
		self.head.as_ref().map(|head| {
            self.nelem -= 1;
            self.head = &head.next;
            &head.value
        })
	}
}

#[test]
fn test_insert() {
	let mut dlist: XmlList<String> = XmlList::new();

	for _ in range(0u, 10000u) {
		let node0: Box<Node<String>> = box Node::new("Lily Chen".to_string());
		let node1: Box<Node<String>> = box Node::new("tracy ma".to_string());
		dlist.push_back_node(node0);
		dlist.push_back_node(node1);
	}

	for val in dlist.iter() {
		println!("{}", val);
	}

}

fn main() {
	let mut dlist: XmlList<String> = XmlList::new();

	for _ in range(0u, 10000u) {
		let node0: Box<Node<String>> = box Node::new("Lily Chen".to_string());
		let node1: Box<Node<String>> = box Node::new("tracy ma".to_string());
		dlist.push_back_node(node0);
		dlist.push_back_node(node1);
	}

	for it in dlist.iter() {
		println!("{}", it);
	}
}