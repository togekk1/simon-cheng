import wasm from './wasm';
import home from './pages/home/home';

class app {
    wasm: wasm = new wasm();
    constructor() { }

    async init() {
        await this.wasm.load();
        new home(this.wasm);
    }
}

new app().init();