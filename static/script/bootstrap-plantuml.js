const PLANTUML_SERVER = "http://www.plantuml.com/plantuml";
const PLANTUML_RENDER_STAT = {
  count: 0,
  pending: 0,
};

function ProcessPlantUml(source, callback) {
  var encoded = plantumlEncoder.encode(source);
  fetch(`${PLANTUML_SERVER}/svg/${encoded}`, {
    method: "GET"
  }).then((resp) => {
    const containerEle = document.createElement("div");
    containerEle.classList.add("svg-container-plantuml");
    resp.text().then((imgContent) => {
      InsertSvgImage(containerEle, PostProcessImageContent(imgContent));
    });
    callback(containerEle);
  }).catch(console.error);
}

function PostProcessImageContent(imgContent) {
  return imgContent.replaceAll('font-family="sans-serif"', "");
}

function InsertSvgImage(ele, imageContent) {
  const parser = new DOMParser();
  const svg = parser.parseFromString(imageContent, "image/svg+xml");

  const links = svg.getElementsByTagName("a");
  for (let i = 0; i < links.length; i++) {
    const link = links[i];
    link.addClass("internal-link");
  }

  ele.innerHTML = svg.documentElement.outerHTML;
  HandlePostRenderPosition(ele);
}

function ScanPlantUmlCodeBlocks() {
  console.debug("[Bootstrap:PlantUML] preprocess started.");
  const supportedLanguages = ["plantuml", "plantuml-svg"];
  supportedLanguages.forEach((lang) => {
    document.querySelectorAll(`pre > code.language-${lang}`).forEach((ele) => {
      const rawContent = ele.textContent;
      const parentEle = ele.parentElement;
      PLANTUML_RENDER_STAT.pending += 1;
      ProcessPlantUml(rawContent, (preprocessedEle) => {
        parentEle.replaceWith(preprocessedEle);
        PLANTUML_RENDER_STAT.count += 1;
        PLANTUML_RENDER_STAT.pending -= 1;
      });
    });
  });
  var statMonitorInterval = setInterval(() => {
    if (PLANTUML_RENDER_STAT.pending === 0) {
      clearInterval(statMonitorInterval);
      console.log(`[Bootstrap:PlantUML] ${PLANTUML_RENDER_STAT.count} PlantUML blocks processed.`)
    }
  }, 200);

}

function InjectPlantUmlCssStyles() {
  const styleEle = document.createElement('style');
  styleEle.setAttribute("id", "bootstrap-plantuml-styles");
  styleEle.innerHTML = `div.svg-container-plantuml {
    position: relative;
    padding-top: 1em;
    padding-bottom: 1em;
  }

  div.svg-container-plantuml svg {
    position: relative;
  }`;
  document.body.insertBefore(styleEle, document.body.firstChild);
}

function HandlePostRenderPosition(container) {
  if (!container) { return; }
  const target = container.querySelector("svg");
  const width = target.getBoundingClientRect().width;
  const containerWidth = container.getBoundingClientRect().width;
  if (width < containerWidth) {
    target.style.width = `${width}px`;
    target.style.left = `${(containerWidth - width) / 2}px`;
  } else {
    container.style.overflowX = "auto";
    container.style.paddingBottom = "1.5em";
  }
}

window.addEventListener("load", () => {
  InjectPlantUmlCssStyles();
  ScanPlantUmlCodeBlocks();
})