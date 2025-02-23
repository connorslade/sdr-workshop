<script lang="ts">
    import { ColorRGBA, WebglLine } from "webgl-plot";
    import LinePlot from "./LinePlot.svelte";

    export let name: string;
    export let colors: ColorRGBA[];
    export let callback: (line: WebglLine[]) => void;
    export let n: number = 1;
</script>

<div class="container">
    <LinePlot
        init={(width, lines) => {
            for (let i = 0; i < n; i++) {
                let line = new WebglLine(colors[i], width);
                line.arrangeX();
                lines.push(line);
            }
        }}
        callback={(lines) => {
            callback(lines);
        }}
    />

    <div>
        <h2>{name}</h2>
        <slot />
    </div>
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
    }

    :global(canvas) {
        width: 100%;
        height: 100%;
    }
</style>
