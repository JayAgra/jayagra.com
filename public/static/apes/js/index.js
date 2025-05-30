const V = "1009";

var navLinks = Array.from(document.querySelectorAll(".navLinks button"));

document.body.onload = () => {
    document.loadContent(0);
}

navLinks.forEach((element, index) => {
    element.addEventListener("click", () => {
        navLinks.forEach((element) => {
            element.classList.remove("navLinkButtonSelected");
        });
        navLinks[index].classList.add("navLinkButtonSelected");
        document.loadContent(index);
    });
});

document.loadContent = (index) => {
    console.log("Loading content with index " + index)
    document.getElementById("mainScreen").classList.add("hidden");
    document.getElementById("loadingScreen").classList.remove("hidden");
    // set new title
    document.querySelector(".descriptionContainer h2").textContent = CYCLES[index].title;
    // remove existing steps
    var stepsContainer = document.getElementsByClassName("steps")[0];
    while (stepsContainer.firstChild) {
        stepsContainer.removeChild(stepsContainer.firstChild);
    }
    CYCLES[index].steps.forEach((item, stepIndex) => {
        var stepCard = document.createElement("div");
        stepCard.className = "stepCard";
        stepCard.onclick = () => { document.selectStep(index, stepIndex); };
        var title = document.createElement("h2");
        title.textContent = item.title;
        var brief = document.createElement("p");
        brief.textContent = item.brief;
        stepCard.appendChild(title);
        stepCard.appendChild(brief);
        stepsContainer.appendChild(stepCard);
    });
    var copyrightDiv = document.createElement("div");
    copyrightDiv.className = "stepCard";
    copyrightDiv.style = "background: transparent !important; border: transparent !important;";
    copyrightDiv.classList.add("mobileHidden");
    var copyright = document.createElement("small");
    copyright.innerHTML = "<details><summary>Copyright © 2025 Jayen Agrawal. All Rights Reserved. Click for details.</summary><p style=\"padding-left: 0.5em\"><br>Licensed Content:<br>" + "Diagram Copyright © 2025 " + CYCLES[index].copyright + "<br>Descriptions Copyright © 2025 Milo Kroh.<br><br>Contact <a href=\"mailto:jay@jayagra.com\">jay@jayagra.com</a> for inquiries and bug reports.<br><br>v" + V + "<br>" + navigator.userAgent + "</p></details><br>"
    copyrightDiv.appendChild(copyright);
    stepsContainer.appendChild(copyrightDiv);
    var imageStack = document.getElementsByClassName("imageLayers")[0];
    while (imageStack.firstChild) {
        imageStack.removeChild(imageStack.firstChild);
    }
    var imagesToLoad = CYCLES[index].layer_images.length + 1;
    [CYCLES[index].base_image].concat(CYCLES[index].layer_images).forEach((item, imageIndex) => {
        var imageLayer = document.createElement("img");
        imageLayer.src = item + "?v=" + V;
        imageLayer.alt = imageIndex;
        imageLayer.classList.add("fullWidthHeight");
        imageLayer.onload = () => { imagesToLoad--; if (imagesToLoad == 0) { document.selectStep(index, 0); } };
        imageLayer.onerror = () => { imagesToLoad--; if (imagesToLoad == 0) { document.selectStep(index, 0); } };
        imageStack.appendChild(imageLayer);
    });
};

document.selectStep = (cycle, step) => {
    var steps = Array.from(document.getElementsByClassName("steps")[0].children);
    steps.pop();
    steps.forEach((item) => {
        item.classList.remove("selectedItem");
    });
    steps[step].classList.add("selectedItem");
    document.querySelector("div.description > h3.mobileMarginAdd").textContent = CYCLES[cycle].steps[step].title;
    var existingDescription = document.querySelector("div.descriptionContainer > div > div.descriptionInsert");
    if (existingDescription) {
        existingDescription.remove();
    }
    document.querySelector("div.description > h3.mobileMarginAdd").insertAdjacentHTML("afterend", "<div class=\"descriptionInsert\"><hr>" + CYCLES[cycle].steps[step].html + "</div>")
    document.activateTooltips();
    var images = Array.from(document.getElementsByClassName("imageLayers")[0].children);
    images.forEach((image) => {
        image.style.opacity = step == 0 ? "1" : "0.1"
    });
    images[0].style.opacity = "0.5"
    images[step].style.opacity = "1"
    if (step == 0) {
        document.getElementById("loadingScreen").classList.add("hidden");
        document.getElementById("mainScreen").classList.remove("hidden");
    }
}
