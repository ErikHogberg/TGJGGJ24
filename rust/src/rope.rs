use godot::engine::{CsgCylinder3D, ICsgCylinder3D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CsgCylinder3D)]
struct Rope {
    #[export]
    start_target: Gd<Node3D>,
    #[export]
    end_target: Gd<Node3D>,
    #[export]
    rotation_parent: Gd<Node3D>,
    dirty: bool,
    #[base]
    base: Base<CsgCylinder3D>,
}


#[godot_api]
impl ICsgCylinder3D for Rope {
    fn init(base: Base<CsgCylinder3D>) -> Self {
        Self {
            start_target: Node3D::new_alloc(),
            end_target: Node3D::new_alloc(),
            rotation_parent: Node3D::new_alloc(),
            dirty:true,
            base,
        }
    }

    fn process(&mut self, _delta: f64){
        if !self.dirty{
            return;
        }
        self.dirty = false;
        let end = self.end_target.get_global_position();
        let start = self.start_target.get_global_position();
        let delta_pos = end-start;
        
        {
            let mut cyl = self.base_mut();
            cyl.set_height(delta_pos.length());
        }
        self.rotation_parent.set_global_position(start + delta_pos*0.5f32);
        self.rotation_parent.look_at(end);
        
    }
}
