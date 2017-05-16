/// Stack contains a stack of elements
/// of the specified type.
pub struct Stack<T: Clone> {
    S: Option<Box<StackElement<T>>>,
}

struct StackElement<T: Clone> {
    value: T,
    next: Stack<T>,
}

impl<T> Stack<T> where T: Clone {


    /// new creates a new stack of the specified type.
    pub fn new() -> Stack<T>
        where T: Clone {
        Stack{S: None}
    }

    /// push inserts a new value onto the stack.
    pub fn push(self, elem: T) -> Stack<T>
        where T: Clone {
        Stack{
            S: Some(Box::new(
            StackElement{
                value: elem,
                next: self,
            })),
        }
    }

    /// pop removes an element from the stack and returns the rest of
    /// the stack.
    pub fn pop(self) -> (Stack<T>, Option<T>) 
        where T: Clone {
        match self {
            Stack{S: None} => (self, None),
            Stack{S: Some(b)} => {
                let el = *b;
                (el.next, Some(el.value))
            }
        }
    }

    /// peek returns the top most element on the stack without
    /// removing it.
    pub fn peek(&self) -> Option<T>
        where T: Clone {
        
        match self {
            &Stack{S: None} => None,
            &Stack{S: Some(ref b)} => {
                let v = b.value.clone();
                Some(v)
            }
        }
    }
}
