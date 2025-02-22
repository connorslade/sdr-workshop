<script lang="ts">
    import { ColorRGBA, WebglLine } from "webgl-plot";
    import LinePlot from "./lib/LinePlot.svelte";

    export let name: string;
    export let color: ColorRGBA;
    export let data: number[] | null = null;

    const TAU: number = 2.0 * Math.PI;

    let frequency: number;
    let phase: number = 0;
    let amplitude: number;
</script>

<div class="container">
    <LinePlot
        init={(width, lines) => {
            let line = new WebglLine(color, width);
            line.arrangeX();
            lines.push(line);
        }}
        callback={(lines) => {
            let line = lines[0];

            if (data != null && data.length != line.numPoints)
                data.length = line.numPoints;

            for (let i = 0; i < line.numPoints; i++) {
                let t = (i / line.numPoints) * TAU;
                let y = amplitude * Math.sin(t * frequency + phase * TAU);

                line.setY(i, y);
                if (data != null) data[i] = y;
            }
        }}
    />
    <div>
        <h2>{name}</h2>
        <div class="controls">
            <label for="frequency">Frequency</label>
            <input
                type="range"
                min="0"
                max="10"
                step="0.01"
                bind:value={frequency}
            />

            <label for="phase">Phase Shift</label>
            <input
                type="range"
                id="phase"
                min="0"
                max="1"
                step="0.01"
                bind:value={phase}
            />

            <label for="amplitude">Amplitude</label>
            <input
                type="range"
                id="amplitude"
                min="0"
                max="1"
                step="0.01"
                bind:value={amplitude}
            />
        </div>
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

    .controls {
        display: grid;
        grid-template-columns: 1fr 3fr;
    }
</style>
