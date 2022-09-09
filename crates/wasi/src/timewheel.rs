use std::collections::HashMap;
use std::task::Poll;
// Time wheel algorithem impl

struct Slot<T> {
    round: usize,
    t: T,
}

pub struct TimeWheel<T: Copy> {
    hashed: HashMap<usize, Vec<Slot<T>>>,
    steps: usize,
    tick: usize,
}

impl<T: Copy> TimeWheel<T> {
    // create new hashed time wheel instance
    pub fn new(steps: usize) -> Self {
        TimeWheel {
            steps: steps,
            hashed: HashMap::new(),
            tick: 0,
        }
    }

    pub fn add(&mut self, timeout: usize, value: T) {
        let slot = (timeout + self.tick) % self.steps;

        let slots = self.hashed.entry(slot).or_insert(Vec::new());

        slots.push(Slot {
            t: value,
            round: (timeout + self.tick) / self.steps,
        });
    }

    pub fn tick(&mut self) -> Poll<Vec<T>> {
        let step = self.tick % self.steps;

        self.tick += 1;

        if let Some(slots) = self.hashed.get_mut(&step) {
            let mut current: Vec<T> = vec![];
            let mut reserved: Vec<Slot<T>> = vec![];

            for slot in slots {
                if slot.round == 0 {
                    current.push(slot.t);
                } else {
                    reserved.push(Slot::<T> {
                        t: slot.t,
                        round: slot.round - 1,
                    });
                }
            }

            self.hashed.insert(step, reserved);

            return Poll::Ready(current);
        }

        Poll::Pending
    }
}