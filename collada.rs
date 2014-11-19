// parse collada file format

use std::mem;
// use core::default::Default;

struct XmlList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
}

struct Node<T> {
    prev: Option<Box<Node<T>>>,
    next: Option<Box<Node<T>>>,
    data: T,
}

impl<T> Node<T> {
	pub fn new(v: T) -> Node<T> {
		Node{prev: None, next: None, data: v}
	}
}

impl<T> XmlList<T> {
	pub fn new() -> XmlList<T> {
		XmlList{head: None, tail: None}
	}
	pub fn push_front_node(&mut self, mut new_head: Box<Node<T>>) {
		match self.head {
			None => {
				self.tail = Some(new_head);
				new_head.prev = None;
				self.head = Some(new_head);
			}
			Some(ref mut head) => {
				new_head.prev = None;
				head.prev = Some(new_head);
				mem::swap(head, &mut new_head);
				head.next = Some(new_head);
			}
		}
	}
}

// impl Default for XmlList<T> {
//   	pub fn default () -> XmlList<T> {
//     	{XmlList::new()}
//   	}
// }

fn main() {
	let dlist: XmlList<int> = XmlList::new();
	let node: Box<Node<int>> = box Node::new(10);
	dlist.push_front_node(node);
}