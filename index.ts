import init, {
  CanvasDriven,
  CanvasDrivenArgs,
  TreeType,
  Pendulum,
} from "./pkg/fluid";

init().then(async () => {
  const canvas = document.createElement("canvas");
  document.body.appendChild(canvas);
  canvas.width = canvas.offsetWidth * devicePixelRatio;
  canvas.height = canvas.offsetHeight * devicePixelRatio;
  drawPendulum(canvas);
});

async function drawPendulum(canvas: HTMLCanvasElement) {
  const pendulum = Pendulum.new();
  const ctx = canvas.getContext("2d")!;
  while (true) {
    pendulum.evolve();
    ctx.save();
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.translate(canvas.width / 2, canvas.height / 2);
    ctx.scale(1, -1);
    pendulum.draw(ctx);
    ctx.restore();
    await raf();
  }
}

function raf() {
  return new Promise((resolve) => requestAnimationFrame(resolve));
}

async function drawParticles(canvas: HTMLCanvasElement) {
  const ctx = canvas.getContext("2d")!;
  const args = CanvasDrivenArgs.default();
  args.width = canvas.width;
  args.height = canvas.height;
  args.tree_type = TreeType.RStar;
  args.particles = 5_000;
  const driven = CanvasDriven.new(args);
  const mousePos = { x: 0, y: 0, isPresing: false };
  canvas.addEventListener("mousemove", (e) => {
    mousePos.x = e.offsetX * devicePixelRatio;
    mousePos.y = e.offsetY * devicePixelRatio;
  });
  canvas.addEventListener("mouseleave", (e) => {
    driven.remove_mouse_pos();
  });
  canvas.addEventListener("mousedown", (e) => {
    mousePos.isPresing = true;
  });
  canvas.addEventListener("mouseup", (e) => {
    mousePos.isPresing = false;
  });
  while (true) {
    driven.update_mouse_pos(mousePos.x, mousePos.y, mousePos.isPresing);
    driven.evolve(1);
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    driven.draw(ctx);
    await raf();
  }
}
