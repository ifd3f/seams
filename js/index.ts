import { CatChatbox } from "./chatbox.ts";
import { greet } from "./console.ts";

document.addEventListener("DOMContentLoaded", function onLoad() {
  customElements.define("cat-chatbox", CatChatbox);
  greet();
});
