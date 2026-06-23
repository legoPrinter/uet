import init, {message} from "../pkg/uet.js"

async function run() {
    await init();
    var label = document.getElementById("test-label");
    var msg = message(10);
    label.innerText = "message: " + msg;
    console.log("Generated message:", msg);
}

run();