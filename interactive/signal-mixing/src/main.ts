import { mount } from "svelte";
import App from "./App.svelte";

export const TAU: number = 2.0 * Math.PI;

const app = mount(App, {
  target: document.getElementById("app")!,
});

export default app;
