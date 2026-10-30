// Copyright (C) 2026  Antonio-Miguel Corbi Bellot
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// use egui::{
//     emath::{self, RectTransform},
//     Frame, Pos2, Rect, Ui,
// };

use crate::{constants, worlds::Worlds};
use egui::{
    emath::{self, RectTransform},
    epaint::Hsva,
    pos2, Color32, CornerRadius, Frame, Painter, PointerButton, Pos2, Rect, Sense, Stroke, Ui,
    Vec2,
};
use libvoronoi::voronoi::Voronoi;
use std::time::Duration;

// -- Types: --------------------------------------------------------------
// ┌────────────┐
// │ VoronoiApp │
// └────────────┘

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// #[derive(serde::Deserialize, serde::Serialize)]
// #[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct VoronoiApp {
    // Example stuff:
    // label: String,

    // #[serde(skip)] // This how you opt-out of serialization of a field
    npoints: usize,
    // ╔════════════╗
    // ║ Model data ║
    // ╚════════════╝
    pub worlds: Option<Worlds>,
    pub voronoi: Option<Voronoi>,
}

// -- Traits: -------------------------------------------------------------
trait Gui {
    fn top_panel(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame);
    fn central_panel(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame);
    fn create_drawing_widget(&mut self, ui: &mut Ui);
    // fn draw_point(&self, painter: &Painter, x: f32, y: f32, color: Color32);
    fn draw_point(&self, painter: &Painter, p: Pos2, color: Color32);
    fn draw_points(&self, painter: &Painter, color: Color32);
    fn draw_bisects(&self, painter: &Painter, color: Color32);
}

// -- Impl. Blocks: -------------------------------------------------------
impl Default for VoronoiApp {
    fn default() -> Self {
        let npoints: usize = 2;

        Self {
            // Example stuff:
            npoints,
            worlds: None,
            voronoi: Some(Voronoi::new(npoints)),
        }
    }
}

impl Gui for VoronoiApp {
    fn draw_point(&self, painter: &Painter, p: Pos2, color: Color32) {
        let radius = 3.0; // Radio del punto en píxeles
                          //let center = Pos2::new(x, y);
        let center = p;
        painter.circle_filled(center, radius, color);
    }

    fn draw_points(&self, painter: &Painter, color: Color32) {
        let voronoi = self
            .voronoi
            .as_ref()
            .expect("voronoi field is not initialized");
        for &p in &voronoi.points {
            let wpos: Pos2 = [p.x, p.y].into();
            let spos = self.pos2_to_screen(wpos);

            self.draw_point(&painter, spos, color);
        }
    }

    fn draw_bisects(&self, painter: &Painter, color: Color32) {
        let voronoi = self
            .voronoi
            .as_ref()
            .expect("voronoi field is not initialized");
        for &p in &voronoi.points {
            let wpos: Pos2 = [p.x, p.y].into();
            let spos = self.pos2_to_screen(wpos);

            self.draw_point(&painter, spos, color);
        }
    }

