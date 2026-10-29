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

use egui::{
    emath::{self, RectTransform},
    Frame, Pos2, Rect, Ui,
};

// ╔════════╗
// ║ Worlds ║
// ╚════════╝
// -- : -------------------------------------------------------------------
#[derive(Clone)]
pub struct Worlds {
    pub world_rect: Rect,
    pub screen_rect: Rect,
    pub w2s: emath::RectTransform,
    pub s2w: emath::RectTransform,
}

impl Worlds {
    pub fn new(wr: Rect, sr: Rect) -> Self {
        let w2s = emath::RectTransform::from_to(wr, sr);
        let s2w = w2s.inverse();

        Self {
            world_rect: wr,
            screen_rect: sr,
            w2s,
            s2w,
        }
    }

    pub fn update_screen_rect(&mut self, screen_rect: Rect) {
        //println!("Update worlds sr & transforms");

        // Store the canvas rect
        self.screen_rect = screen_rect;

        // Compute world2screen and screen2world transforms
        self.w2s = emath::RectTransform::from_to(self.world_rect, self.screen_rect);
        self.s2w = self.w2s.inverse();
    }

    pub fn pos2_to_screen(&self, pos: Pos2) -> Pos2 {
        // if !self.world_rect.contains(pos) {
        //     println!("pos: {:?} out of wr", pos);
        // }
        // Check that point x,y is inside its world_rect.
        // assert!(self.world_rect.contains(pos));
        self.w2s.transform_pos_clamped(pos)
    }

    pub fn pos2_to_world(&self, pos: Pos2) -> Pos2 {
        // Check that point x,y is inside its screen_rect.
        //assert!(self.screen_rect.contains(pos));
        self.s2w.transform_pos_clamped(pos)
    }

    pub fn rect_to_screen(&self, rect: Rect) -> Rect {
        // Check that 'rect' is inside its world_rect.
        // assert!(self.world_rect.contains(rect.min));
        // assert!(self.world_rect.contains(rect.max));
        self.w2s.transform_rect(rect)
    }

    pub fn rect_to_world(&self, rect: Rect) -> Rect {
        // Check that 'rect' is inside its screen_rect.
        // assert!(self.screen_rect.contains(rect.min));
        // assert!(self.screen_rect.contains(rect.max));
        self.s2w.transform_rect(rect)
    }
}
