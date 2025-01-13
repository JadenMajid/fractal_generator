use std::{f32::consts::PI, ops::RangeInclusive, os::windows::ffi::EncodeWide};
use egui::{accesskit::Tree, epaint::PathStroke, Painter, Pos2, Rgba, Ui};

use crate::Drawable;


#[derive(Debug, Copy, Clone)]
pub(crate) struct TreeFractal {
    pub origin: Pos2,
    pub depth: u32,
    pub angle: f32,
    pub length: f32,
    pub sweep_angle: f32,
    pub length_factor: f32,
    pub line_thickness: f32,
    pub color: [f32; 3],
    pub arms: i32,
    pub centered: bool,
    pub sweep: bool,
    pub sweep_speed:f32,
}

struct TreeFractalNode {
    
}

impl TreeFractal {
    
    fn recurse(&mut self, painter: &Painter) {
        if self.depth == 0 {
            return;
        }
        let vec_to_add = egui::Vec2::angled(self.angle) * self.length;
        let line_endpoint = self.origin + vec_to_add;
        painter.line_segment(
            [self.origin, line_endpoint],
            PathStroke::new(
                self.line_thickness,
                Rgba::from_rgb(
                    self.color[0],
                    self.color[1],
                    self.color[2],
                ),
            ),
        );
        self.origin = line_endpoint;
        self.length *= self.length_factor;
        self.depth -= 1;

        for i in 0..self.arms{
            let mut node = self.clone();
            node.angle -= node.sweep_angle /2.;
            node.angle += node.sweep_angle * (i as f32) / ((self.arms-1) as f32);

            node.recurse(painter);
        }


    }
}

impl Drawable for TreeFractal {
    fn update(&mut self, ctx: &egui::Context) {
        let screenspace = ctx.input(|i: &egui::InputState| i.screen_rect());
        self.origin = screenspace.center();
        if !self.centered {
            self.origin.y += screenspace.height()/4.
        }
        let dt = ctx.input(|i|{i.stable_dt});
        if self.sweep {
            self.sweep_angle += self.sweep_speed*dt;
            self.sweep_angle %= 8. * PI;
            if self.sweep_angle >= 8. * PI{
                self.sweep_angle = 0.0;
            } else if self.sweep_angle <= 0. {
                self.sweep_angle = 8.*PI;
            }
        }
    }
        

    fn draw(&mut self, painter: &Painter){
        
        for i in 0..self.arms{
            let mut node = self.clone();
            node.angle -= node.sweep_angle /2.;
            node.angle += node.sweep_angle * (i as f32) / ((self.arms-1) as f32);

            node.recurse(painter);
        }
    }


    fn draw_controls(&mut self, ui: &mut Ui) {
            // This is all ui stuff, feel free to ignore
            ui.add(
                egui::Slider::new(
                    &mut self.angle,
                    RangeInclusive::new(0.0, 2.0 * PI),
                )
                .text("Fractal Angle"),
            );
            // ui.add(
            //     egui::Slider::new(&mut self.whole_rotation_speed, RangeInclusive::new(-5., 5.))
            //         .text("Fractal Rotation Speed"),
            // );
            ui.add(
                egui::Slider::new(
                    &mut self.sweep_angle,
                    RangeInclusive::new(0.0, 8.0 * PI),
                )
                .text("sweep Angle"),
            );
            ui.add(
                egui::Slider::new(&mut self.sweep_speed, RangeInclusive::new(-1., 1.))
                    .text("sweep Speed"),
            );

            ui.add(egui::Slider::new(&mut self.arms, RangeInclusive::new(1, 10)).text("Arms"));
            ui.add(
                egui::Slider::new(
                    &mut self.length,
                    RangeInclusive::new(0.0, 300.0),
                )
                .text("Arm Length"),
            );
            ui.add(
                egui::Slider::new(
                    &mut self.line_thickness,
                    RangeInclusive::new(0.0, 10.0),
                )
                .text("Arm Thickness"),
            );
            ui.add(
                egui::Slider::new(
                    &mut self.length_factor,
                    RangeInclusive::new(0.000001, 2.),
                )
                .text("Arm Length Factor"),
            );
            ui.add(
                egui::Slider::new(&mut self.depth, RangeInclusive::new(1, 10))
                    .text("depth"),
            );
            ui.checkbox(&mut self.centered, "centered?");
            ui.checkbox(&mut self.sweep, "Sweep?");
            ui.add(egui::Label::new("Fractal Color"));
            egui::color_picker::color_edit_button_rgb(ui, &mut self.color);
    }
}
