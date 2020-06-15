use sequence_trie::SequenceTrie;

pub struct StackInstructionCounter {
    current_stack: Vec<u64>,
    instr_counts: SequenceTrie<u64, u64>,
}

impl StackInstructionCounter {
    pub fn new() -> Self {
        Self {
            current_stack: vec![],
            instr_counts: SequenceTrie::new(),
        }
    }

    pub fn push(&mut self, addr: u64) {
        if self.current_stack.len() >= 100 {
            panic!("maximum stack depth exceeded")
        }
        self.current_stack.push(addr);
    }

    pub fn pop(&mut self) {
        self.current_stack.pop();
    }

    pub fn count(&mut self, addr: u64, amount: u64) {
        if let Some(count) = self
            .instr_counts
            .get_mut(self.current_stack.iter().chain(Some(&addr)))
        {
            *count += amount;
        } else {
            self.instr_counts
                .insert(self.current_stack.iter().chain(Some(&addr)), amount);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Vec<u64>, u64)> + '_ {
        self.instr_counts.iter().map(|(stack, count)| {
            let addrs = stack.into_iter().rev().cloned().collect();
            (addrs, *count)
        })
    }
}
