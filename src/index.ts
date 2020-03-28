import wasm from './wasm';
// @ts-ignore
import wasm_html from './pages/home/home.html';

class app {
    wasm: wasm = new wasm();
    constructor() { }

    async init() {
        await this.wasm.load();
        const { alloc_string }: wasm = this.wasm,
            click_event_str: string = wasm_html.match(/\(click\)="(.*)"/gm)[0],
            click_event: string = click_event_str.replace('(click)=\"', '').replace('\"', ''),
            wasm_html_new = wasm_html.replace(click_event_str, '');
        this.wasm.main(alloc_string(wasm_html_new), alloc_string(click_event));
    }
}

new app().init();