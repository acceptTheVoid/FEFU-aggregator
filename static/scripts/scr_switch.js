function page_switch(){
    let content = document.getElementById('content');
    set_opacity(content, 0, 60);
    setTimeout(() => set_opacity(content, 1, 60), 1000);
  }



function set_opacity(content, end_op, time) {
    let op = Number(content.style.opacity);
    if (content.style.opacity == "") op = 1;
    if (end_op > op) sign = 1;
    else sign = -1;
    function run() {
        content.style.opacity = op;
        op += sign*0.1;
        if (sign*op > sign*end_op) {
            content.style.opacity = end_op;
            clearInterval(timer);
        }
    };
    let timer = setInterval(run, time);
}
