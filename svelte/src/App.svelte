<script lang="ts">
    import { greet, render } from "wasm";
    let canvas: HTMLCanvasElement = document.createElement("canvas");
    let [width, height] = [1920, 800];
    let [start, end] = [0, 0];
    $: delta = end-start;
    function click() {
        canvas.width = width;
        canvas.height = height;
        const ctx = canvas.getContext("2d");
        ctx.imageSmoothingEnabled = false;
        const d = ctx.createImageData(width, height);
        start = performance.now();
        const data = render(width, height);
        end = performance.now();
        d.data.set(data);
        ctx.putImageData(d, 0, 0);
    }
</script>

<main>
    <input bind:value={width} />
    <input bind:value={height} />
    <code>{delta} ms === {1000/delta} fps</code>
    <button on:click={click}>please</button>
    <div><canvas bind:this={canvas} {width} {height} /></div> 
    <div><canvas width={1920} height={1080}></canvas></div>
</main>

<style lang="sass">
    canvas
        height: 100%
</style>
