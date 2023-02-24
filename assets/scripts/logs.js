import * as wasm from "/gbc-emu.js"

const log_markup = `
<div id="logs_container">
<div id="logs">
    <div id="logs_inner"></div>
</div>
</div>`;

function insertHTML() {
    const container = document.createElement("div");
    const logs = document.createElement("div");
    const inner = document.createElement("div");

    logs.appendChild(inner);
    container.appendChild(logs);
    inner.id = "logs_inner";
    logs.id = "logs";
    container.id = "logs_container";
    document.body.insertBefore(container, document.body.firstChild)
}

class LogDisplay {
    constructor(elm = document.querySelector("#logs")) {
        this.elm = elm;
        this.elm_height = 32;
        this.container_height = this.elm.parentElement.clientHeight / this.elm_height;
        this.inner = elm.querySelector("#logs_inner");
        this.logger_height = 0;
        this.last_top = 0;
        this.last_bottom = 0;

        this.top = 0;
        this.last_max = 0;
        this.bottom = 0;

        for (let i = 0; i < this.container_height; i++) {
            let elm = document.createElement("span");
            elm.innerText = "Testing!";
            this.inner.append(elm);
        }
    }

    update() {
        console.log(wasm.log_count())
        this.logger_height = wasm.log_count() * this.elm_height;
        this.elm.setAttribute("style", `min-height:${this.logger_height}px;`);
        this.container_height = this.elm.parentElement.clientHeight;
        this.top = Math.abs(this.elm.parentElement.scrollTop / this.elm_height) | 0;
        this.bottom = (this.top + this.elm.parentElement.clientHeight / this.elm_height) | 0;
        let h = this.bottom - this.top;

        let before_elm = this.inner.firstChild;
        for (let i = 0; i < h; i++) {
            let index = this.top + i;
            if (index < this.last_top) {
                const elm = this.inner.lastChild;
                elm.textContent = JSON.parse(wasm.get_logs(index, index + 1));

                this.inner.insertBefore(elm, before_elm);
            } else if (index >= this.last_bottom) {
                const elm = this.inner.firstChild;
                elm.textContent = JSON.parse(wasm.get_logs(index, index + 1));
                this.inner.appendChild(elm);

            }

        }

        this.inner.setAttribute("style", `
        margin-top:${this.top * this.elm_height}px;
        height:${h * this.elm_height}px;
        `);
        this.last_bottom = this.bottom;
        this.last_top = this.top;
    }
}


document.addEventListener("DOMContentLoaded", () => {
    insertHTML();

    setTimeout(() => {
        const logDisplay = new LogDisplay();

        function update() {
            logDisplay.update();
            requestAnimationFrame(update)
        }
        update();
    }, 100)
})
