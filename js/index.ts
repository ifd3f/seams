import { CatChatbox } from "./chatbox.ts";
import { greet } from "./console.ts";
import { initNsfw, setNsfw } from "./nsfw";
import { playRandomXPSound } from "./xpsounds.ts";

declare global {
  interface Window {
    playRandomXPSound: () => void;
    setNsfw: (shown: boolean) => void;
  }
}

window.playRandomXPSound = playRandomXPSound;
window.setNsfw = setNsfw;

document.addEventListener("DOMContentLoaded", function onLoad() {
  customElements.define("cat-chatbox", CatChatbox);
  initNsfw();
  greet();
});
