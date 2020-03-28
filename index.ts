import * as loader from '@assemblyscript/loader';
(async (): Promise<void> => {
    try {
        const wasm = await loader.instantiateStreaming(fetch('build/optimized.wasm'), { /* imports */ })
        console.log(wasm);
    }
    catch (err) {
        console.error(err);
    }
})
