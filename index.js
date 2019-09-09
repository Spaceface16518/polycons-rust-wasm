function timestamp() {
    return window.performance && window.performance.now ? window.performance.now() : new Date().getTime();
}

import('./pkg').then(wasm => {
    const canvas = document.getElementById('display');
    const ctx = canvas.getContext('2d');
    console.log('canvas located');
    const world = wasm.initWorld(40, canvas.width, canvas.height, 100, 150, -100, 100, 1, 3);
    console.log('world loaded');

    let now, dt = 0;
    let last = timestamp();
    let step = 1/60;

    function frame() {
        now = timestamp();
        dt = dt + Math.min(1, (now - last) / 1000);
        while(dt > step) {
            dt = dt - step;
            wasm.generateNodes(world, dt);
            const lines = wasm.generateLines(world);
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            wasm.drawLines(ctx, lines); // draw lines first so they go under
            wasm.drawNodes(ctx, world);
        }
        last = now;
        requestAnimationFrame(frame);
    }

    requestAnimationFrame(frame);
}).catch(console.error);