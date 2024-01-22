import "./styles.css";
import VttView from "./lib/VttView.svelte";

const app = new VttView({
  target: document.getElementById("app"),
});

export default app;
