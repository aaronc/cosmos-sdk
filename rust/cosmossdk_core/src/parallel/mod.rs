use im::{HashSet, OrdMap, OrdSet};

struct RoundScheduler<Op> {
    last_key_write: OrdMap<Vec<u8>, u64>,
    edges: OrdMap<u64, HashSet<u64>>,
    dep_count: OrdMap<u64, u64>,
    next_scheduled_id: u64,
    scheduled: OrdMap<u64, (Op, KeySet)>
}

struct KeyAccess {
    key: Vec<u8>,
    write: bool,
}

impl <Op> RoundScheduler<Op> {
    fn schedule(&mut self, op_id: u64, ks: KeySet, op: Op) -> bool {
        if op_id != self.next_scheduled_id {
            return false;
        }

        for ka in ks.iter() {
            let key = &ka.key;
            if let Some(last) = self.last_key_write.get(key) {
                let last_edges = self.edges.get_mut(&last).unwrap();
                if !last_edges.contains(&op_id) {
                    last_edges.insert(op_id);
                    self.dep_count.insert(op_id, self.dep_count.get(&op_id).unwrap_or(&0) + 1);
                }
            }
            if ka.write {
                self.last_key_write.insert(key.clone(), op_id);
            }
        }

        self.scheduled.insert(op_id, (op, ks));
        self.next_scheduled_id += 1;

        true
    }
}

impl <Op: FnOnce(KeySet)> RoundScheduler<Op> {
    fn execute(&mut self) {
        // let (op_id, _) = self.scheduled.get_min();
        todo!()
    }
}

type KeySet = HashSet<KeyAccess>;
