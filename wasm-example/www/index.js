import * as wasm from "wasm-example"

const message = wasm.greet('hoge')
console.log(message)

const container = document.getElementsByClassName('container')[0]
container.textContent = null
container.insertAdjacentHTML('afterbegin', message)
