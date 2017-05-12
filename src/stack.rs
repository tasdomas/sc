pub const GREETING: &'static str = "well hello!";

pub struct Stack<T> {
    S: Option<Box<StackElement<T>>>,
}

struct StackElement<T> {
    value: T,
    next: Stack<T>,
}

impl<T> Stack<T> {

    pub fn new() -> Stack<T> {
        Stack{S: None}
    }

    pub fn push(self, elem: T) -> Stack<T> {
        Stack{
            S: Some(Box::new(
            StackElement{
                value: elem,
                next: self,
            })),
        }
    }

    pub fn pop(self) -> (Stack<T>, Option<T>) {
        match self {
            Stack{S: None} => (self, None),
            Stack{S: Some(b)} => {
                let el = *b;
                (el.next, Some(el.value))
            }
        }
    }
}
