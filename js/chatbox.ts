type User = {
  name: string;
  color: string;
  avatar?: string;
};

const assistant: User = {
  name: "Virtual Assistant",
  color: "red",
  avatar:
    "https://s3.us-west-000.backblazeb2.com/nyaabucket/691f771b94261483884dd6621b924f15794bb4233e4a8923115b57072ead6412.jpg",
};

const you: User = {
  name: "You",
  color: "blue",
  avatar:
    "https://s3.us-west-000.backblazeb2.com/nyaabucket/bd7da12645eb0d1daf241c2c457851c4f612bb2b1a4d952b3a7af0ae07ffbfec/lego-yoda.png",
};

type Message = { user: User; text: string };

/**
 * A custom element that has a chatbox you can talk to.
 *
 * It is extremely smart.
 */
export class CatChatbox extends HTMLElement {
  constructor() {
    super();
  }

  connectedCallback() {
    const box = new ActiveCatChatbox(this);
    box.queueMessage({
      message: {
        user: assistant,
        text: "Hello! I'm your virtual assistant. Let me know if you need anything!",
      },
      scrollToBottom: false,
    });
  }
}

type QueueMessageParams = {
  /**
   * The message to insert into the chatbox
   */
  message: Message;
  /**
   * Whether or not to scroll to the bottom of the chat log
   */
  scrollToBottom?: boolean;
  /**
   * Time in ms to wait before the "X is typing..." animation to start
   */
  timeToStartTyping?: number;
  /**
   * Time in ms after the "X is typing..." animation to wait before sending the message.
   */
  typingTime?: number;
};

/**
 * The actual, active state of the chatbox.
 *
 * Only created after the element is connected to the DOM.
 */
class ActiveCatChatbox {
  chatlog: HTMLElement;
  chatform: HTMLFormElement;
  ellipsis: HTMLElement;

  constructor(private root: HTMLElement) {
    const templates = getTemplates();

    root.appendChild(templates.chatBox.content.cloneNode(true));

    this.chatlog = root.getElementsByClassName(
      "catchat-message-log"
    )[0] as HTMLElement;
    this.chatform = root.getElementsByClassName(
      "catchat-form"
    )[0] as HTMLFormElement;
    this.ellipsis = root.getElementsByClassName(
      "catchat-ellipsis"
    )[0] as HTMLElement;

    this.chatform.onsubmit = (ev) => {
      ev.preventDefault();

      const data = new FormData(this.chatform);
      let message = data.get("input")?.toString().trim();
      if (!(message && message.length > 0)) {
        return;
      }

      this.addMessage({ user: you, text: message });
      this.chatform.reset();
      this.scrollToBottom();
      this.queueMessage({
        message: { user: assistant, text: generateNya() },
        scrollToBottom: true,
      });
    };
  }

  addMessage(message: Message) {
    const templates = getTemplates();
    const node = templates.chatEntry.content.cloneNode(true) as HTMLElement;

    const userElement = node.querySelector(".user") as HTMLElement;
    userElement.style.color = message.user.color;
    userElement.innerText = message.user.name;

    (node.querySelector(".text") as HTMLElement).innerText = message.text;

    if (message.user.avatar) {
      const img = new Image();
      img.src = message.user.avatar;
      (node.querySelector(".avatar") as HTMLElement).appendChild(img);
    }

    this.chatlog.append(node);
  }

  queueMessage({
    message,
    scrollToBottom = true,
    timeToStartTyping = 1000,
    typingTime = 1000,
  }: QueueMessageParams) {
    setTimeout(() => {
      this.setEllipsis(true);
      setTimeout(() => {
        this.setEllipsis(false);
        this.addMessage(message);
        if (scrollToBottom) {
          this.scrollToBottom();
        }
      }, typingTime);
    }, timeToStartTyping);
  }

  setEllipsis(shown: boolean) {
    this.ellipsis.hidden = !shown;
  }

  scrollToBottom() {
    const c = this.chatlog.lastElementChild;
    if (c) {
      (c as HTMLElement).scrollIntoView();
    }
  }
}

function generateNya() {
  const clauses = Math.random() * Math.random() * 4;
  const sentence = [];

  let firstWord = true;

  for (let c = 0; c < clauses; c++) {
    const clauseLength = Math.random() * 5 + 1;
    const clause = [];
    for (let w = 0; w < clauseLength; w++) {
      const word = [];
      const wordLength = Math.random() * Math.random() * Math.random() * 3;
      for (let s = 0; s < wordLength; s++) {
        if (firstWord) {
          word.push("Nya");
          firstWord = false;
        } else {
          word.push("nya");
        }
      }
      clause.push(word.join(""));
    }
    sentence.push(clause.join(" "));
  }

  return sentence.join(", ") + ".";
}

type Templates = {
  chatBox: HTMLTemplateElement;
  chatEntry: HTMLTemplateElement;
};

var _templates: Templates | null = null;
function getTemplates(): Templates {
  if (_templates) {
    return _templates;
  }

  const chatBox = document.createElement("template");
  chatBox.innerHTML = `
    <div class="catchat-container">
      <div style="flex-grow: 1; overflow: scroll; border: 1px solid gray; margin-bottom: 3px; ">
          <div class="catchat-message-log"></div>
          <p class="catchat-ellipsis" hidden>Assistant is typing...</p>
      </div>
      <div>
          <form class="catchat-form">
              <input class="catchat-input" name="input">
              <input type="submit" value="Send">
          </form>
      </div>
  </div>`;

  const chatEntry = document.createElement("template");
  chatEntry.innerHTML = `
  <div class="catchat-entry">
      <div class="avatar" style="flex-shrink: 0"></div>
      <div style="flex-grow: 1">
          <p class="user"></p>
          <p class="text"></p>
      </div>
  </div>`;

  _templates = { chatBox, chatEntry };
  return _templates;
}
