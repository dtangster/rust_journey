use std::fmt;

#[derive(Debug)]
struct Person {
    name: String,
}

impl Person {
    fn new(name: String) -> Self {
        Self {
            name
        }
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn func1(p: Person) {
    func2(p);
    // Can't use p here anymore because func2 now owns p
}

fn func2(p: Person) {
    func3(p);
    // Can't use p here anymore because func3 now owns p
}

fn func3(p: Person) {
    dbg!(p);
}

fn func1ref(p: &Person) {
    // We can't derefence here to explicitly take ownership.
    // The compiler will complain if you uncomment the following line.
    // let p2 = *p;

    // The following assignment doesn't cause p2 to take ownership of p
    // because it is a reference.
    let p2 = p;

    // Note how we can still use p even though we have the p2=p1 assignment
    // from the previous line.
    func2ref(p);

    dbg!("END func1ref {}", p);
}

fn func2ref(p: &Person) {
    func3ref(p);
    dbg!("END func2ref {}", p);
}

fn func3ref(p: &Person) {
    dbg!("END func3ref {}", p);
}

fn main() {
    let p = Person::new("david".to_string());
    println!("{}", p); // This requires implementing the fmt::Display trait
    func1ref(&p);
    func1(p);
}
