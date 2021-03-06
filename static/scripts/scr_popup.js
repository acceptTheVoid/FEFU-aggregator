function logout() {
    const url = 'http://127.0.0.1:8000/logout';
    let xhr = new XMLHttpRequest();
    xhr.open('GET', url);
    xhr.send();
    setTimeout(() => window.location.href = '/', 50);
}

/*********************************************************/
/*  Вызовы окна.  */
function popup(id) {
    document.querySelector('#screen-lock').style.display = 'block';
    document.querySelector(`#popup-${id}`).style.display = 'flex';
    move(id, 'open', 0);
    setTimeout(() => document.querySelector('#screen-lock').style.display = 'none', 600);

    let popup = document.querySelector(`#popup-${id}`);
    popup.addEventListener('keypress', (e) => {
        if (e.which === 13) {
            e.preventDefault();
            let form = popup.querySelector('form');
            let url = `http://127.0.0.1:8000/${id == 'in' ? 'login' : 'register'}`;
            let xhr = new XMLHttpRequest();
            let fd = new FormData(form);
            xhr.open('POST', url);
            xhr.send(fd);
            setTimeout(() => window.location.href = '/', 50);
        }
    })
}

function popout(id) {
    document.querySelector('#screen-lock').style.display = 'block';
    move(id, 'close', 0);
    setTimeout(() => {
        document.querySelector(`#popup-${id}`).style.display = 'none';
        document.querySelector('#screen-lock').style.display = 'none';
    }, 600);
}
/*********************************************************/
/*  Чудо-move.  */
function move(id, mod, time) {
    let end_top = (mod == 'open' ? 0 : 100);
    let form = document.querySelector(`#form-${id}`);
    let top = (mod == 'open' ? 100 : 0);
    let popup = document.querySelector(`#popup-${id}`);
    let background = (mod == 'open' ? 0 : 0.6);
    let sign = (mod == 'open' ? 1 : -1);
    let a = 0.1;

    function run() {
        form.style.top = top + '%';
        popup.style.background = 'rgba(0, 0, 0, ' + background + ')'
        top += sign * (a - 3);
        background += sign * 0.005;
        if ((a + 0.1) < 3)
            a += a / 13.001;
        if (sign * top < sign * end_top)
            form.style.top = end_top;
        if (sign * background > sign * (mod == 'open' ? 0.6 : 0))
            clearInterval(timer);
    };
    let timer = setInterval(run, time);
}