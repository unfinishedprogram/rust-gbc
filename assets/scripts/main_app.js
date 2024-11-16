import init, { Application } from "/application.js";
await init();

import AudioContext from "./audio_context.js";

const ctx = document.querySelector("#screen").getContext("2d");
const app = new Application();

let audio = new AudioContext();

function run(time) {
  audio.play();

  let delta_t = app.step_lcd_frame(time);
  let screen_image = app.render_screen();
  let to_pull = audio.samplesToPull(delta_t);
  let samples = app.pull_audio_samples(to_pull);

  audio.pushSamples(samples);
  ctx.putImageData(screen_image, 0, 0);
}

// const loop = (time) => {
//   requestAnimationFrame(loop);
// };

setInterval(() => run(performance.now()), 16);

// requestAnimationFrame(loop);

// setInterval(loop, 16);

export default app;