    fn top_panel(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("top_panel").show(ui, |ui| {
            //ui.heading("Voronoi Diagram");
            ui.horizontal(|ui| {
                // The top panel is often a good place for a menu bar:
                egui::MenuBar::new().ui(ui, |ui| {
                    // NOTE: no File->Quit on web pages!
                    let is_web = cfg!(target_arch = "wasm32");
                    if !is_web {
                        ui.menu_button("File", |ui| {
                            if ui.button("Quit").clicked() {
                                ui.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        });
                        ui.add_space(8.0);
                    }

                    // Todo lo que pongas en este bloque se alineará a la derecha y de derecha a izquierda
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Nota: Al usar right_to_left, el primer elemento que añadas
                        // quedará más a la derecha.
                        ui.colored_label(egui::Color32::GOLD, "V.2026.10");

                        // Si quisieras añadir otro elemento a su izquierda dentro de este bloque:
                        ui.separator();
                        ui.colored_label(egui::Color32::RED, "Voronoi Diagram");

                        ui.separator();
                        // Dark/Light/OS theme style
                        egui::widgets::global_theme_preference_buttons(ui);
                    });
                });
            });
        });
    }

    fn central_panel(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            // ui.heading("eframe template");

            // ui.horizontal(|ui| {
            //     ui.label("Write something: ");
            //     ui.text_edit_singleline(&mut self.label);
            // });

            // ui.add(
            //     egui::Slider::new(&mut self.npoints, 2..=200)
            //         .text(egui::RichText::new("Random points").color(egui::Color32::LIGHT_BLUE)),
            // );
            // if ui.button("Increment").clicked() {
            //     self.npoints += 1;
            // }

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Random points:").color(egui::Color32::LIGHT_BLUE));
                let response = ui.add(
                    egui::DragValue::new(&mut self.npoints)
                        .clamp_existing_to_range(true)
                        .range(2..=constants::MAX_POINTS),
                );
                //if response.dragged() {
                if response.drag_stopped_by(egui::PointerButton::Primary) {
                    let mut voronoi = self
                        .voronoi
                        .as_mut()
                        .expect("voronoi field is not initialized");
                    // println!("Resizing: {}", self.npoints);
                    voronoi.resize(self.npoints);
                    voronoi.compute_perp_lines();
                }
            });

            ui.separator();
            self.create_drawing_widget(ui);
            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/main/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }

    fn create_drawing_widget(&mut self, ui: &mut Ui) {
        // const CANVAS_W: f32 = 800.0;
        // const CANVAS_H: f32 = 600.0;
        let height_for_widgets = 40.0; // Espacio que necesitas abajo
        let scroll_height = ui.available_height() - height_for_widgets;

        egui::ScrollArea::both() // Habilita scroll horizontal y vertical
            .auto_shrink([false; 2]) // Evita que el área se colapse si hay poco contenido
            .max_height(scroll_height) // Limitamos la altura del scroll
            .show(ui, |ui| {
                // 1. Definimos el tamaño total de nuestro "papel" o lienzo
                let canvas_size = egui::vec2(constants::CANVAS_W, constants::CANVAS_H);
                Frame::canvas(ui.style())
                    // .corner_radius(5.0)
                    // Fondo
                    .fill(Color32::from_rgb(20, 60, 100)) // Fondo azul
                    //.stroke(Stroke::new(1.5, Color32::LIGHT_RED)) // Borde negro
                    .show(ui, |ui| {
                        // self.ui_canvas(ui); // Llamamos a la lógica de dibujo

                        let (response, painter) = ui.allocate_painter(
                            // ui.available_size() - [2.0, 45.0].into(),
                            canvas_size,
                            Sense::drag() | Sense::click(),
                            // ¡Importante! Detectar arrastre (clic + movimiento), Sense::DRAG | Sense::CLICK,
                        );

                        // If there is no worlds object defined,
                        // define it, else update screen_rect
                        self.init_worlds(response.rect);

                        // 1. Comprobamos el click izquierdo
                        if response.secondary_clicked() {
                            // println!("¡Click derecho detectado en el Painter!");
                            if let Some(pos) = response.interact_pointer_pos() {
                                println!("Right Click en la posición: {:?}", pos);
                                // let wpos = self.worlds.as_ref().unwrap().pos2_to_world(pos);
                                // let wx = wpos.x;
                                // let wy = wpos.y;
                                // let nrepellers_old = self.repellers.len();
                                // self.remove_repellers_at_point(wx, wy);
                                // let nrepellers_new = self.repellers.len();
                                // // dbg!(nrepellers_old, nrepellers_new);
                                // if nrepellers_new == nrepellers_old {
                                //     // No repeller under (wx,wy) removed so create a new one
                                //     let repeller = Repeller::new(
                                //         wx,
                                //         wy,
                                //         constants::REP_POWER,
                                //         constants::REP_SIZE,
                                //     );
                                //     self.repellers.push(repeller);
                                // }
                            }
                        }

                        // 2. Comprobamos el click intermedio
                        if response.middle_clicked() {
                            //println!("¡Click central detectado en el Painter!");
                            if let Some(pos) = response.interact_pointer_pos() {
                                println!("Middle Click en la posición: {:?}", pos);
                            }
                            //self.show_data();
                        }

                        // 3. Comprobamos el click izquierdo
                        if response.clicked() {
                            if let Some(pos) = response.interact_pointer_pos() {
                                println!("Left Click en la posición: {:?}", pos);

                                // Show random points coords.
                                let voronoi = self
                                    .voronoi
                                    .as_ref()
                                    .expect("voronoi field is not initialized");
                                for &p in &voronoi.points {
                                    let wpos: Pos2 = [p.x, p.y].into();
                                    let spos = self.pos2_to_screen(wpos);

                                    println!(
                                        "Screenpos: sx[{}],sy[{}],wx[{}],wy[{}]",
                                        spos.x, spos.y, wpos.x, wpos.y
                                    );
                                }
                            }
                        }

                        if response.dragged_by(PointerButton::Primary) {
                            // Obtenemos la posición actual del puntero
                            if let Some(pos) = response.interact_pointer_pos() {
                                static mut WX_LAST: usize = 0;
                                static mut WY_LAST: usize = 0;
                                // let wpos = self.worlds.as_ref().unwrap().pos2_to_world(pos);
                                // let wx = wpos.x as usize;
                                // let wy = wpos.y as usize;

                                // Hide Mouse
                                // let ctx = ui.ctx();
                                // ctx.send_viewport_cmd(egui::ViewportCommand::CursorVisible(false));
                            }
                        }

                        if response.drag_stopped_by(PointerButton::Primary) {
                            // Show mouse
                            // let ctx = ui.ctx();
                            // // Útil si el usuario estaba arrastrando y soltó el botón
                            // ctx.send_viewport_cmd(egui::ViewportCommand::CursorVisible(true));
                        }

                        // ┌──────────────┐
                        // │ Drawing code │
                        // └──────────────┘
                        // Draw voronoi points.
                        self.draw_points(&painter, Color32::LIGHT_RED);

                        // Sleep a bit
                        // std::thread::sleep(Duration::from_micros(
                        //     (constants::SLEEP as f32 / self.speed) as u64,
                        // ));
                    });
            });
    }
}

