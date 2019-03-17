import * as wasm from "wasm-example"

const message = wasm.greet('hoge')
console.log(message)

const markdown = `
# h1
test

- item1
- item2
- item3
`
const container = document.getElementsByClassName('container')[0]
container.textContent = null
container.insertAdjacentHTML('afterbegin', message)
container.insertAdjacentHTML('afterbegin', wasm.markdown_to_html(markdown))
