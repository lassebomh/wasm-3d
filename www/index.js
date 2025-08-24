import init from "./pkg/wasm_3d.js";
const { render } = await init();
/**
 * @returns {never}
 */
function fail() {
  throw new Error();
}

const canvas = /** @type {HTMLCanvasElement} */ (document.getElementById("canvas") ?? fail());

const ctx = canvas.getContext("2d") ?? fail();

let width = 0;
let height = 0;

const observer = new ResizeObserver((entries) => {
  for (const entry of entries) {
    width = entry.contentRect.width;
    height = entry.contentRect.height;
    canvas.width = width;
    canvas.height = height;
    render(ctx, width, height, performance.now());
  }
});
observer.observe(document.body);

/**
 *
 * @param {number} t
 */
function loop(t) {
  if (width !== 0 && height !== 0) render(ctx, width, height, t);
  requestAnimationFrame(loop);
}

loop(performance.now());
