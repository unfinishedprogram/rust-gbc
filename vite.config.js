import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import { defineConfig } from 'vite'
export default defineConfig({
  plugins: [
    /**
     * Since 2.x version of this plugin, the `filter` option has been removed.
     * 
     * For 1.x (with Vite 2.x):
     *   By default ALL `.wasm` imports will be transformed to WebAssembly ES module.
     *   You can also set a filter (function or regex) to match files you want to transform.
     *   Other files will fallback to Vite's default WASM loader (i.e. You need to call `initWasm()` for them). 
     *   ```js
     *   wasm({
     *     filter: /syntect_bg.wasm$/
     *   })
     *   ```
     */
    wasm(),
    topLevelAwait()
  ]
});