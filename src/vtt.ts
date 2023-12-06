import "./styles.css";
import VttView from "./VttView.svelte";

const app = new VttView({
  target: document.getElementById("app"),
});

export default app;
