use kurbo::{Circle, Rect, Shape};
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::{
    hilbert_tree::{SpaceFillingCurve, SpaceFillingTree},
    particle::{GeoQuery, Particle, World, PARTICLE_RADIUS},
    quad_tree::QuadTree,
    rstar_tree::RStartree,
    v2::{TreeValue, V2},
};

pub struct DrawContext {
    pub mouse_pos: Option<V2>,
    pub mouse_radius: f64,
    pub width: f64,
    pub height: f64,
}

pub trait Drawable {
    fn draw(&self, ctx: &CanvasRenderingContext2d, draw_context: &DrawContext) -> Option<()>;
}

impl<T: TreeValue> Drawable for QuadTree<T> {
    fn draw(&self, ctx: &CanvasRenderingContext2d, draw_context: &DrawContext) -> Option<()> {
        if let Some(mouse_pos) = draw_context.mouse_pos.as_ref() {
            draw_mouse_range(ctx, mouse_pos, draw_context.mouse_radius);
            ctx.set_fill_style(&JsValue::from("red"));
            ctx.begin_path();
            self.query_distance(mouse_pos, draw_context.mouse_radius, |value| {
                value.draw(ctx, draw_context);
            });
            ctx.fill();

            ctx.begin_path();
            ctx.set_stroke_style(&JsValue::from("yellow"));
            self.for_each(|node| {
                let rect = node.get_rect();
                ctx.rect(rect.x0, rect.y0, rect.width(), rect.height());
            });
            ctx.stroke();

            ctx.begin_path();
            ctx.set_stroke_style(&JsValue::from("red"));
            self.query_distance_path(mouse_pos, draw_context.mouse_radius)
                .into_iter()
                .for_each(|node| {
                    let rect = node.get_rect();
                    ctx.rect(rect.x0, rect.y0, rect.width(), rect.height());
                });
            ctx.stroke();
        }
        Some(())
    }
}

impl<T: SpaceFillingCurve> Drawable for SpaceFillingTree<T> {
    fn draw(&self, ctx: &CanvasRenderingContext2d, draw_context: &DrawContext) -> Option<()> {
        ctx.begin_path();
        ctx.set_stroke_style(&JsValue::from("yellow"));
        ctx.move_to(0., 0.);
        let num_of_points = self.number_of(draw_context.width * 2.0, draw_context.height * 2.0);
        (0..num_of_points).for_each(|i| {
            let (x, y) = self.pair_of(i);
            ctx.line_to(x, y);
        });
        ctx.stroke();
        if let Some(mouse_pos) = draw_context.mouse_pos.as_ref() {
            let mouse_circle = Circle::new((mouse_pos.x, mouse_pos.y), draw_context.mouse_radius);
            let rect = mouse_circle.bounding_box();
            let lims = self.get_rect_limits(&rect);

            let first = self.pair_of(lims.0);
            ctx.begin_path();
            ctx.set_stroke_style(&JsValue::from("red"));
            ctx.move_to(first.0, first.1);
            (lims.0..lims.1).for_each(|i| {
                let (x, y) = self.pair_of(i);
                ctx.line_to(x, y);
            });
            ctx.stroke();

            draw_mouse_range(ctx, mouse_pos, draw_context.mouse_radius);
            ctx.set_fill_style(&JsValue::from("red"));
            ctx.begin_path();
            self.query_distance(mouse_pos, draw_context.mouse_radius, |value| {
                value.draw(ctx, draw_context);
            });
            ctx.fill();
            let order = self.number_of(mouse_pos.x, mouse_pos.y);
            ctx.fill_text(&format!("{order}"), 20.0, 20.0).unwrap();
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

        if let Some(mouse_pos) = draw_context.mouse_pos.as_ref() {
            draw_mouse_range(ctx, mouse_pos, draw_context.mouse_radius);
            ctx.set_fill_style(&JsValue::from("red"));
            ctx.begin_path();
            self.query_distance(mouse_pos, draw_context.mouse_radius, |value| {
                value.draw(ctx, draw_context);
            });
            ctx.fill();
        }
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

fn draw_mouse_range(ctx: &CanvasRenderingContext2d, mouse_pos: &V2, radius: f64) {
    ctx.set_stroke_style(&JsValue::from("red"));
    ctx.begin_path();
    ctx.arc(
        mouse_pos.x,
        mouse_pos.y,
        radius,
        0.,
        std::f64::consts::PI * 2.,
    )
    .unwrap();
    ctx.stroke();
}
