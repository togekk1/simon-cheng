import * as loader from '@assemblyscript/loader';
(async (): Promise<void> => {
    const { add } = await loader.instantiateStreaming(fetch('optimized.wasm'))
    console.log(add(1, 2), 3);
})();
