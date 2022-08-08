#[derive(Debug)]

struct Queue<T>{
    queue: Vec<T>
}

impl<T> Queue<T>{
    fn CreateNew() -> Self{
        Queue { queue: Vec::new() }
    }  
    fn enqueue(&mut self, item: T){
        self.queue.push(item)
    }
    fn dequeue(&mut self) -> T{
        self.queue.remove(0)
    }
    fn length(&self) -> usize{
        self.queue.len()
    }
    fn isEmpty(&self) -> bool{
        self.queue.is_empty()
    }
    fn peek(&self) -> Option<&T>{
        self.queue.first()
    }
}
 fn main(){
    let mut _queue: Queue<usize> = Queue::CreateNew();
    _queue.enqueue(1);
    _queue.enqueue(3);
    _queue.enqueue(4);
    println!("{:?}",_queue);
 }