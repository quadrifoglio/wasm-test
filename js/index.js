const crate = import('../pkg/wasm_test');

document.getElementById('btn').onclick = () => {
    let value = document.getElementById('color').value;

    if (value) {
        let r = parseInt(value.substring(1, 3), 16);
        let g = parseInt(value.substring(3, 5), 16);
        let b = parseInt(value.substring(5, 7), 16);

        if (r !== NaN && g !== NaN && b !== NaN) {
            crate
                .then(m => m.change_color(r, g, b))
                .catch(console.error);
        }
    }
}
