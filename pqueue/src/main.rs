// TODO make item store generic types?
struct Item {
    priority: u32,
    item: u32
}

struct PriorityQueue {
    heap: Vec<Item>
}

impl PriorityQueue {
    fn insert(mut &self, &item: Item) -> bool {
        return false;
    }

    fn pop(&self) -> &Item {
        return Item{};
    }

    fn deleteHighestPriority(mut &self) -> &Item {
        return Item{};
    }
    }

    fn decreaseKey(mut &self, &id: u32, &new_priority: u32) -> bool {
        return false;
    }

    fn increaseKey(mut &self, &id: u32, &new_priority: u32) -> bool {
        return false;
    }

    fn empty(&self) -> bool {
        self.heap.size() == 0
    }
}

fn main() {
    println!("Hello, world!");
}
