// parse collada file format

use std::mem;
use std::ptr;
// use core::default::Default;

type Link<T> = Option<Box<Node<T>>>;

struct RawLink<T> {
    p: *mut T
}

struct XmlList<T> {
    head: Link<T>,
    tail: RawLink<Node<T>>,
}

struct Node<T> {
    prev: RawLink<Node<T>>,
    next: Link<T>,
    data: T,
}

impl<T> RawLink<T> {
	pub fn none() -> RawLink<T> {
		RawLink{ p: ptr::null_mut() }
	}
	pub fn some(n: &mut T) -> RawLink<T>{
		RawLink{ p: n}
	}
}

impl<T> Node<T> {
	pub fn new(v: T) -> Node<T> {
		Node{prev: RawLink::none(), next: None, data: v}
	}
}

impl<T> XmlList<T> {
	pub fn new() -> XmlList<T> {
		XmlList{head: None, tail: RawLink::none()}
	}
	pub fn push_front_node(&mut self, mut new_head: Box<Node<T>>) {
		match self.head {
			None => {
				self.tail = RawLink::some(&mut *new_head);
				new_head.prev = RawLink::none();
				self.head = Some(new_head);
			}
			Some(ref mut head) => {
				new_head.prev = RawLink::none();
				head.prev = RawLink::some(&mut *new_head);
				mem::swap(head, &mut new_head); // let the head point to new_head
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
	let mut dlist: XmlList<String> = XmlList::new();
	let node0: Box<Node<String>> = box Node::new("Lily Chen".to_string());
	let node1: Box<Node<String>> = box Node::new("tracy ma".to_string());
	dlist.push_front_node(node0);
	dlist.push_front_node(node1);
}