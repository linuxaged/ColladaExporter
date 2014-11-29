use std::io::File;
use std::cmp;

struct BoyerMoore {
    pat: Vec<u8>,
    source: Vec<u8>,
    delta1: [int, ..256],
    delta2: Vec<int>,
}

impl BoyerMoore {
    fn new(content: &str, target: &str) -> BoyerMoore {
        let pat = target.to_string().into_bytes();
        let source = content.to_string().into_bytes();
        let delta1 = BoyerMoore::make_delta1(pat.as_slice());
        let delta2 = BoyerMoore::make_kmp(pat.as_slice());
        BoyerMoore {
            pat: pat,
            source: source,
            delta1: delta1,
            delta2: delta2,
        }
    }

    fn make_delta1(pat: &[u8]) -> [int, ..256] {
        let mut delta1 = [pat.len() as int, ..256];

        for i in range(0,pat.len()) {
            delta1[pat[i] as uint] = (pat.len() -i -1) as int;
        }

        delta1
    }

    fn make_kmp(pat: &[u8]) -> Vec<int> {
        let mut kmp: Vec<int> = Vec::from_elem(pat.len(), 0);
        kmp[0] = -1;
        if pat.len() > 2 {
            let mut index = 0;
            for i in range (3, pat.len()) {
                if pat[i] == pat[index as uint] {
                    index = index + 1; // todo: can be out of range
                    kmp[i] = index;
                } else {
                    index = 0;
                }
            }
        }
        kmp
    }

    fn search(&self) -> Option<uint> {
        if self.pat.len() == 0 {
            return None
        }
        let mut i = self.pat.len() - 1;
        while i < self.source.len() {
            let mut j = (self.pat.len() - 1) as int;
            while (j >= 0) && (self.source[i] == self.pat[j as uint]) {
                i = i-1;
                j = j-1;
            }


            if j < 0 {
                return Some(i + j as uint + 1);
            }
            i += cmp::max(self.delta1[self.source[i] as uint], self.delta2[j as uint]) as uint;
        }
        None
    }
}

#[test]
fn test_make_delta1() {
    let delta1 = BoyerMoore::make_delta1("EXAMPLE".to_string().into_bytes().as_slice());
    for d in range(0, delta1.len()) {
        println!("[{}],{}", d, delta1[d]);
    }
}

#[test]
fn test_make_kmp() {
    let delta2 = BoyerMoore::make_kmp("ABCDABCEABCDABC".to_string().into_bytes().as_slice());
    for d in range(0, delta2.len()) {
        println!("[{}],{}", d, delta2[d]);
    }
}

fn main() {
    let path = Path::new("/tmp/data.txt");
    let raw_string = File::open(&path).read_to_string().unwrap();
    let result = BoyerMoore::new(raw_string.as_slice(), "SIMPLE").search();
    println!("{}", result);
}