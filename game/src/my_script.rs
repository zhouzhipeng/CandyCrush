
use fyrox::{
    core::{visitor::prelude::*, reflect::prelude::*, type_traits::prelude::*},
    event::Event, script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};
use fyrox::event::{ElementState, MouseButton, WindowEvent};

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "3ef5e75c-c03e-4053-b776-cd9f7c4ec9f9")]
#[visit(optional)]
pub struct MyScript {
    // Add fields here.
}

impl ScriptTrait for MyScript {
    fn on_init(&mut self, context: &mut ScriptContext) {
        // Put initialization logic here.
        println!("init22>>>");

    }

    fn on_start(&mut self, context: &mut ScriptContext) {
        // There should be a logic that depends on other scripts in scene.
        // It is called right after **all** scripts were initialized.
    }

    fn on_deinit(&mut self, context: &mut ScriptDeinitContext) {
        // Put de-initialization logic here.
    }

    fn on_os_event(&mut self, event: &Event<()>, context: &mut ScriptContext) {
        // Respond to OS events here.
        if let Event::WindowEvent {event , ..} = event{
            if let WindowEvent::MouseInput { button, state, .. } = event {
                if *button == MouseButton::Left && *state == ElementState::Pressed{
                    println!("mouse left button 3333");
                }
            }
        }
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        // Put object logic here.
        // println!("on_update22>>>");
    }
}
    