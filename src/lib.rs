use quad_tree::QuadTree;
use rstar_tree::RStartree;
use tree_drawings::{DrawContext, Drawable};
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
mod particle;
mod quad_tree;
mod rstar_tree;
mod tree_drawings;
mod v2;
mod zorder_tree;
use particle::*;
use v2::*;
use zorder_tree::ZOrderTree;

#[wasm_bindgen]
pub struct CanvasDriven {
    world: World<QuadTree<Particle>>,
    draw_context: DrawContext,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

#[wasm_bindgen]
impl CanvasDriven {
    pub fn new(width: f64, height: f64, particles: usize) -> CanvasDriven {
        let mut world = World::new(V2::new(width, height), V2::new(0., 30.));
        world.add_random_particles(particles, random);
        CanvasDriven {
            world,
            draw_context: DrawContext { mouse_pos: None },
        }
    }

    pub fn evolve(&mut self) {
        self.world.evolve(4);
    }

    pub fn is_pressing_mouse(&mut self, is_pressing: bool) {
        self.world.is_pressing_mouse = is_pressing;
    }

    pub fn remove_mouse_pos(&mut self) {
        self.draw_context.mouse_pos = None;
        self.world.update_mouse_pos(None);
    }

    pub fn update_mouse_pos(&mut self, x: f64, y: f64) {
        self.draw_context.mouse_pos = Some(V2::new(x, y));
        self.world
            .update_mouse_pos(self.draw_context.mouse_pos.clone());
    }

    pub fn draw(&self, ctx: JsValue) {
        self._draw(ctx);
    }
}

impl CanvasDriven {
    fn _draw(&self, ctx: JsValue) -> Option<()> {
        let ctx: CanvasRenderingContext2d = ctx.dyn_into().ok()?;
        self.world.draw(&ctx, &self.draw_context);
        // web_sys::console::log_1(&"drawn".into());
        Some(())
    }
}
