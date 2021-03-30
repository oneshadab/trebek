use std::collections::HashMap;

use crate::types::t_object::TObject;


pub type ObjectId = usize;

pub struct ObjectHeap {
  pub objects: HashMap<ObjectId, Box<TObject>>,
  last_id: ObjectId
}

impl ObjectHeap {
  pub fn new() -> ObjectHeap {
    ObjectHeap {
      objects: HashMap::new(),
      last_id: 0
    }
  }

  pub fn add(&mut self, obj: TObject) -> ObjectId {
    let obj_id = self.gen_id();
    self.objects.insert(obj_id, Box::new(obj));
    obj_id
  }

  pub fn get(&self, obj_id: ObjectId) -> Option<&TObject>  {
    match self.objects.get(&obj_id) {
      Some(boxed_obj) => { Some(boxed_obj) }
      None => { None }
    }
  }

  pub fn get_mut(&mut self, obj_id: ObjectId) -> Option<&mut TObject>  {
    match self.objects.get_mut(&obj_id) {
      Some(boxed_obj) => { Some(boxed_obj) }
      None => { None }
    }
  }

  fn gen_id(&mut self) -> ObjectId {
    let id = self.last_id;
    self.last_id += 1;
    id
  }
}
