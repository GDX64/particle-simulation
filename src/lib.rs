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
use v2::V2;
use zorder_tree::ZOrderTree;

#[wasm_bindgen]
pub struct CanvasDriven {
    world: Box<dyn ParticleWorld>,
    draw_context: DrawContext,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum TreeType {
    ZOrder,
    Quad,
    RStar,
}

#[wasm_bindgen]
pub struct CanvasDrivenArgs {
    pub width: f64,
    pub height: f64,
    pub particles: usize,
    pub tree_type: TreeType,
}

#[wasm_bindgen]
impl CanvasDrivenArgs {
    pub fn default() -> Self {
        CanvasDrivenArgs {
            width: 800.,
            height: 600.,
            particles: 100,
            tree_type: TreeType::RStar,
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

#[wasm_bindgen]
impl CanvasDriven {
    pub fn new(args: CanvasDrivenArgs) -> CanvasDriven {
        match args.tree_type {
            TreeType::ZOrder => CanvasDriven::_new::<ZOrderTree<Particle>>(args),
            TreeType::Quad => CanvasDriven::_new::<QuadTree<Particle>>(args),
            TreeType::RStar => CanvasDriven::_new::<RStartree<Particle>>(args),
        }
    }

    fn _new<T: GeoQuery<Particle> + Drawable + 'static>(args: CanvasDrivenArgs) -> CanvasDriven {
        let CanvasDrivenArgs {
            width,
            height,
            particles,
            ..
        } = args;
        let gravity = V2::new(0., 30.);
        let mut world = World::<T>::new(V2::new(width, height), gravity);
        world.add_random_particles(particles, random);
        CanvasDriven {
            world: Box::new(world),
            draw_context: DrawContext {
                mouse_pos: None,
                mouse_radius: 50.,
            },
        }
    }

    pub fn evolve(&mut self) {
        self.world.evolve(4);
    }

    pub fn remove_mouse_pos(&mut self) {
        self.draw_context.mouse_pos = None;
        self.world.update_mouse_pos(None, false);
    }

    pub fn update_mouse_pos(&mut self, x: f64, y: f64, is_pressing: bool) {
        self.draw_context.mouse_pos = Some(V2::new(x, y));
        self.world
            .update_mouse_pos(self.draw_context.mouse_pos.clone(), is_pressing);
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

trait ParticleWorld {
    fn evolve(&mut self, n: usize);
    fn draw(&self, ctx: &CanvasRenderingContext2d, draw_context: &DrawContext);
    fn update_mouse_pos(&mut self, mouse_pos: Option<V2>, is_pressing: bool);
}

impl<T> ParticleWorld for World<T>
where
    T: GeoQuery<Particle> + Drawable,
{
    fn evolve(&mut self, n: usize) {
        World::<T>::evolve(self, n);
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d, draw_context: &DrawContext) {
        Drawable::draw(self, ctx, draw_context);
    }

    fn update_mouse_pos(&mut self, mouse_pos: Option<V2>, is_pressing: bool) {
        World::<T>::update_mouse_pos(self, mouse_pos, is_pressing);
    }
}
