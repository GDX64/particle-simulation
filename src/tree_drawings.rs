use kurbo::Rect;
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
        if let Some(mouse_pos) = draw_context.mouse_pos.as_ref() {
            ctx.set_fill_style(&JsValue::from("red"));
            ctx.begin_path();
            self.query_distance(mouse_pos, PARTICLE_RADIUS, |value| {
                value.draw(ctx, draw_context);
            });
            ctx.fill();

            self.query_distance_path(mouse_pos, PARTICLE_RADIUS)
                .into_iter()
                .for_each(|node| {
                    ctx.set_stroke_style(&JsValue::from("red"));
                    ctx.begin_path();
                    let circ = node.get_circ();
                    ctx.arc(
                        circ.center.x,
                        circ.center.y,
                        circ.radius,
                        0.0,
                        std::f64::consts::PI * 2.0,
                    )
                    .ok();
                    ctx.stroke();
                })
        }
        Some(())
    }
}

impl<T: TreeValue> Drawable for ZOrderTree<T> {
    fn draw(&self, ctx: &CanvasRenderingContext2d, draw_context: &DrawContext) -> Option<()> {
        let mut values = self.values();
        if let Some(first) = values.next() {
            let first_position = first.position();
            ctx.set_stroke_style(&JsValue::from("yellow"));
            ctx.move_to(first_position.x, first_position.y);
            values.for_each(|value| {
                let position = value.position();
                ctx.line_to(position.x, position.y);
            });
            ctx.stroke();
        }
        if let Some(mouse_pos) = draw_context.mouse_pos.as_ref() {
            ctx.set_fill_style(&JsValue::from("red"));
            ctx.begin_path();
            self.query_distance(mouse_pos, PARTICLE_RADIUS, |value| {
                value.draw(ctx, draw_context);
            });
            ctx.fill();
        }
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
        if self.is_pressing_mouse {
            self.tree.draw(ctx, draw_context);
        }
        Some(())
    }
}

impl<T: TreeValue> Drawable for T {
    fn draw(&self, ctx: &CanvasRenderingContext2d, draw_context: &DrawContext) -> Option<()> {
        let position = self.position();
        ctx.rect(
            position.x - PARTICLE_RADIUS,
            position.y - PARTICLE_RADIUS,
            PARTICLE_RADIUS * 2.0,
            PARTICLE_RADIUS * 2.0,
        );
        Some(())
    }
}
