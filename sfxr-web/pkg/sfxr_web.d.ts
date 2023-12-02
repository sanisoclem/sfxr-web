/* tslint:disable */
/* eslint-disable */
/**
*/
export class SoundEffectGenerator {
  free(): void;
/**
*/
  constructor();
/**
*/
  play(): void;
/**
* @param {string} name
* @param {bigint} seed
*/
  preset(name: string, seed: bigint): void;
/**
* @returns {any}
*/
  export_raw(): any;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_soundeffectgenerator_free: (a: number) => void;
  readonly soundeffectgenerator_new: (a: number) => void;
  readonly soundeffectgenerator_play: (a: number, b: number) => void;
  readonly soundeffectgenerator_preset: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly soundeffectgenerator_export_raw: (a: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
