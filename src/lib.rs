use hudhook::ImguiRenderLoop;
use hudhook::hooks::dx12::ImguiDx12Hooks;
use hudhook::windows::Win32::UI::WindowsAndMessaging::HOOKPROC;
use hudhook::*;
use imgui::Condition;
use imgui::Key::T;
use mem_rs::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;

struct SendProcess(Process);

unsafe impl Send for SendProcess {}
unsafe impl Sync for SendProcess {}

struct SendPointer(Pointer);
unsafe impl Sync for SendPointer {}

unsafe impl Send for SendPointer {}

mod memory;
use memory::ds1::Ds1;

impl ImguiRenderLoop for Ds1 {
    fn render(&mut self, ui: &mut imgui::Ui) {
        ui.window("Hello hudhook")
            .size([368.0, 568.0], Condition::FirstUseEver)
            .position([16.0, 16.0], Condition::FirstUseEver)
            .build(|| {
                ui.text(format!("HP {:?}", self.get_HP()));
                ui.text(format!("Stamina {:?}", self.get_Stamina()));
                ui.text(format!("Pos X {:?}", self.get_x_pos()));
                ui.text(format!("Pos Y {:?}", self.get_y_pos()));
                ui.text(format!("Pos Z {:?}", self.get_z_pos()));

                if (ui.button("Eject")) {
                    print!("test");
                    hudhook::eject();
                }
                if (ui.checkbox("inf stam", &mut self.get_no_stam_consume())) {
                    self.set_no_stam_consume();
                }

                if (ui.checkbox("no update ai", &mut self.get_no_update_ai())) {
                    self.set_no_update_ai();
                }
            });
    }
}

use hudhook::hooks::dx9::ImguiDx9Hooks;
use hudhook::*;

hudhook!(ImguiDx9Hooks, Ds1::new());
