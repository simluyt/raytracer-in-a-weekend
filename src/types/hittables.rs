use crate::types::hittable::Hittable;
use crate::types::ray::Ray;
use crate::types::hitrecord::Hitrecord;

pub struct Hittables {
    pub(crate) items :  Vec<Box<dyn Hittable>>
}

impl Hittables {
    pub fn clear(&mut self) { self.items.clear()}
    pub fn add(&mut self, item : Box<dyn Hittable>) { self.items.push(item)}
}

impl Hittable for Hittables {
    fn hit(&self, r :&Ray, t_min : f64, t_max : f64, rec: &mut Hitrecord) -> bool {
        let mut temp_rec = rec;
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.items.iter() {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                //rec = temp_rec;
            }
        }

        return hit_anything;
    }
}