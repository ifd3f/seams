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

export class CatChatbox extends HTMLElement {
  constructor() {
    super();
  }

  connectedCallback() {
    const box = new ActiveCatChatbox(this);
    box.queueMessage({
      user: assistant,
      text: "Hello! I'm your virtual assistant. Let me know if you need anything!",
    });
  }
}

class ActiveCatChatbox {
  chatTemplate: HTMLTemplateElement;

  chatlog: HTMLElement;
  chatform: HTMLFormElement;
  ellipsis: HTMLElement;

  constructor(private root: HTMLElement) {
    const boxTemplate = document.getElementById(
      "catchat-template"
    ) as HTMLTemplateElement;
    this.chatTemplate = document.getElementById(
      "catchat-message-template"
    ) as HTMLTemplateElement;

    root.appendChild(boxTemplate.content.cloneNode(true));

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
      this.queueMessage({ user: assistant, text: generateNya() });
    };
  }

  addMessage(message: Message) {
    const node = this.chatTemplate.content.cloneNode(true) as HTMLElement;

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

  queueMessage(message: Message) {
    setTimeout(() => {
      this.setEllipsis(true);
      this.scrollToBottom();
      setTimeout(() => {
        this.setEllipsis(false);
        this.addMessage(message);
        this.scrollToBottom();
      }, 1000);
    }, 1000);
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
