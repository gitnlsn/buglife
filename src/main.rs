
use std::io;

enum Mark {
    N, /* no mark */
    M, /* male mark */
    F  /* female mark */
}

struct Bug {
    related: Vec<Bug>,
    tag: Mark
}

impl Bug {
    pub fn new() -> Bug {
        Bug {
            related: Vec::new(),
            tag: Mark::N,
        }
    }
    
    fn touch(&mut self, bug: Bug) {
        self.related.push(bug);
    }
    
    fn iter(&mut self) -> impl Iterator<Item = &Bug>{
        self.related.iter()
    }
    
    fn clear(&mut self) {
        self.tag = Mark::N;
    }
    
    fn set(&mut self, tag: Mark) -> &Bug {
        self.tag = tag;
        return self;
    }
    
    fn check(&mut self, tag: Mark) -> bool {
        let this_mark = &self.tag;
        match (this_mark, tag) {
            (Mark::M, Mark::M) => return true,
            (Mark::F, Mark::F) => return true,
            (Mark::N, Mark::N) => return true,
            _ => return false,
        };
    }
}

struct Colony {
    bugs: Vec<Bug>,
}

impl Colony {
    pub fn new(size: usize) -> Colony {
        Colony {
            bugs: Vec::new(),
        }
    }
}

fn get_input() -> String {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed at read_line");
    return input.trim().parse().expect("Failed to parse");
}

fn read_tuple() -> (usize, usize) {
    let input: Vec<usize> = get_input()
        .split(" ")
        .map(|x| x.parse().expect("Failed to parse interger"))
        .collect();

    return (input[0], input[1]);
}

#[test]
fn test_bug_set_check() {
    let mut a = Bug::new();
    
    a.set(Mark::M);
    assert!( a.check(Mark::M));
    assert!(!a.check(Mark::F));
    assert!(!a.check(Mark::N));
    
    a.set(Mark::F);
    assert!(!a.check(Mark::M));
    assert!( a.check(Mark::F));
    assert!(!a.check(Mark::N));
    
    a.clear();
    assert!(!a.check(Mark::M));
    assert!(!a.check(Mark::F));
    assert!( a.check(Mark::N));
}

#[test]
fn test_bug_touch() {}

#[test]
fn foo() {}

fn main() {
    let total_tests: u32 = get_input().parse().expect("Failed to parse integer");
    for index in 0..total_tests {
        
    }
}
