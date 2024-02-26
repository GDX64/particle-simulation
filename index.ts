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
  const N = canvas.offsetWidth / 5;
  const arrPendulum = Array.from({ length: N }, (_, i) => {
    return Pendulum.new(15, 0.01);
  });
  const ctx = canvas.getContext("2d")!;
  const matrix = new DOMMatrix();
  matrix.translateSelf(canvas.width / 2, canvas.height / 10);
  matrix.scaleSelf(devicePixelRatio, -devicePixelRatio);
  canvas.addEventListener("pointermove", (e) => {
    const { offsetX: x, offsetY: y } = e;
    const transformed = matrix.inverse().transformPoint({ x, y });
    arrPendulum.forEach((p, i) =>
      p.update_fixed_ball(transformed.x + (i - N / 2) * 5, transformed.y)
    );
  });
  while (true) {
    arrPendulum.forEach((p) => p.evolve(0.016, 1));
    ctx.save();
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.setTransform(matrix);
    arrPendulum.forEach((p) => p.draw(ctx));
    ctx.restore();
    await raf();
    // await awaitClick();
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
    driven.evolve(4);
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    driven.draw(ctx);
    await raf();
  }
}

async function awaitClick() {
  return new Promise((resolve) => {
    document.addEventListener("click", resolve, { once: true });
  });
}
