use std::collections::VecDeque;

struct Queue<T> {
    elements: VecDeque<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue {
            elements: VecDeque::new(),
        }
    }
    fn enqueue(&mut self, val: T) {
        self.elements.push_back(val);
    }
    fn dequeue(&mut self) -> Option<T> {
        self.elements.pop_front()
    }
    fn peek(&self) -> Option<&T> {
        self.elements.front()
    }
}

fn main() {
    let mut queue = Queue::new();
    queue.enqueue(100);
    queue.enqueue(200);
    println!("Queue Dequeue: {:?}", queue.dequeue());
}
