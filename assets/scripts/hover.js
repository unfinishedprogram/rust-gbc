const input = {
    joyp_right: false,
    joyp_left: false,
    joyp_down: false,
    joyp_up: false,
    start: false,
    select: false,
    button_a: false,
    button_b: false,
}

const convert = ({
    button_a: a,
    button_b: b,
    joyp_right: right,
    joyp_left: left,
    joyp_down: down,
    joyp_up: up,
    start,
    select,
}) => JSON.stringify({
    a,
    b,
    select,
    start,
    right,
    left,
    up,
    down,
});


const input_elms = {};

for (let key of Object.keys(input)) {
    input_elms[key] = document.getElementById(key);
}

console.log(input_elms);

const handleTouch = (e) => {
    for (key in input_elms) {
        input[key] = false;
        input_elms[key].removeAttribute("data-pressed");
    }


    for (let { clientX: x, clientY: y } of e.touches) {
        let elm = document.elementFromPoint(x, y);
        if (elm.id in input) {
            input[elm.id] = true;
            elm.setAttribute("data-pressed", "");
        }
    }

    window.controller_state = convert(input);
};



document.addEventListener("touchmove", handleTouch);
document.addEventListener("touchend", handleTouch);
document.addEventListener("touchstart", handleTouch);
document.addEventListener("touchcancel", handleTouch);