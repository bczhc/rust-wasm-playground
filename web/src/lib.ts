import * as wasm from 'wasm-lib';

export function useWasm() {
    return wasm
}

export function to_hex_padded(byte: number, padded_length: number) {
    let s = byte.toString(16);
    if (s.length < padded_length) {
        return '0'.repeat(padded_length - s.length) + s;
    }
    return s;
}
