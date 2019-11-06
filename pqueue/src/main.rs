// TODO make item store generic types?
struct Item {
    priority: u32,
    item: u32
}

struct PriorityQueue {
    heap: Vec<Item>
}

impl PriorityQueue {
    fn insert(&mut self, item: &Item) -> bool {
        return false;
    }

    fn pop(&self) -> &Item {
        return &Item{priority: 0, item: 0};
    }

    fn deleteHighestPriority(&mut self) -> Item {
        return Item{priority: 0, item: 0};
    }

    fn decreaseKey(&mut self, id: u32, new_priority: u32) -> bool {
        return false;
    }

    fn increaseKey(&mut self, id: u32, new_priority: u32) -> bool {
        return false;
    }

    fn empty(&self) -> bool {
        self.heap.len() == 0
    }
}

fn main() {
    let mut values : Vec<Item>;
    println!("Hello, world!");
}
