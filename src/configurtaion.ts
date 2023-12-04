import "./styles.css";
import App from "./App.svelte";
import Configuration from "./Configuration.svelte";

const app = new Configuration({
  target: document.getElementById("app"),
});

export default app;
