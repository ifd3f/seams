const sounds = [
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/11948fb0d3bdb64d53129c99aca1c59a312080f356dd81104b380e3f4a79a6c1/chimes.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/b107a3c054def993e46e821ec07ab3522510b777a2662fe0607e1ae99600c014/chord.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/9bdba20dd09c6a056cf25ce4565e9ade17caf79adc661a100612c6d8177476c4/ding.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/b825d69c694f151ca3df993ccccac195d411dbb89c4a70f6852ae108ba86f112/flourish.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/28bd0c01f7643326d6450cb6e4d7d9c35e468ff30ee6caf400ee006d044a34e0/notify.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/ae085632359bb38aa0b1e8c30bf8eaca81315e51948a11c11166656fc82a19c8/onestop.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/7c9b2d650c7f89004bdad1ad3ab1d42610cf990efed9cdeec7ec645a90c0049d/recycle.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/5fa5c82bf3a56d26c6b998fb08ff61e36ddb34a4d2f8ca8f10effcac7eb737ba/ringin.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/facd869452e6c5cc25008d51763a3f2b1088993154def9ba21f47d7b3c7f6436/ringout.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/afb648fecf5c2b64de95faf11f1a61e7f7db3c5e4b741a5e57737fa96c62bfa6/start.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/304b24373d1b68c6f14126b3fb299b88c2374c1e6fcce2e704dfd23ed6d4945d/tada.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/bcade8586ed02ea600f2a5904d078353e19e8e580cca207da21d6c871ce6753f/town.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/7af388355a2820aa30491b7439d1d862f97e54cffc3053d5e1f42c7e982d6259/Windows Feed Discovered.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/78b2cc70b487cae6b43730a8039d8bea1f8f69cd798aadd6d43cff0dcda72418/Windows Information Bar.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/082a9add8c6535e5431c81c1eb0b2622e0040fe4b0f07a0c73f7157235c94d3f/Windows Navigation Start.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/ed4c085f1f00877c074c77e0a11b3193bceab4c33be865d66e27dbd17291b08a/Windows Pop-up Blocked.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/bbb8962171b168df1d6daab75e1d931814af8661004874916b89b745f8056c4e/Windows XP Balloon.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/669b55a0ee18f8341499e1277c28fadfce8a690957253626a42f1cbb877c858d/Windows XP Battery Critical.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/03157f00f105352114596bc5b100369e3a025586a6d62cfb5f9076fb73954b87/Windows XP Battery Low.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/0b98766815f77012eceac51a5ec0414869f849b2259c9bc4277f25368f5d16fb/Windows XP Critical Stop.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/643ff0d1cfe2699d72746520bf74bf7fcbcb3194810af493e5e1f934c70d9f2d/Windows XP Default.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/65e4c320edd8947e2a947f6b90911260d126fff805c502e18e98a6b785beec1a/Windows XP Ding.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/1898903461e958682034705903d75b684ae33c899f9688fe5d32dd4f04064d34/Windows XP Error.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/eabcb5cf8090f9e9725a70fd987f7621a06be809b8bd3699fe11066a76da5986/Windows XP Exclamation.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/cc3db26249693c98c8059a73c79c70722b9e331b06ac0fcaf676b39ee1c4b61b/Windows XP Hardware Fail.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/a2c39514a291b06c568bc474a3a897e711ca53b371567b1f5a428f140bb16e96/Windows XP Hardware Insert.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/4a13cdd20a54066a912126751a269d264acb10409457964c31c89ae93319869b/Windows XP Hardware Remove.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/be0a3d0e2ca1b0598ae549c09352e67c6c03a43f278e46c428de915bc3950aa0/Windows XP Information Bar.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/3788390515d250043c75a2faf4c3b8bc84726d029355509ebfd72c3ab0c056b2/Windows XP Logoff Sound.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/7eb4c58378604de5399b0f4d33e2318f6d0cea56047c0448e6c5a937b9bfbd3d/Windows XP Logon Sound.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/b3e29525bfc59606871f0938419a676e45c9bcd6be725acc4bca9d8df17c2282/Windows XP Menu Command.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/3c5caa4d4426a6d590d88e044364d51260f9ef07150cd731cea6fdce85bf5832/Windows XP Minimize.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/8d3212f2607440b1efd04a301c2e13d10f5b0b89ef300ffac0b5000a7dc1a968/Windows XP Notify.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/99b23bd72244714a6f5d851e05fdafb8f67d267b2bfe36afad3aade661f9285a/Windows XP Pop-up Blocked.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/4f9faa90838f66c83a90899adf8e6669554fd51190b068357913d4f0069e38fd/Windows XP Print complete.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/2db831dac50c8362cf5e82da61132fb130e114429da47be9d8078873bcf8ed0a/Windows XP Recycle.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/3f59f983534d9c5d69883c96e676cf9133c26152f3e24bb74607672851169436/Windows XP Restore.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/9257768155d81dc2a86d305f9461b6cd200fa44f0e5047385ed68fafaa966aa9/Windows XP Ringin.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/1746ac1ef644b748891ba1411f4119ce1f48d0ece662d22fd070806e36f3f7bb/Windows XP Ringout.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/292c9f3dc4c492727c2d7f0f4244ecebed9b1da6dc7f427b9f599e0937c64e74/Windows XP Shutdown.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/24472fa09ad796c5091a96a3e442517256a661128cad73e0084f03214f8ef0e5/Windows XP Start.mp3",
  "https://s3.us-west-000.backblazeb2.com/nyaabucket/241240fbfc9da59b8b0ae26288fc2aa30d2dbe0d5dd7f0b7a187d1b38fa2fe45/Windows XP Startup.mp3",
];

function getNextSound(): HTMLAudioElement {
  const src = sounds[Math.floor(Math.random() * sounds.length)];
  return new Audio(src);
}

// Keep it in a variable so it's cached early
var nextSound: HTMLAudioElement = getNextSound();
export function playRandomXPSound() {
  const toPlay = nextSound;
  nextSound = getNextSound();

  toPlay.play();
}
