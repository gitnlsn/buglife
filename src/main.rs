use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;
use std::io;

#[derive(Copy, Clone)]
enum Mark {
    M, /* male mark */
    F, /* female mark */
}

impl fmt::Display for Mark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mark::M => write!(f, "male"),
            Mark::F => write!(f, "female")
        }
    }
}

impl Mark {
    fn equals(self, other: Mark) -> bool {
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

    fn clear_relations(&mut self) {
        self.relations = Vec::new();
    }

    fn set_tag(&mut self, tag: Mark) -> &Bug {
        self.tag = Some(Box::new(tag));
        return self;
    }

    fn check(&mut self, tag: Mark) -> bool {
        match &self.tag {
            Some(existing_tag) => return existing_tag.equals(tag),
            None => return false,
        }
    }
    
    fn get_tag(&mut self) -> String {
        match &self.tag {
            Some(existing_tag) => return existing_tag.to_string(),
            None => return "undefined".to_string(),
        }
    }

    fn has_mark(&mut self) -> bool {
        match &self.tag {
            Some(_) => return true,
            None => return false,
        }
    }

    fn add_relation(&mut self, bug: &Rc<RefCell<Bug>>) -> usize {
        self.relations.push(Rc::clone(bug));
        return self.relations.len();
    }

    fn has_relation(&mut self, bug: &Rc<RefCell<Bug>>) -> bool {
        for related_bug in self.relations.iter() {
            if related_bug.borrow().id == bug.borrow().id {
                return true;
            }
        }
        return false;
    }
}

struct Colony {
    bugs: Vec<Rc<RefCell<Bug>>>,
}

impl Colony {
    pub fn new(mut size: usize) -> Colony {
        let mut this = Colony { bugs: Vec::new() };
        while size > 0 {
            this.new_bug();
            size = size - 1;
        }
        return this;
    }

    fn new_bug(&mut self) -> usize {
        let bug_id = self.bugs.len();
        let bug = Bug::new(bug_id);
        let bug_ref = Rc::new(RefCell::new(bug));
        self.bugs.push(bug_ref);
        return self.bugs.len();
    }

    fn add_relation(&mut self, a_id: usize, b_id: usize) -> (usize, usize) {
        let bug_a = self.bugs.get(a_id).unwrap();
        let bug_b = self.bugs.get(b_id).unwrap();

        let a_relations_size = bug_a.borrow_mut().add_relation(&bug_b);
        let b_relations_size = bug_b.borrow_mut().add_relation(&bug_a);

        return (a_relations_size, b_relations_size);
    }

    fn size(&mut self) -> usize {
        self.bugs.len()
    }

    fn clear_tags(&mut self) {
        for index in 0..self.bugs.len() {
            self.bugs.get(index).unwrap().borrow_mut().clear();
        }
    }
    
    fn is_suspicious(&mut self) -> bool {
        let colony_size = self.size();
        for index in 0..colony_size {
            self.clear_tags();
            if !self.is_consistent(index, Mark::M) {
                return false;
            }
        }
        return true;
    }
    
    fn is_consistent(&mut self, bug_id: usize, expected_mark: Mark) -> bool {
        let bug = self.bugs.get(bug_id).unwrap();
        let bug_is_marked = bug.borrow_mut().has_mark();
        
        if !bug_is_marked {
            bug.borrow_mut().set_tag(expected_mark);
    
            let related_bug_id_list: Vec<usize> = bug
                .borrow()
                .relations
                .iter()
                .map(|related_bug| related_bug.borrow().id)
                .collect();
    
            for related_bug_id in related_bug_id_list {
                if !self.is_consistent (
                    related_bug_id,
                    opposed_mark(expected_mark),
                ) {
                    return false;
                }
            }
            return true;
        }
        return bug.borrow_mut().check(expected_mark);
    }
}


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

    assert!(!a.has_mark());
    assert!(!a.check(Mark::M));
    assert!(!a.check(Mark::F));

    a.set_tag(Mark::M);
    assert!(a.has_mark());
    assert!(a.check(Mark::M));
    assert!(!a.check(Mark::F));

    a.set_tag(Mark::F);
    assert!(a.has_mark());
    assert!(!a.check(Mark::M));
    assert!(a.check(Mark::F));

    a.clear();
    assert!(!a.has_mark());
    assert!(!a.check(Mark::M));
    assert!(!a.check(Mark::F));
}

