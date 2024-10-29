export interface Transaction {
    version: number,
    lockTime: number,
    in: TxIn[],
    out: TxOut[],
}

export interface TxIn {
    outpointTxId: string,
    outpointIndex: number,
    sequence: number,
    scriptSig: string,
}

export interface TxOut {
    amount: number,
    scriptPubKey: string,
}

export const CHECK_DIGITS = x => /^\d*$/.test(x);

export function defaultTxIn(): TxIn {
    return {
        outpointTxId: '',
        outpointIndex: 0,
        scriptSig: '',
        sequence: 0xfffffffd,
    };
}

export function defaultTxOut(): TxOut {
    return {
        amount: 0,
        scriptPubKey: '',
    };
}
