var offset = 0;
var current = 0;
var posts_ended = false;

async function checkPosition() {
    if (posts_ended) return;
    const height = document.body.offsetHeight;
    const screenHeight = window.innerHeight;

    const threshold = height - 10 * screenHeight;
    const position = window.scrollY + screenHeight;

    if (position >= threshold) {
        await fetchPosts();
    }
}

function throttle(calee, timeout) {
    let timer = null
    return function perform(...args) {
        if (timer) return;

        timer = setTimeout(() => {
            calee(...args);
            clearTimeout(timer);
            timer = null;
        }, timeout);
    }
}

(() => {
    window.addEventListener("scroll", throttle(checkPosition, 1500));
    window.addEventListener("resize", throttle(checkPosition, 1500));
})()

async function fetchPosts() {
    const url = `${window.location.href}/get?offset=${offset}`;
    const response = await fetch(url);
    offset += 15;
    console.log(response);
    const posts = await response.json();
    if (posts.length < 1) {
        posts_ended = true;
        return;
    }
    posts.forEach(appendPost);
}

function appendPost(postData) {
    if (!postData) return;

    const main = document.querySelector("main");
    const postNode = composePost(postData);

    main.append(postNode);
}

function composePost(postData) {
    if (!postData) return;

    const template = document.querySelector('template');
    console.log(template);

    const post = template.content.cloneNode(true);

    const { body, id, threads } = postData;
    post.querySelector("p").innerText = body;
    post.querySelector('button').id = id;

    threads.forEach((n) => {
        let title = document.createElement('h1');
        title.innerHTML = n.title;

        let href = document.createElement('a');
        href.innerHTML = 'Перейти к обсуждению';
        href.href = `/thread/${n.id}`;

        let article = post.querySelector('article');

        article.appendChild(title);
        article.appendChild(href);
    });

    return post;
}

fetchPosts()

function createThread(post_id) {
    let href = window.location.href.split('/');
    let group_id = href[href.length - 1] === "" ? href[href.length - 2] : href[href.length - 1];

    console.log(group_id);

    sessionStorage.setItem('group_id', group_id);
    sessionStorage.setItem('post_id', post_id);

    window.location.href = 'http://127.0.0.1:8000/new_thread';
}