<script lang="ts">
    import { ColorRGBA } from "webgl-plot";

    import Input from "./Input.svelte";
    import Derived from "./Derived.svelte";
    import { InputState } from "./types";
    import { TAU } from "./main";

    let signal_1: InputState = new InputState();
    let signal_2: InputState = new InputState();
</script>

<main>
    <h1>Interactive Signal Mixing</h1>

    <div class="container">
        <Input
            name="Signal 1 (S₁)"
            color={new ColorRGBA(1.0, 0.0, 0.0, 1.0)}
            state={signal_1}
        />
        <Input
            name="Signal 2 (S₂)"
            color={new ColorRGBA(0.0, 0.0, 1.0, 1.0)}
            state={signal_2}
        />
        <Derived
            name="Mixed (S₁ ⊗ S₂)"
            n={3}
            colors={[
                new ColorRGBA(1.0, 0.0, 1.0, 1.0),
                new ColorRGBA(0.0, 0.0, 0.0, 0.75),
                new ColorRGBA(0.0, 0.0, 0.0, 0.75),
            ]}
            callback={(line) => {
                for (let i = 0; i < line[0].numPoints; i++)
                    line[0].setY(i, signal_1.data[i] * signal_2.data[i]);

                function updateLine(idx: number, sign: number) {
                    for (let i = 0; i < line[idx].numPoints; i++) {
                        let t = (i / line[idx].numPoints) * TAU;

                        let a = -signal_1.amplitude * signal_2.amplitude * 0.5 * sign;
                        let f = signal_1.frequency + sign * signal_2.frequency;
                        let p = signal_1.phase + sign * signal_2.phase;
                        let y = a * Math.cos(t * f + p * TAU);
                        line[idx].setY(i, y);
                    }
                }

                updateLine(1, -1); // f₁-f₂
                updateLine(2, +1);  // f₁+f₂
            }}
        >
            The grey lines show the new frequency components at the sum and
            difference of the input signal frequencies. (f₁±f₂)
        </Derived>
        <Derived
            name="Summed (S₁ + S₂)"
            colors={[new ColorRGBA(1.0, 0.0, 1.0, 1.0)]}
            callback={(line) => {
                for (let i = 0; i < line[0].numPoints; i++)
                    line[0].setY(i, signal_1.data[i] + signal_2.data[i]);
            }}
        ></Derived>
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
