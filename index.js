import("./wasm_mdn").then(module => {
    console.log(module.hello());
});