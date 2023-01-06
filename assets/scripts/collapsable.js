document.querySelectorAll(".collapsible").forEach(elm => {
    let label = elm.querySelector("label");
    let content = elm.querySelector(".content");
    content.style.maxHeight = "0px";

    label.addEventListener("click", () => {
        elm.toggleAttribute("data-open");
        if (elm.hasAttribute("data-open")) {
            content.style.maxHeight = `${content.scrollHeight}px`;
        } else {
            content.style.maxHeight = null;
        }
    });
})