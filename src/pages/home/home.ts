import wasm from '../../wasm';
// @ts-ignore
import html from './home.html';
// @ts-ignore
import './home.less';

export default class home {
    constructor(
        private wasm: wasm
    ) {
        const { alloc_string, main }: wasm = wasm;
        new main(alloc_string(html));
    }
}