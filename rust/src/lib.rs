use gdnative::prelude::*;

mod gltf_util;

fn init(handle: InitHandle) {
    handle.add_class::<gltf_util::GltfUtil>();
}

godot_init!(init);
