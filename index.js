import init, { CanvasDriven } from "./pkg/fluid.js";

init().then(async () => {
  const canvas = document.createElement("canvas");
  document.body.appendChild(canvas);
  canvas.width = canvas.offsetWidth * devicePixelRatio;
  canvas.height = canvas.offsetHeight * devicePixelRatio;
  const driven = CanvasDriven.new(canvas.width, canvas.height, 1000);
  const ctx = canvas.getContext("2d");
  const mousePos = { x: 0, y: 0 };
  canvas.addEventListener("mousemove", (e) => {
    mousePos.x = e.offsetX * devicePixelRatio;
    mousePos.y = e.offsetY * devicePixelRatio;
  });
  canvas.addEventListener("mouseleave", (e) => {
    driven.remove_mouse_pos();
  });
  canvas.addEventListener("mousedown", (e) => {
    driven.is_pressing_mouse(true);
  });
  canvas.addEventListener("mouseup", (e) => {
    driven.is_pressing_mouse(false);
  });
  while (true) {
    driven.update_mouse_pos(mousePos.x, mousePos.y);
    // driven.evolve();
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    driven.draw(ctx);
    await raf();
  }
});

function raf() {
  return new Promise((resolve) => requestAnimationFrame(resolve));
}
