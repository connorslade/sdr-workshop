<script lang="ts">
    import { onMount } from "svelte";
    import { WebglPlot, WebglLine, ColorRGBA } from "webgl-plot";

    export let style = "";
    export let cssClass = "";
    export let init: (width: number, lines: WebglLine[]) => void;
    export let callback: (lines: WebglLine[]) => void;

    let lines: WebglLine[] = [];

    let canvas: HTMLCanvasElement;
    onMount(() => {
        const devicePixelRatio = window.devicePixelRatio || 1;
        canvas.width = canvas.clientWidth * devicePixelRatio;
        canvas.height = canvas.clientHeight * devicePixelRatio;

        init(canvas.width, lines);

        const plot = new WebglPlot(canvas);
        for (let line of lines) plot.addLine(line);

        function update() {
            callback(lines);
            plot.update();
            requestAnimationFrame(update);
        }

        requestAnimationFrame(update);
    });
</script>

<canvas bind:this={canvas} class={cssClass} {style}></canvas>
