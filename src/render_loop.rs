use hudhook::ImguiRenderLoop;
use imgui::Condition;
use crate::memory::Ds1;

pub struct RenderLoop
{
    ds1: Ds1,
}

impl RenderLoop
{
    pub fn new() -> Self
    {
        let ds1 = Ds1::new();
        RenderLoop { ds1 }
    }
}

impl ImguiRenderLoop for RenderLoop {
    fn render(&mut self, ui: &mut imgui::Ui) {
        ui.window("Hello hudhook")
            .size([368.0, 568.0], Condition::FirstUseEver)
            .position([16.0, 16.0], Condition::FirstUseEver)
            .build(|| {
                ui.text(format!("HP {:?}", self.ds1.get_hp()));
                ui.text(format!("Stamina {:?}", self.ds1.get_stamina()));
                ui.text(format!("Pos X {:?}", self.ds1.get_x_pos()));
                ui.text(format!("Pos Y {:?}", self.ds1.get_y_pos()));
                ui.text(format!("Pos Z {:?}", self.ds1.get_z_pos()));

                if ui.button("Eject") {
                    print!("test");
                    hudhook::eject();
                }
                if ui.checkbox("inf stam", &mut self.ds1.get_no_stam_consume()) {
                    self.ds1.set_no_stam_consume();
                }

                if ui.checkbox("no update ai", &mut self.ds1.get_no_update_ai()) {
                    self.ds1.set_no_update_ai();
                }
            });
    }
}