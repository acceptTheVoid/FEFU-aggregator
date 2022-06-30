async function init() {
    let response = await fetch('/get_targets');
    let targets = await response.json();
    return targets;
}

async function page_switch(id) {
    let target = targets[id];
    let content = document.getElementById('content');
    set_opacity(content, 0, 60);
    setTimeout(() => set_opacity(content, 1, 60), 1000);
    content.innerHTML = "";
    target.forEach(element => {
        content.innerHTML += element.name + "<br>";
    });
}

function set_opacity(content, end_op, time) {
    let op = Number(content.style.opacity);
    if (content.style.opacity == "") op = 1;
    if (end_op > op) sign = 1;
    else sign = -1;

    function run() {
        content.style.opacity = op;
        op += sign * 0.1;
        if (sign * op > sign * end_op) {
            content.style.opacity = end_op;
            clearInterval(timer);
        }
    };
    let timer = setInterval(run, time);
}