// parse collada file format

use std::mem;
use std::ptr;
use std::io::{BufferedReader, File};
use std::slice::{Found, NotFound};

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

	fn push_back_node(&mut self, mut new_tail: Box<Node<T>>) {
		match self.tail.resolve() {
			None => {
				self.tail = RawLink::some(&mut *new_tail);
				self.head = Some(new_tail);
			}
			Some(tail) => {
				self.tail = RawLink::some(&mut *new_tail);
				tail.next = link_pre(new_tail, RawLink::some(tail));
			}
		}
		self.length += 1;
	}

	pub fn push_back(&mut self, elt: T) {
		self.push_back_node(box Node::new(elt))
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
	#[inline]
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

fn main() {
	// read line-by-line, parse library_* modules
	let path = Path::new("test.cs");
    let mut v = Vec::new();
    for line in BufferedReader::new(File::open(&path)).lines().filter_map(|result| result.ok()) {
        v.push(line.trim_chars(' ').to_string());
    }

    for ln in v.iter() {
        print!("{}",ln)
    }

    let seek = "//\n";
	let resultIndex = match v.as_slice().binary_search(|probe| probe.cmp(&seek.to_string())) {
		Found(index) => println!("find: {}", index),
		NotFound(err) => println!("not found: {}",err),
	};
    
    // println!("{}",v);

	let mut dlist: XmlList<uint> = XmlList::new();

	for i in range(0u, 10u) {
		println!("add: {}",i);
		dlist.push_back(i);
	}

	for it in dlist.iter() {
		println!("result: {}", it);
	}
}