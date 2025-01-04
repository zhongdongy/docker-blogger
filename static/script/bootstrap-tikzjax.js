function TikzCodeBlockPreProcessor(source) {
  const containerEle = document.createElement("div");
  containerEle.classList.add("svg-container-tikzjax");
  const scriptEle = document.createElement("script");
  scriptEle.setAttribute("type", "text/tikz");
  scriptEle.setAttribute("data-show-console", "true");
  scriptEle.innerHTML = TidyTikzSource(source);
  containerEle.appendChild(scriptEle);
  return containerEle;
}

function TidyTikzSource(source) {
  return source.replaceAll("&nbsp;", "").split("\n").map(line => line.trim()).filter(line => line).join("\n");
}

function ScanTikzCodeBlocks() {
  console.debug("[Bootstrap:TikzJax] preprocess started.");
  let count = 0;
  document.querySelectorAll("pre > code.language-tikz").forEach((ele) => {
    const rawContent = ele.textContent;
    const parentEle = ele.parentElement;
    const preprocessedEle = TikzCodeBlockPreProcessor(rawContent);
    parentEle.replaceWith(preprocessedEle);
    count += 1;
  });
  console.log(`[Bootstrap:TikzJax] ${count} tikz blocks processed.`)
}

function InjectTikJaxCssStyles() {
  const styleEle = document.createElement('style');
  styleEle.setAttribute("id", "bootstrap-tikzjax-styles");
  styleEle.innerHTML = `div.svg-container-tikzjax {
    position: relative;
  }

  div.svg-container-tikzjax svg {
    position: relative;
  }`;
  document.body.insertBefore(styleEle, document.body.firstChild);
}

function HandlePostRenderEvent(ev) {
  if (!ev) { return; }
  const target = ev.target;
  const parent = target.parentElement;
  const width = target.getBoundingClientRect().width;
  const containerWidth = parent.getBoundingClientRect().width;
  if (width < containerWidth) {
    target.style.width = `${width}px`;
    target.style.left = `${(containerWidth - width) / 2}px`;
  } else {
    parent.style.overflowX = "auto";
    parent.style.paddingBottom = "0.5em";
  }
}

window.addEventListener("load", () => {
  InjectTikJaxCssStyles();
  ScanTikzCodeBlocks();
  window.addEventListener("tikzjax-load-finished", HandlePostRenderEvent);
})