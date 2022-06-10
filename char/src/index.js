import { chars } from "./chars-list.js";
let imp = import("./pkg");
let mod;

let counters = [];
imp.then(wasm => {
    mod = wasm;
    addCounter();
    let b = document.getElementById("add-counter");
    if (!b) throw new Error("Unable to find #add-counter");
    b.addEventListener("click", ev => addCounter());
}).catch(console.error);

function addCounter() {
    const ctr = mod.Counter.new(randomChar(), 0);
    counters.push(ctr);
    update();
}

function update() {
    const container = document.getElementById("container");
    if (!container) {
        throw new Error("Unable to find #container in dom");
    }

    while (container.hasChildNodes()) {
        if (container.lastChild.id === "add-counter") break;
        container.removeChild(container.lastChild);
    }

    for (let i = 0; i < counters.length; i++) {
        let counter = counters[i];
        container.appendChild(newCounter(counter.key(), counter.count(), ev => {
            counter.increment();
            update();
        }));
    }
}


function randomChar() {
    console.log("randomChar");
    const idx = Math.floor(Math.random() * (chars.length - 1));
    console.log("index", idx);
    const ret = chars.splice(idx, 1)[0];
    console.log("char", ret);
    return ret;
}


function newCounter(key, value, cb) {
    let container = document.createElement("div");
    container.setAttribute("class", "counter");
    let title = document.createElement("h1");
    title.appendChild(document.createTextNode("Counter ", key));
    container.appendChild(title);
    container.appendChild(newField("Count", value));

    const plus = document.createElement("button");
    plus.setAttribute("type", "button");
    plus.setAttribute("class", "plus-button");
    plus.appendChild(document.createTextNode("+"));
    plus.addEventListener("click", cb);
    container.appendChild(plus);
    return container;
}

function newField(key, value) {
    const ret = document.createElement("div");
    ret.setAttribute("class", "field");

    const name = document.createElement("span");
    name.setAttribute("class", "name");
    name.appendChild(document.createTextNode(key));
    ret.appendChild(name);

    const val = document.createElement("span");
    val.setAttribute("class", "value");
    val.appendChild(document.createTextNode(value));
    ret.appendChild(val);
    return ret;
}