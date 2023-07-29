function resize() {
    let trading_panel_width = 320;

    let el = document.getElementById("main");


    el.style.setProperty('display', "block");

    let winWidth = window.innerWidth;
    let winHeight = window.innerHeight;
    el.style.setProperty('--main-height', winHeight + "px");
    el.style.setProperty('--main-width', winWidth + "px");


    let left_offset = el.style.getPropertyValue('--terminal-left-offset');
    let top_offset = el.style.getPropertyValue('--terminal-top-offset');

    el.style.setProperty('--terminal-left', left_offset + "px");
    el.style.setProperty('--terminal-top', top_offset + "px");

    let terminal_width = winWidth - left_offset;

    el.style.setProperty('--terminal-height', (winHeight - top_offset) + "px");
    el.style.setProperty('--terminal-width', terminal_width + "px");

    el.style.setProperty('--trading-panel-left', (terminal_width - trading_panel_width) + "px");
    el.style.setProperty('--trading-panel-width', trading_panel_width + "px");

    el.style.setProperty('height', winHeight + "px");
}


function set_focus(id) {
    setTimeout(function () {
        console.log("set focus: " + id);
        let el = document.getElementById(id);
        el.focus();
        el.select();

    }, 100);
}
addEventListener("resize", resize);
setTimeout(resize, 100);