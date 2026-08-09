const PROCESSOR_URL = new URL("./audio_processor.js", import.meta.url);

export default class GBCAudio {
  constructor() {
    this.ctx = new (window.AudioContext || window.webkitAudioContext)();
    this.workletNode = null;
    this.deltas = new Float64Array(64);
    this.deltaIndex = 0;
    this.deltaCount = 0;
    this._initPromise = null;
  }

  async _init() {
    await this.ctx.audioWorklet.addModule(PROCESSOR_URL);
    this.workletNode = new AudioWorkletNode(this.ctx, "gbc-audio-processor", {
      numberOfInputs: 0,
      numberOfOutputs: 1,
      outputChannelCount: [2],
    });
    this.workletNode.connect(this.ctx.destination);
  }

  play() {
    if (!this._initPromise) {
      this._initPromise = this._init();
    }
    if (this.ctx.state === "suspended") {
      this.ctx.resume();
    }
  }

  pushSamples(samples) {
    if (this.workletNode) {
      this.workletNode.port.postMessage(samples);
    }
  }

  samplesToPull(deltaMs) {
    // Smooth delta over 64 frames to avoid jitter-driven over/under-pulls
    this.deltas[this.deltaIndex] = deltaMs;
    this.deltaIndex = (this.deltaIndex + 1) % this.deltas.length;
    if (this.deltaCount < this.deltas.length) this.deltaCount++;

    let sum = 0;
    for (let i = 0; i < this.deltaCount; i++) sum += this.deltas[i];
    const smoothedDeltaSeconds = sum / this.deltaCount / 1000;

    return Math.floor(this.sampleRate() * smoothedDeltaSeconds);
  }

  sampleRate() {
    return this.ctx.sampleRate;
  }
}
