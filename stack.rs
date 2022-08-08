#[derive(Debug)]
struct Stack<T>{
    stack: Vec<T>
}

//impl: implements method on described struct
// here the arg self means it points to the Struct and `&` means it can borrow or not
//there is a difference between Self and self
impl<T> Stack<T> {
    fn new() -> Self{
        Stack { stack: Vec::new() }
    }   

    fn push(&mut self, item: T){
        self.stack.push(item)
    }

    fn pop(&mut self) -> Option<T>{
        self.stack.pop()
    }

    //Vector has lib method is_empty()
    fn isEmpty(&self) -> bool {
        self.stack.is_empty()
    }

    //Vectory has lib method len()
    fn length(&self) -> usize{
        self.stack.len()
    }

    fn peek(&self) -> Option<&T>{
        self.stack.last()
    }
}

fn main(){
    let mut _stack: Stack<String> = Stack::new();
    _stack.push("parit1".to_string());
    _stack.push("parit2".to_string());
    _stack.push("parit3".to_string());
    _stack.push("parit4".to_string());
    println!("{:?}",_stack);

    let mut _stack2: Stack<isize> = Stack::new();
    _stack2.push(1);
    _stack2.push(2);
    _stack2.push(3);
    _stack2.push(4);
    println!("{:?}",_stack2);
}