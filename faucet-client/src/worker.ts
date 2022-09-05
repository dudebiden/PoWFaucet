
import { PoWWorker } from "./worker/PoWWorker";
import { getScrypt, getScryptMine, getScryptReadyPromise, Scrypt } from "../../libs/scrypt_wasm";

(() => {
  getScryptReadyPromise().then(() => {
    (globalThis as any).powWorker = new PoWWorker({
      scrypt: getScrypt(),
      mine: getScryptMine(),
    });
  })
})();