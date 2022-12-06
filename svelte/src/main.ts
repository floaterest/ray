import './app.sass'
import App from './App.svelte'
import init from 'wasm';


await init();

const app = new App({
  target: document.getElementById('app')
})

export default app
