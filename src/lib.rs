extern crate samp;
use glam::Vec3;
use parking_lot::RwLock;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;
use samp::{initialize_plugin, native, plugin::SampPlugin, prelude::*, error::AmxResult, exec_public, cell::AmxString};

static OBJECTS: Lazy<RwLock<HashMap<u32, DynamicObject>>> = Lazy::new(|| RwLock::new(HashMap::new()));
static mut NEXT_OBJECT_ID: u32 = 1;

#[derive(Debug, Serialize, Deserialize)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl From<Vec3> for Vector3 {
    fn from(v: Vec3) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<Vector3> for Vec3 {
    fn from(v: Vector3) -> Self {
        Vec3::new(v.x, v.y, v.z)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct DynamicObject {
    model_id: i32,
    position: Vector3,
    rotation: Vector3,
}

pub struct ObjectEditor;

impl SampPlugin for ObjectEditor {
    fn on_load(&mut self) {
        println!("Dynamic Object Editor loaded!");
    }

    fn on_unload(&mut self) {
        println!("Dynamic Object Editor unloaded!");
    }
}

impl ObjectEditor {
    #[native(name = "CreateDynamicObject")]
    pub fn create_dynamic_object(&mut self, _: &Amx, model_id: i32, x: f32, y: f32, z: f32, rx: f32, ry: f32, rz: f32) -> AmxResult<u32> {
        let object = DynamicObject {
            model_id,
            position: Vector3 { x, y, z },
            rotation: Vector3 { x: rx, y: ry, z: rz },
        };

        let object_id = unsafe {
            let id = NEXT_OBJECT_ID;
            NEXT_OBJECT_ID += 1;
            id
        };

        OBJECTS.write().insert(object_id, object);
        Ok(object_id)
    }

    #[native(name = "EditDynamicObject")]
    pub fn edit_dynamic_object(&mut self, amx: &Amx, player_id: i32, object_id: u32) -> AmxResult<bool> {
        if let Some(obj) = OBJECTS.read().get(&object_id) {
            let _ = exec_public!(
                amx,
                "CreatePlayerObject",
                player_id,
                obj.model_id,
                obj.position.x,
                obj.position.y,
                obj.position.z,
                obj.rotation.x,
                obj.rotation.y,
                obj.rotation.z,
                300.0
            )?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[native(name = "GetDynamicObjectPos")]
    pub fn get_dynamic_object_pos(&mut self, _: &Amx, object_id: u32, x: &mut f32, y: &mut f32, z: &mut f32) -> AmxResult<bool> {
        if let Some(obj) = OBJECTS.read().get(&object_id) {
            *x = obj.position.x;
            *y = obj.position.y;
            *z = obj.position.z;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[native(name = "SetDynamicObjectPos")]
    pub fn set_dynamic_object_pos(&mut self, _: &Amx, object_id: u32, x: f32, y: f32, z: f32) -> AmxResult<bool> {
        if let Some(obj) = OBJECTS.write().get_mut(&object_id) {
            obj.position = Vector3 { x, y, z };
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[native(name = "GetDynamicObjectRot")]
    pub fn get_dynamic_object_rot(&mut self, _: &Amx, object_id: u32, rx: &mut f32, ry: &mut f32, rz: &mut f32) -> AmxResult<bool> {
        if let Some(obj) = OBJECTS.read().get(&object_id) {
            *rx = obj.rotation.x;
            *ry = obj.rotation.y;
            *rz = obj.rotation.z;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[native(name = "SetDynamicObjectRot")]
    pub fn set_dynamic_object_rot(&mut self, _: &Amx, object_id: u32, rx: f32, ry: f32, rz: f32) -> AmxResult<bool> {
        if let Some(obj) = OBJECTS.write().get_mut(&object_id) {
            obj.rotation = Vector3 { x: rx, y: ry, z: rz };
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[native(name = "DeleteDynamicObject")]
    pub fn delete_dynamic_object(&mut self, _: &Amx, object_id: u32) -> AmxResult<bool> {
        Ok(OBJECTS.write().remove(&object_id).is_some())
    }

    #[native(name = "ExportMapData")]
    pub fn export_map_data(&mut self, amx: &Amx, filename: AmxString<'_>) -> AmxResult<bool> {
        let objects = OBJECTS.read();
        if let Ok(json) = serde_json::to_string(&*objects) {
            if let Ok(_) = std::fs::write(filename.to_string(), json) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    #[native(name = "ImportMapData")]
    pub fn import_map_data(&mut self, amx: &Amx, filename: AmxString<'_>) -> AmxResult<bool> {
        if let Ok(json) = std::fs::read_to_string(filename.to_string()) {
            if let Ok(objects) = serde_json::from_str::<HashMap<u32, DynamicObject>>(&json) {
                *OBJECTS.write() = objects;
                return Ok(true);
            }
        }
        Ok(false)
    }
}

impl Default for ObjectEditor {
    fn default() -> Self {
        ObjectEditor
    }
}

initialize_plugin!(
    natives: [
        ObjectEditor::create_dynamic_object,
        ObjectEditor::edit_dynamic_object,
        ObjectEditor::get_dynamic_object_pos,
        ObjectEditor::set_dynamic_object_pos,
        ObjectEditor::get_dynamic_object_rot,
        ObjectEditor::set_dynamic_object_rot,
        ObjectEditor::delete_dynamic_object,
        ObjectEditor::export_map_data,
        ObjectEditor::import_map_data
    ],
    {
        let plugin = ObjectEditor;
        return plugin;
    }
);
