<script lang="ts">
    import { ColorRGBA, WebglLine } from "webgl-plot";
    import LinePlot from "./lib/LinePlot.svelte";
    import FFT from "fft.js";

    export let name: string;
    export let color: ColorRGBA;
    export let callback: (line: WebglLine) => void;

    let fft: number[];

    function nextPowerOfTwo(n) {
        return Math.pow(2, Math.ceil(Math.log2(n)));
    }
</script>

<div class="container">
    <LinePlot
        init={(width, lines) => {
            let line = new WebglLine(color, width);
            fft = new Array(width);
            line.arrangeX();
            lines.push(line);
        }}
        callback={(lines) => {
            let line = lines[0];
            callback(line);

            // let real = [];
            // let size = nextPowerOfTwo(line.numPoints);
            // for (let i = 0; i < size; i++) real.push(line.getY(i) ?? 0);

            // let f = new FFT(size);
            // let out = f.createComplexArray();
            // f.realTransform(out, real);

            // fft.length = out.length / 2;
            // for (let i = 0; i < out.length / 4; i++) fft[i] = out[i * 2] - 1.0;
        }}
    />

    <div>
        <h2>{name}</h2>
    </div>

    <LinePlot
        init={(width, lines) => {
            let line = new WebglLine(color, width);
            line.arrangeX();
            lines.push(line);
        }}
        callback={(lines) => {
            let line = lines[0];
            for (let i = 0; i < line.numPoints; i++) {
                let idx = Math.floor((i * fft.length) / line.numPoints / 50.0);
                line.setY(i, fft[idx]);
            }
        }}
    />
</div>

<style>
    .container {
        display: grid;
        gap: 1rem;
        grid-template-columns: 2fr 1fr;
    }

    .container > div,
    :global(canvas) {
        border: 1px solid black;
        border-radius: 0.5rem;
    }

    .container > div {
        padding: 10px;
        grid-row: span 2;
    }

    :global(canvas) {
        width: 100%;
        height: 100%;
    }
</style>
