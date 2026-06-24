import init, {start_rust_program} from "../pkg/uet.js"

async function run() {
    await init();

    start_rust_program();
}

run();