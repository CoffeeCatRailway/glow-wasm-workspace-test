import init, {mainJs} from "./pkg/wasm.js";

async function run() {
    await init();
}

run().then(() => {
    // mainJs();
    console.log("WASM Loaded");
});