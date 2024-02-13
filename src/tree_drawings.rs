use kurbo::{Affine, Circle, Rect};
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::{
    particle::{GeoQuery, Particle, World, PARTICLE_RADIUS},
    quad_tree::QuadTree,
    rstar_tree::RStartree,
    v2::{TreeValue, V2},
    zorder_tree::ZOrderTree,
};

pub struct DrawContext {
    pub mouse_pos: Option<V2>,
}

pub trait Drawable {
    fn draw(&self, ctx: &CanvasRenderingContext2d, draw_context: &DrawContext) -> Option<()>;
}

impl<T: TreeValue> Drawable for QuadTree<T> {
    fn draw(&self, ctx: &CanvasRenderingContext2d, draw_context: &DrawContext) -> Option<()> {
        Some(())
    }
}

impl<T: TreeValue> Drawable for ZOrderTree<T> {
    fn draw(&self, ctx: &CanvasRenderingContext2d, draw_context: &DrawContext) -> Option<()> {
        Some(())
    }
}

impl<T: TreeValue> Drawable for RStartree<T> {
    fn draw(&self, ctx: &CanvasRenderingContext2d, draw_context: &DrawContext) -> Option<()> {
        let values = self.boundings();
        ctx.save();
        ctx.set_stroke_style(&JsValue::from("white"));
        ctx.begin_path();
        values.for_each(|rect: Rect| ctx.rect(rect.x0, rect.y0, rect.width(), rect.height()));
        ctx.stroke();
        ctx.restore();
        Some(())
    }
}

impl<T: GeoQuery<Particle> + Drawable> Drawable for World<T> {
    fn draw(&self, ctx: &CanvasRenderingContext2d, draw_context: &DrawContext) -> Option<()> {
        ctx.save();
        ctx.begin_path();
        ctx.set_fill_style(&JsValue::from_str("white"));
        self.particles.iter().for_each(|particle| {
            particle.draw(ctx, draw_context);
        });
        ctx.fill();
        ctx.restore();
        // let center = V2::new(WIDTH as f64 / 2., HEIGHT as f64 / 2.);
        // let gradient = self.calc_gradient(&center);
        // draw_arrow(&center, &center.add(&gradient), ctx);
        if let Some(ref mouse_pos) = self.mouse_pos {
            if self.show_quad_tree {
                self.tree.draw(ctx, draw_context);
            }
        };
        Some(())
    }
}

impl Drawable for Particle {
    fn draw(&self, ctx: &CanvasRenderingContext2d, draw_context: &DrawContext) -> Option<()> {
        ctx.rect(
            self.position.x - PARTICLE_RADIUS,
            self.position.y - PARTICLE_RADIUS,
            PARTICLE_RADIUS * 2.0,
            PARTICLE_RADIUS * 2.0,
        );
        Some(())
    }
}
