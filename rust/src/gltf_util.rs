use std::boxed::Box;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::fs;
use std::rc::Rc;

use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
#[user_data(user_data::MutexData<GltfUtil>)]
#[register_with(Self::register)]
pub struct GltfUtil {
    #[property]
    file_path: String,
    // TODO godot dictionary to pass back to godot?
}

#[methods]
impl GltfUtil {
    fn register(_builder: &ClassBuilder<Self>) {}

    fn new(_owner: &Node) -> Self {
        return GltfUtil {
            file_path: "".to_string(),
        };
    }

    #[export]
    fn _ready(&mut self, _owner: &Node) {
        Self::import_scene(&self.file_path).expect("Failed importing scene");
    }

    fn import_scene(path: &str) -> Result<(), Box<dyn StdError>> {
        let file = fs::File::open(&path)?;
        let (doc, buffers, images) = gltf::import(path).unwrap();
        let imp = ImportData {
            doc,
            buffers,
            images,
        };

        // println!("{:#?}", imp.doc.meshes());

        let main_scene = imp.doc.scenes().nth(0).expect("Main scene not found");
        for node in main_scene.nodes() {
            // TODO we have mesh access
            node.mesh().unwrap();
            // TODO recursively access more nodes
        }

        Ok(())
    }
}

struct ImportData {
    pub doc: gltf::Document, // AKA root
    pub buffers: Vec<gltf::buffer::Data>,
    pub images: Vec<gltf::image::Data>,
}

// #[derive(Default)]
// struct GltfRoot {
//     pub nodes: Vec<GltfNode>,
//     pub meshes: Vec<Rc<GltfMesh>>, // TODO!: use gltf indices; drop Rc?
//     pub textures: Vec<Rc<GltfTexture>>,
//     pub materials: Vec<Rc<GltfMaterial>>,
// }

// struct GltfNode {
//     pub index: usize, // glTF index
//     pub name: Option<String>,
//     pub children: Vec<usize>,
//     pub mesh: Option<Rc<GltfMesh>>,

//     // Godot data types because I'm lazy
//     pub rotation: Quat,
//     pub scale: Vector3,
//     pub translation: Vector3,
// }

// struct GltfMesh {
//     pub index: usize, // glTF index
//     pub name: Option<String>,
//     pub primitives: Vec<GltfPrimitive>,
// }

// struct GltfTexture {
//     pub index: usize, // glTF index
//     pub name: Option<String>,
//     pub id: u32,        // OpenGL id
//     pub tex_coord: u32, // the tex coord set to use
// }

// struct GltfMaterial {
//     pub index: Option<usize>,
//     /// glTF index
//     pub name: Option<String>,

//     pub base_color_factor: Color, // Godot data type
//     pub base_color_texture: Option<Rc<GltfTexture>>,
//     pub metallic_factor: f32,
//     pub roughness_factor: f32,
//     pub metallic_roughness_texture: Option<Rc<GltfTexture>>,

//     pub normal_texture: Option<Rc<GltfTexture>>,
//     pub normal_scale: Option<f32>,

//     pub occlusion_texture: Option<Rc<GltfTexture>>,
//     pub occlusion_strength: f32,
//     pub emissive_factor: Vector3,
//     pub emissive_texture: Option<Rc<GltfTexture>>,

//     pub alpha_cutoff: f32,
//     pub alpha_mode: gltf::material::AlphaMode,

//     pub double_sided: bool,
// }

// struct GltfVertex {
//     pub position: Vector3,
//     pub normal: Vector3,
//     pub tangent: Color,
//     pub tex_coord_0: Vector2,
//     pub tex_coord_1: Vector2,
//     pub color_0: Color,
//     pub joints_0: [u16; 4],
//     pub weights_0: Color,
// }

// struct GltfPrimitive {}
