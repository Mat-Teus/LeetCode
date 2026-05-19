struct MinStack {
    stack: Vec<i32>,
    stack_min: Vec<i32>,
}

impl MinStack {

    fn new() -> Self {
        Self{
            stack: Vec::new(),
            stack_min: Vec::new(),
        }
    }
    
    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.stack_min.is_empty() || *self.stack_min.last().unwrap() >= val{
            self.stack_min.push(val);
        }
    }
    
    fn pop(&mut self) {
        if *self.stack.last().unwrap() == *self.stack_min.last().unwrap(){
            self.stack_min.pop();
        }
        self.stack.pop();
    }
    
    fn top(&self) -> i32 {
        return *self.stack.last().unwrap();
    }
    
    fn get_min(&self) -> i32 {
        return *self.stack_min.last().unwrap()
    }
}

fn main(){
    let mut MinStack = MinStack::new();
    MinStack.push(-2);
    MinStack.push(0);
    MinStack.push(-3);
    println!("{}", MinStack.get_min());
    println!("{}", MinStack.top());
}