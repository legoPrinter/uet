import init, {render_frame} from "../pkg/uet.js"

async function run() {
    await init();

    const canvas = document.querySelector("canvas");
    const width = 200;
    const height = 200;
    canvas.width = width;
    canvas.height = height;
    var buffer = render_frame();
    const image = new ImageData(
        new Uint8ClampedArray(buffer),
        width,
    );

    const ctx = canvas.getContext("2d");

    ctx.putImageData(image, 0, 0);
}

run();