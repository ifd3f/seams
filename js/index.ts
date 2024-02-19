import { CatChatbox } from "./chatbox.ts";
import { greet } from "./console.ts";
import { playRandomXPSound } from "./xpsounds.ts";

declare global {
  interface Window {
    playRandomXPSound: () => void;
  }
}

window.playRandomXPSound = playRandomXPSound;

document.addEventListener("DOMContentLoaded", function onLoad() {
  customElements.define("cat-chatbox", CatChatbox);
  greet();
});
