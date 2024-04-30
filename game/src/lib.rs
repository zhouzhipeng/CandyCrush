//! Game project.
pub mod my_script;
pub mod my_script_2;

use fyrox::{
    core::pool::Handle, core::visitor::prelude::*,
    event::Event,
    gui::message::UiMessage,
    plugin::{Plugin, PluginContext, PluginRegistrationContext},
    scene::Scene,
};
use std::path::Path;

// Re-export the engine.
pub use fyrox;
use crate::my_script::MyScript;
use crate::my_script_2::MyScript2;

#[derive(Default, Visit)]
pub struct Game {
    scene: Handle<Scene>,
}

impl Plugin for Game {
    fn register(&self, _context: PluginRegistrationContext) {
        // Register your scripts here.
        _context.serialization_context.script_constructors.add::<MyScript>("My Script");
        _context.serialization_context.script_constructors.add::<MyScript2>("My Script22");

    }
    
    fn init(&mut self, scene_path: Option<&str>, context: PluginContext) {
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));
    }

    fn on_deinit(&mut self, _context: PluginContext) {
        // Do a cleanup here.
    }

    fn update(&mut self, _context: &mut PluginContext) {
        // Add your global update code here.
    }

    fn on_os_event(
        &mut self,
        _event: &Event<()>,
        _context: PluginContext,
    ) {
        // Do something on OS event here.
    }

    fn on_ui_message(
        &mut self,
        _context: &mut PluginContext,
        _message: &UiMessage,
    ) {
        // Handle UI events here.
    }

    fn on_scene_begin_loading(&mut self, _path: &Path, ctx: &mut PluginContext) {
        if self.scene.is_some() {
            ctx.scenes.remove(self.scene);
        }
    }

    fn on_scene_loaded(
        &mut self,
        _path: &Path,
        scene: Handle<Scene>,
        _data: &[u8],
        _context: &mut PluginContext,
    ) {
        self.scene = scene;
    }
}
