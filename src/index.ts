//@ts-ignore
import init, { greet } from 'sfxr-rs';

init().then(() => {
  console.log('init wasm-pack');
  greet('test');
});