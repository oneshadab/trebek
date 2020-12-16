use std::{collections::HashSet};


use super::object_heap::{ObjectHeap, ObjectId};

pub struct GarbageCollector {
  marked: HashSet<ObjectId>
}

impl GarbageCollector {
  pub fn new() -> GarbageCollector {
    GarbageCollector {
      marked: HashSet::new(),
    }
  }

  pub fn collect(&mut self, root_obj_id: ObjectId, heap: &mut ObjectHeap) {
    self.marked.clear();

    self.mark(root_obj_id, heap);

    self.sweep(heap);
  }

  fn mark(&mut self, root_obj_id: ObjectId, heap: &ObjectHeap) {

  }

  fn sweep(&self, heap: &mut ObjectHeap) {

  }
}
