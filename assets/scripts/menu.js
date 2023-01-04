const main_element = document.querySelector("#main");
const menu_toggle = document.querySelector("#toggle_menu");

const menu = {
    open: false,
    element: document.querySelector("#menu"),
    animate_open_element: document.querySelector("#animate_open"),
    animate_close_element: document.querySelector("#animate_close"),
    toggle_open: function () {
        if (this.open) {
            main_element.setAttribute("data-menu-open", "")
            this.animate_open_element.beginElement();
        } else {
            main_element.removeAttribute("data-menu-open")
            this.animate_close_element.beginElement();
        }
        this.open = !this.open;
    }
}

menu_toggle.addEventListener("click", () => {
    menu.toggle_open();
})