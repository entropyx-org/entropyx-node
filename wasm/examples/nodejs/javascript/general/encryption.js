const entropyx = require('../../../../nodejs/entropyx');

entropyx.initConsolePanicHook();

(async () => {

    let encrypted = entropyx.encryptXChaCha20Poly1305("my message", "my_password");
    console.log("encrypted:", encrypted);
    let decrypted = entropyx.decryptXChaCha20Poly1305(encrypted, "my_password");
    console.log("decrypted:", decrypted);

})();
