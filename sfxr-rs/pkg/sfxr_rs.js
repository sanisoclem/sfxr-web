
const lAudioContext = (typeof AudioContext !== 'undefined' ? AudioContext : (typeof webkitAudioContext !== 'undefined' ? webkitAudioContext : undefined));
let wasm;

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}
/**
*/
export class FmOsc {

    static __wrap(ptr) {
        const obj = Object.create(FmOsc.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_fmosc_free(ptr);
    }
    /**
    */
    constructor() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.fmosc_new(retptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return FmOsc.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets the gain for this oscillator, between 0.0 and 1.0.
    * @param {number} gain
    */
    set_gain(gain) {
        wasm.fmosc_set_gain(this.ptr, gain);
    }
    /**
    * @param {number} freq
    */
    set_primary_frequency(freq) {
        wasm.fmosc_set_primary_frequency(this.ptr, freq);
    }
    /**
    * @param {number} note
    */
    set_note(note) {
        wasm.fmosc_set_note(this.ptr, note);
    }
    /**
    * This should be between 0 and 1, though higher values are accepted.
    * @param {number} amt
    */
    set_fm_amount(amt) {
        wasm.fmosc_set_fm_amount(this.ptr, amt);
    }
    /**
    * This should be between 0 and 1, though higher values are accepted.
    * @param {number} amt
    */
    set_fm_frequency(amt) {
        wasm.fmosc_set_fm_frequency(this.ptr, amt);
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('sfxr_rs_bg.wasm', import.meta.url);
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        var ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_connect_23205ccf67cb254c = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).connect(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_connect_00d53ad6331afb5d = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).connect(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_settype_9258fd7ca4968686 = function(arg0, arg1) {
        getObject(arg0).type = takeObject(arg1);
    };
    imports.wbg.__wbg_frequency_bf89d7a5c1f063a4 = function(arg0) {
        var ret = getObject(arg0).frequency;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_start_92de6a31a329b20a = function() { return handleError(function (arg0) {
        getObject(arg0).start();
    }, arguments) };
    imports.wbg.__wbg_destination_97006ee89653d765 = function(arg0) {
        var ret = getObject(arg0).destination;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_7b23bc5a2d082b0d = function() { return handleError(function () {
        var ret = new lAudioContext();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_close_66d819a2d05872ee = function() { return handleError(function (arg0) {
        var ret = getObject(arg0).close();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createGain_532ad5f6cbd7460d = function() { return handleError(function (arg0) {
        var ret = getObject(arg0).createGain();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createOscillator_5ce665d11b0f37dd = function() { return handleError(function (arg0) {
        var ret = getObject(arg0).createOscillator();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_value_36b0df0105e70ae8 = function(arg0) {
        var ret = getObject(arg0).value;
        return ret;
    };
    imports.wbg.__wbg_setvalue_be470337de0de85d = function(arg0, arg1) {
        getObject(arg0).value = arg1;
    };
    imports.wbg.__wbg_gain_ba40a5d6d1af8cc7 = function(arg0) {
        var ret = getObject(arg0).gain;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }



    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

export default init;

