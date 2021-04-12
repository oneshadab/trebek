use std::{
    collections::{HashSet, VecDeque},
    env,
};

use super::object_heap::{ObjectHeap, ObjectId};

pub struct GarbageCollector {
    marked: HashSet<ObjectId>,
    current_iter: u32,
}

static GC_ITERATIONS: u32 = 1154;

impl GarbageCollector {
    pub fn new() -> GarbageCollector {
        GarbageCollector {
            marked: HashSet::new(),
            current_iter: 0,
        }
    }

    pub fn collect(&mut self, obj_id: ObjectId, heap: &mut ObjectHeap) {
        self.current_iter += 1;

        if self.current_iter % GC_ITERATIONS != 0 {
            // Only run GC once every GC_ITERATIONS
            return;
        }

        self.marked.clear();

        self.mark(obj_id, heap);

        self.sweep(heap);
    }

    fn mark(&mut self, obj_id: ObjectId, heap: &ObjectHeap) {
        let mut queue = VecDeque::new();

        queue.push_back(obj_id);

        while !queue.is_empty() {
            let obj_id = queue.pop_front().unwrap();

            self.marked.insert(obj_id);

            let obj = match heap.get(obj_id) {
                Some(obj) => obj,
                None => panic!("Object with id {} no longer in heap", obj_id),
            };

            for ref_obj_id in obj.trace() {
                if !self.marked.contains(&ref_obj_id) {
                    self.marked.insert(ref_obj_id);
                    queue.push_back(ref_obj_id);
                }
            }
        }
    }

    fn sweep(&self, heap: &mut ObjectHeap) {
        let unreachbale_ids: Vec<ObjectId> = heap
            .objects
            .keys()
            .filter(|obj_id| !self.marked.contains(obj_id))
            .map(|obj_id| obj_id.clone())
            .collect();

        if let Ok(_) = env::var("DEBUG") {
            for id in unreachbale_ids.clone() {
                let obj = heap.get(id).unwrap();
                eprintln!("[GC DBG]: {:?} {:?}", id, obj);
            }
        }

        for obj_id in unreachbale_ids {
            heap.objects.remove(&obj_id);
        }
    }
}
