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

    const { body } = postData;
    post.querySelector("p").innerText = body;

    return post;
}

fetchPosts()