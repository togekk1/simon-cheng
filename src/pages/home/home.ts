import wasm from '../../wasm';
// @ts-ignore
import html from './home.html';
// @ts-ignore
import './home.less';
import '../../assets/fonts/stylesheet.css'

export default class home {
    constructor(
        private wasm: wasm
    ) {
        const { alloc_string, main }: wasm = wasm;
        const main_new = new main();
        main_new.html = alloc_string(html);
        main_new.render();
        setTimeout(() => {
            main_new.on_enter = true;
            main_new.render();
        }, 2000);
        setTimeout(() => {
            main_new.disabled = false;
            main_new.render();
        }, 5300);
    }
}