/*********************************************************/
/*  Инициализация.  */
async function init() {
    let response = await fetch('/get_targets');
    let targets = await response.json()
    return targets;
}
const targets = init();
/*********************************************************/
/*  Смена категорий.  */
async function page_switch(id) {
    document.querySelector('#screen-lock').style.display = 'block';
    let targets = await init();
    let publics_info = targets[id];
    let content = document.getElementById('content');
    set_opacity(content, 0, 15);
    setTimeout(function funk() {
        content.innerHTML = '';
        content.insertAdjacentHTML('beforeend', '<div id="public-panel"></div>')
        create_element(publics_info);
    }, 300)

    setTimeout(() => set_opacity(content, 1, 25), 400);
    setTimeout(() => document.querySelector('#screen-lock').style.display = 'none', 600)
}
/*********************************************************/
/*  Добавление паблика.  */
function create_element(publics_info) {
    publics_info.forEach(public_info => {
        let public_panel = document.getElementById('public-panel');
        console.log(public_panel);
        public_panel.insertAdjacentHTML('beforeend',
            `<button class="public-cell" type="submit" onclick="pub_ref('${public_info.tag}')">` +
            `${public_info.name}` +
            '</button>');
    })
}

function pub_ref(href) {
    setTimeout(() => window.location.href = `/group/${href}`, 50);
}

/*********************************************************/
/*  Изменение прозрачности элемента.  */
function set_opacity(content, end_op, time) {
    let op = Number(content.style.opacity);
    if (content.style.opacity == '') op = 1;
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