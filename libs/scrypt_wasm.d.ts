
export type Scrypt = (password: string, salt: string, n: number, r: number, p: number, dklen: number) => string;
export type ScryptMine = (start: number, end: number, difficulty: number, salt: string, n: number, r: number, p: number, dklen: number) => string;

export function getScrypt(): Scrypt;
export function getScryptMine(): ScryptMine;
export function getScryptReadyPromise(): Promise<void>;
