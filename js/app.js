import 'bootstrap';
import "app.scss";
import hljs from "highlight.js";

hljs.initHighlightingOnLoad();

window.publish = function(elem) {
    var publishedInput = document.getElementById("published");
    publishedInput.value = true;
    return true;
}

