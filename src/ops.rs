use stack::Stack;

/// Sum sums the two top most values on the stack and
/// inserts the result onto the stack.
pub fn Sum(mut s: Stack<f32>) -> Stack<f32> {
    let mut a :f32 = 0.0;
    {
        let (ss, input_a) = s.pop();
        match input_a {
            Some(val) => a = val,
            None => return ss,
        }
        s = ss;
    }
    let mut b :f32 = 0.0;
    {
        let (ss, input_b) = s.pop();
        match input_b {
            Some(val) => b = val,
            None => return ss,
        }
        s = ss;
    }
    s.push(a+b)
}
