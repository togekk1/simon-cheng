import * as loader from '@assemblyscript/loader';

interface wasm_type {
    main: main;
    __getString: Function;
    __allocString: Function;
    click_event: Function;
}

export class main {
    html_global: string = '';
    constructor(_html: string) { }
}

export default class wasm {
    wasm: wasm_type = {} as wasm_type;
    main: main;
    get_string: Function = () => { };
    alloc_string: Function = () => { };
    click_event: Function = () => { };

    constructor() { }

    async load() {
        const { main, __getString, __allocString, click_event }: wasm_type = await loader.instantiateStreaming(fetch('index.wasm'), {
            env: {
                abort(msg, file, line, column) {
                    throw Error("Assertion failed: " + (msg ? "'" + __getString(msg) + "' " : "") + "at " + __getString(file) + ":" + line + ":" + column);
                },
                logi(i: number): void { console.log(i) },
                logf(i: number): void { console.log(i) },
                log(str: number): void { console.log(__getString(str)) },
                render(html: number): void {
                    const div = document.createElement("div");
                    div.innerHTML = __getString(html);
                    const { body }: Document = document;
                    body.firstChild && body.replaceChild(document.createDocumentFragment().appendChild(div), body.firstChild);
                },
                add_event_listener(id: number): void {
                    document.getElementById(__getString(id))?.addEventListener('click', () => { click_event() })
                }
            }
        }) as wasm_type;

        this.main = main;
        this.get_string = __getString;
        this.alloc_string = __allocString;
    }
}

