const readline = require('readline').createIterface({
    input: process.stdin,
    output: process.stdout
})

readline.question("Whta tour name?", name => {
    console.log(`Hi ${name}`)
    readline.close()
})
