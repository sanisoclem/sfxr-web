/* tslint:disable */
/* eslint-disable */
/**
*/
export class EffectGenerator {
  free(): void;
/**
*/
  constructor();
/**
*/
  play(): void;
/**
*/
  mutate(): void;
/**
* @param {string} name
*/
  randomize(name: string): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_effectgenerator_free: (a: number) => void;
  readonly effectgenerator_new: (a: number) => void;
  readonly effectgenerator_play: (a: number, b: number) => void;
  readonly effectgenerator_mutate: (a: number, b: number) => void;
  readonly effectgenerator_randomize: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
