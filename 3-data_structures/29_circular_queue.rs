struct CircularQueue {
    buffer: Vec<Option<i32>>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

impl CircularQueue {
    fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![None; capacity],
            head: 0,
            tail: 0,
            size: 0,
            capacity,
        }
    }

    fn enqueue(&mut self, val: i32) -> bool {
        if self.size == self.capacity {
            return false;
        }
        self.buffer[self.tail] = Some(val);
        self.tail = (self.tail + 1) % self.capacity;
        self.size += 1;
        true
    }

    fn dequeue(&mut self) -> Option<i32> {
        if self.size == 0 {
            return None;
        }
        let val = self.buffer[self.head].take();
        self.head = (self.head + 1) % self.capacity;
        self.size -= 1;
        val
    }
}

fn main() {
    let mut cq = CircularQueue::new(3);
    cq.enqueue(1);
    cq.enqueue(2);
    cq.enqueue(3);
    println!("CircularQueue Dequeue: {:?}", cq.dequeue());
}