impl VoronoiApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // // Load previous app state (if any).
        // // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        // } else {
        //     Default::default()
        // }

        Default::default()
    }

    pub fn create_worlds(&mut self, wr: Rect, sr: Rect) {
        self.worlds = Some(Worlds::new(wr, sr));
    }

    pub fn init_worlds(&mut self, sr: Rect) {
        if self.worlds.is_none() {
            let wr = egui::Rect::from_min_max(constants::WR_MIN.into(), constants::WR_MAX.into());
            self.create_worlds(wr, sr);
            // self.set_status_text(&format!("Worlds created"), egui::Color32::RED);
        } else {
            // Update worlds screen_rect
            self.worlds.as_mut().unwrap().update_screen_rect(sr);
        }
    }

    pub fn pos2_to_screen(&self, pos: Pos2) -> Pos2 {
        self.worlds.as_ref().unwrap().pos2_to_screen(pos)
    }

    pub fn pos2_to_world(&self, pos: Pos2) -> Pos2 {
        self.worlds.as_ref().unwrap().pos2_to_world(pos)
    }
}

impl eframe::App for VoronoiApp {
    /// Called by the framework to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // 1. Create the top panel, it can hold a menu bar among other things
        self.top_panel(ui, frame);
        // 2. Create the top panel, it can hold a menu bar among other things
        self.central_panel(ui, frame);
        // 3. Force redraw of contents
        ui.request_repaint();
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