#[test]
fn test_bug_add_relation() {
    let bug_0 = Rc::new(RefCell::new(Bug::new(0)));
    let bug_1 = Rc::new(RefCell::new(Bug::new(1)));
    let bug_2 = Rc::new(RefCell::new(Bug::new(2)));
    let bug_3 = Rc::new(RefCell::new(Bug::new(3)));

    let a_relations_size = bug_0.borrow_mut().add_relation(&bug_1);
    let a_relations_size = bug_0.borrow_mut().add_relation(&bug_2);
    let b_relations_size = bug_1.borrow_mut().add_relation(&bug_2);
    let c_relations_size = bug_2.borrow_mut().add_relation(&bug_0);

    assert!(a_relations_size == 2);
    assert!(b_relations_size == 1);
    assert!(c_relations_size == 1);

    assert!(bug_0.borrow_mut().has_relation(&bug_1));
    assert!(bug_0.borrow_mut().has_relation(&bug_2));
    assert!(bug_1.borrow_mut().has_relation(&bug_2));
    assert!(bug_2.borrow_mut().has_relation(&bug_0));

    assert!(!bug_2.borrow_mut().has_relation(&bug_1));
    assert!(!bug_1.borrow_mut().has_relation(&bug_0));
}

#[test]
fn test_colony_size() {
    let mut colony = Colony::new(1);
    assert!(colony.size() == 1);

    let mut colony = Colony::new(2);
    assert!(colony.size() == 2);
}

#[test]
fn test_colony_relations() {
    let mut colony = Colony::new(3);

    colony.add_relation(0, 1);
    colony.add_relation(2, 1);

    let bug_0 = colony.bugs.get(0).unwrap();
    let bug_1 = colony.bugs.get(1).unwrap();
    let bug_2 = colony.bugs.get(2).unwrap();

    assert!(bug_0.borrow_mut().has_relation(&bug_1));
    assert!(bug_1.borrow_mut().has_relation(&bug_0));

    assert!(!bug_0.borrow_mut().has_relation(&bug_2));
    assert!(!bug_2.borrow_mut().has_relation(&bug_0));

    assert!(bug_2.borrow_mut().has_relation(&bug_1));
    assert!(bug_1.borrow_mut().has_relation(&bug_2));
    
    let mut colony = Colony::new(4);
    
    colony.add_relation(0, 1);
    colony.add_relation(1, 2);
    colony.add_relation(2, 3);
    colony.add_relation(3, 0);
    
    let bug_0 = colony.bugs.get(0).unwrap();
    let bug_1 = colony.bugs.get(1).unwrap();
    let bug_2 = colony.bugs.get(2).unwrap();
    let bug_3 = colony.bugs.get(3).unwrap();
    
    assert!(bug_0.borrow_mut().has_relation(&bug_1));
    assert!(bug_1.borrow_mut().has_relation(&bug_2));
    assert!(bug_2.borrow_mut().has_relation(&bug_3));
    assert!(bug_3.borrow_mut().has_relation(&bug_0));
    assert!(!bug_0.borrow_mut().has_relation(&bug_2));
    assert!(!bug_1.borrow_mut().has_relation(&bug_3));
}

#[test]
fn test_inspect_small_cases() {
    let mut colony = Colony::new(2);
    colony.add_relation(0, 1);
    assert!(colony.is_suspicious());
    
    let mut colony = Colony::new(3);
    colony.add_relation(0, 1);
    colony.add_relation(1, 2);
    colony.add_relation(2, 0);
    assert!(!colony.is_suspicious());
    
    let mut colony = Colony::new(4);
    colony.add_relation(0, 1);
    colony.add_relation(1, 2);
    colony.add_relation(2, 3);
    colony.add_relation(3, 0);
    assert!(colony.is_suspicious());
}

fn main() {
    let total_tests: usize = get_input().parse().expect("Failed to parse integer");
    for index in 0..total_tests {
        
        let (colony_size, interations_length) = read_tuple();
        
        let mut colony = Colony::new(colony_size);
        
        for _ in 0..interations_length {
            let (bug_a, bug_b) = read_tuple();
            colony.add_relation(bug_a - 1, bug_b - 1);
        }
        
        println!("Scenario #{}", index + 1);
        if colony.is_suspicious() {
            println!("Suspicious bugs found!");
        } else {
            println!("No suspicious bugs found!");
        }
    }
}
