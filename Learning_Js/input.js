var readline = require('readline').createInterface({
    input: process.stdin,
    output: process.stdout
});
readline.question("Whta tour name?", function (name) {
    console.log("Hi ".concat(name));
    readline.close();
});
