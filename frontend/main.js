import init, { analyze_image } from './pkg/image_detector.js';

let wasmReady = false;

async function initWasm() {
    await init();
    wasmReady = true;
    console.log("WASM Ready");
}

document.addEventListener("DOMContentLoaded", () => {
    initWasm();

    const dropzone = document.getElementById("dropzone");

    dropzone.addEventListener("dragover", e => {
        e.preventDefault();
    });

    dropzone.addEventListener("drop", async e => {
        e.preventDefault();
        if (!wasmReady) return;

        const file = e.dataTransfer.files[0];
        const bytes = new Uint8Array(await file.arrayBuffer());

        const result = analyze_image(bytes);

        document.getElementById("score").textContent =
            (result.ai_probability * 100).toFixed(1) + "% AI probability";
    });

    if ("serviceWorker" in navigator) {
        navigator.serviceWorker.register("sw.js");
    }
});
