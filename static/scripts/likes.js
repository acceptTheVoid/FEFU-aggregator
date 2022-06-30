async function like(id) {
    const url = `http://127.0.0.1:8000/like?id=${id}`;
    let resp = await fetch(url);
    let values = await resp.json();
    let buttons = document.getElementsByTagName(id);
    let likes = buttons[0];
    let dislikes = buttons[1];

    likes.innerHTML = values.likes;
    dislikes.innerHTML = values.dislikes;
}

async function dislike(id) {
    const url = `http://127.0.0.1:8000/dislike?id=${id}`;
    let resp = await fetch(url);
    let values = await resp.json();
    let buttons = document.getElementsByTagName(id);
    let likes = buttons[0];
    let dislikes = buttons[1];

    likes.innerHTML = values.likes;
    dislikes.innerHTML = values.dislikes;
}