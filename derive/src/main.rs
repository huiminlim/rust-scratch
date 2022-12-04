#[allow(dead_code)]
// Derive debug printing functionality
#[derive(Debug, Clone, Copy)] // Ownership of object is not changed, as clone/copy is used
enum Position {
    Manager,
    Supervisor,
    Worker,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)] // Ownership of object is not changed, as clone/copy is used
struct Employee {
    position: Position,
    work_hours: i32,
}

fn main() {
    let me = Employee {
        position: Position::Manager,
        work_hours: 40,
    };
    println!("{:?}", me);
}
