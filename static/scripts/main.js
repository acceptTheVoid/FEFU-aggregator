var offset = 0;
var current = 0;

function checkPosition() {
    const height = document.body.offsetHeight;
    const screenHeight = window.innerHeight;

    const scrolled = window.scrollY;
    const threshold = height - screenHeight / 4;
    const position = scrolled + screenHeight;

    if (position >= threshold) {

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
    window.addEventListener("scroll", throttle(checkPosition, 250));
    window.addEventListener("resize", throttle(checkPosition, 250));
})()

async function fetchPosts() {
    const url = `${window.location.href}/get?offset=${offset}`;
    const response = await fetch(url);
    console.log(response);
    const text = await response.json();
    text.forEach(element => {
        console.log(element.text);
    });
}

fetchPosts();