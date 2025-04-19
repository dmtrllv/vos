import ReactDOM from "react-dom/client";
import { App } from "./App";

ReactDOM.createRoot(document.getElementById("root")!).render(
  	<App />
);

fetch("/api/test").then(r => r.text()).then(console.log)