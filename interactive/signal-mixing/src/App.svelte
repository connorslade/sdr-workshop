<script lang="ts">
    import { ColorRGBA } from "webgl-plot";
    import Input from "./Input.svelte";
    import Derived from "./Derived.svelte";

    let signal_1: number[] = [];
    let signal_2: number[] = [];
</script>

<main>
    <h1>Interactive Signal Mixing</h1>

    <div class="container">
        <Input
            name="Signal 1"
            color={new ColorRGBA(1.0, 0.0, 0.0, 1.0)}
            data={signal_1}
        />
        <Input
            name="Signal 2"
            color={new ColorRGBA(0.0, 0.0, 1.0, 1.0)}
            data={signal_2}
        />
        <Derived
            name="Mixed (S₁ ⊗ S₂)"
            color={new ColorRGBA(1.0, 0.0, 1.0, 1.0)}
            callback={(line) => {
                for (let i = 0; i < line.numPoints; i++)
                    line.setY(i, signal_1[i] * signal_2[i]);
            }}
        />
    </div>
</main>

<style>
    h1 {
        text-align: center;
    }

    .container {
        display: grid;
        gap: 1rem;
    }

    :global(canvas) {
        min-height: 170px;
    }
</style>
