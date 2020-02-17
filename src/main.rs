use std::cell::RefCell;
use std::rc::Rc;
use std::io;

#[derive(Copy, Clone)]
enum Mark {
    M, /* male mark */
    F, /* female mark */
}

impl Mark {
    fn equals (self, other: Mark) -> bool {
        match (self, other) {
            (Mark::M, Mark::M) => return true,
            (Mark::F, Mark::F) => return true,
            _ => return false,
        };
    }
}

struct Bug {
    id: usize,
    tag: Option<Box<Mark>>,
    relations: Vec<Rc<RefCell<Bug>>>,
}

impl Bug {
    pub fn new(id: usize) -> Bug {
        Bug {
            id: id,
            tag: None,
            relations: Vec::new(),
        }
    }

    fn clear(&mut self) {
        self.tag = None;
    }

    fn set_tag(&mut self, tag: Mark) -> &Bug {
        self.tag = Some(Box::new(tag));
        return self;
    }

    fn check(&mut self, tag: Mark) -> bool {
        match &self.tag {
            Some(existingTag) => return existingTag.equals(tag),
            None => return false,
        }
    }

    fn hasMark(&mut self) -> bool {
        match &self.tag {
            Some(existingTag) => return true,
            None => return false,
        }
    }

    fn add_relation(&mut self, bug: &Rc<RefCell<Bug>>) -> usize {
        self.relations.push(Rc::clone(bug));
        return self.relations.len();
    }
}

// struct Colony {
//     bugs: Vec<Rc<Bug>>,
// }

// impl Colony {
//     pub fn new(mut size: usize) -> Colony {
//         let mut this = Colony { bugs: Vec::new() };
//         while size > 0 {
//             this.new_bug();
//             size = size - 1;
//         }
//         return this;
//     }

//     fn new_bug(&mut self) -> usize {
//         let bug = Bug::new(self.bugs.len());
//         self.bugs.push(bug);
//         return self.bugs.len();
//     }

//     fn add_relation(&mut self, a_id: usize, b_id: usize) -> (usize, usize) {
//         let a_relations_size = self.bugs.get_mut(a_id).unwrap().add_relation(b_id);

//         let b_relations_size = self.bugs.get_mut(b_id).unwrap().add_relation(a_id);

//         return (a_relations_size, b_relations_size);
//     }

//     fn size(&mut self) -> usize {
//         self.bugs.len()
//     }

//     fn clear_tags(&mut self) {
//         for index in 0..self.bugs.len() {
//             self.bugs[index].clear();
//         }
//     }
// }

// fn inspect(colony: &mut Colony, bug_id: usize, expected: Mark) -> bool {
//     let bug = colony.bugs.get_mut(bug_id).unwrap();
//     if !bug.hasMark() {
//         bug.set_tag(expected);
//         for &related_bug_index in bug.relations.values() {
//             if inspect(colony, related_bug_index, opposed_mark(expected)) {
//                 return true;
//             }
//         }
//         return false;
//     }
//     return bug.check(expected);
// }

fn opposed_mark(tag: Mark) -> Mark {
    match tag {
        Mark::M => return Mark::F,
        Mark::F => return Mark::M,
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
fn test_opposed_mark() {
    assert!(opposed_mark(Mark::M).equals(Mark::F));
    assert!(opposed_mark(Mark::F).equals(Mark::M));
}

#[test]
fn test_bug_set_check() {
    let mut a = Bug::new(0);

    assert!(!a.hasMark());
    assert!(!a.check(Mark::M));
    assert!(!a.check(Mark::F));
    
    a.set_tag(Mark::M);
    assert!(a.hasMark());
    assert!(a.check(Mark::M));
    assert!(!a.check(Mark::F));

    a.set_tag(Mark::F);
    assert!(a.hasMark());
    assert!(!a.check(Mark::M));
    assert!(a.check(Mark::F));

    a.clear();
    assert!(!a.hasMark());
    assert!(!a.check(Mark::M));
    assert!(!a.check(Mark::F));
}

#[test]
fn test_bug_add_relation() {
    let mut bug_0 = Rc::new(RefCell::new(Bug::new(0)));
    let mut bug_1 = Rc::new(RefCell::new(Bug::new(1)));
    let mut bug_2 = Rc::new(RefCell::new(Bug::new(2)));
    let mut bug_3 = Rc::new(RefCell::new(Bug::new(3)));
    
    let a_relations_size = bug_0.borrow_mut().add_relation(&bug_1);
    let a_relations_size = bug_0.borrow_mut().add_relation(&bug_2);
    let b_relations_size = bug_1.borrow_mut().add_relation(&bug_2);
    let c_relations_size = bug_2.borrow_mut().add_relation(&bug_0);
    
    assert!(a_relations_size == 2);
    assert!(b_relations_size == 1);
    assert!(c_relations_size == 1);
}

// #[test]
// fn test_colony_index() {
//     let mut colony = Colony::new(3);
//     let bug = colony.bugs.get_mut(0).unwrap();
//     assert!(bug.id == 0);
// }

#[test]
fn foo() {}

fn main() {
    let total_tests: u32 = get_input().parse().expect("Failed to parse integer");
    for index in 0..total_tests {}
}
