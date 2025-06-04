const vecno = require('../../../../nodejs/vecno');

vecno.initConsolePanicHook();

(async () => {

    let encrypted = vecno.encryptXChaCha20Poly1305("my message", "my_password");
    console.log("encrypted:", encrypted);
    let decrypted = vecno.decryptXChaCha20Poly1305(encrypted, "my_password");
    console.log("decrypted:", decrypted);

})();
