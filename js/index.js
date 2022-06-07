const { runtime } = require("webpack");

const rust = import("./pkg");

rust.then(m => m.greet("Word!"))
    .catch(console.error);

