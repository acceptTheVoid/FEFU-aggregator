function send_thread() {
    let group_id = sessionStorage.getItem('group_id');
    let post_id = sessionStorage.getItem('post_id');

    console.log(group_id);
    console.log(post_id);

    let xhr = new XMLHttpRequest();
    let fd = new FormData(form);

    fd.append('group_id', group_id);
    fd.append('post_id', post_id);

    console.log(fd);

    xhr.open('POST', 'http://127.0.0.1:8000/new_thread');
    try {
        xhr.send(fd)
    } catch (e) {
        window.location.href = 'http://127.0.0.1:8000';
    };

    sessionStorage.removeItem('group_id');
    sessionStorage.removeItem('post_id');

    window.location.href = 'http://127.0.0.1:8000/threads';
}

const form = document.querySelector('form');

console.log(form);

form.addEventListener('submit', (event) => {
    event.preventDefault();
    send_thread();
});