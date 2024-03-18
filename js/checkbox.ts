import Cookies from "js-cookie";

class CookieCheckbox extends HTMLElement {
  static observedAttributes = ["color", "size"];
  constructor() {
    super();
  }

  connectedCallback() {}
}

/**
 * Apply the nsfw status
 *
 * @param shown if nsfw is to be shown
 */
function applyNsfw(shown: boolean) {
  let body = document.getElementsByTagName("body")[0];
  if (shown) {
    body.classList.add("show-nsfw");
  } else {
    body.classList.remove("show-nsfw");
  }
}

/**
 * Record the nsfw status
 *
 * @param shown if nsfw is to be shown
 */
function recordNsfw(shown: boolean) {
  Cookies.set("nsfw", shown ? "1" : "0", { sameSite: "lax", path: "/" });
}

function getNsfw(): boolean {
  return Cookies.get("nsfw") == "1";
}

export function initNsfw() {
  const initialState = getNsfw();
  const checkbox = document.getElementById("nsfw-switch") as HTMLInputElement;
  if (checkbox) {
    checkbox.oninput = () => {
      setNsfw(!getNsfw());
    };
  }
  setNsfw(initialState);
}

export function setNsfw(shown: boolean) {
  console.log("horny mode " + (shown ? "on" : "off"));
  recordNsfw(shown);
  applyNsfw(shown);

  const checkbox = document.getElementById("nsfw-switch") as HTMLInputElement;
  if (checkbox) {
    checkbox.value = shown ? "1" : "0";
  }
}
