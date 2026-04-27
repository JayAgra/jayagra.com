"use strict";

function encodeCred() {
    var e = (document.getElementById("in_username") as HTMLInputElement).value,
        t = (document.getElementById("in_password") as HTMLInputElement).value;

    (document.getElementById("cred_output") as HTMLTextAreaElement).value = btoa(e + ":" + t);
}
function decodeCred() {
    var e = (document.getElementById("cred_input") as HTMLInputElement).value;
    if (e) {
        var decoded = atob(e).split(":");
        (document.getElementById("out_username") as HTMLInputElement).value = decoded[0] || "";
        (document.getElementById("out_password") as HTMLInputElement).value = decoded[1] || "";
    }
}
function convertText() {
    var e = (document.getElementById("base64str") as HTMLTextAreaElement).value;
    (document.getElementById("outputBase64") as HTMLTextAreaElement).textContent = atob(e);
}
function convertTextR() {
    var e = (document.getElementById("base64strR") as HTMLTextAreaElement).value;
    (document.getElementById("outputBase64R") as HTMLTextAreaElement).textContent = btoa(e);
}
function convertImage() {
    let e = (document.getElementById("imageUpload") as HTMLInputElement),
        t = e.files![0],
        n = new FileReader();
    (n.onloadend = () => {
        let e = n.result;
        (document.getElementById("outputBase64URL") as HTMLTextAreaElement).textContent = e!.toString();
    }),
        n.readAsDataURL(t);
}
function convertToImage() {
    let b64 = (document.getElementById("inputBase64URL") as HTMLTextAreaElement).value;
    if (b64) {
        (document.getElementById("outputImage") as HTMLImageElement).src = b64;
    }
}
// export functions
function downloadImage() {
    let b64 = (document.getElementById("inputBase64URL") as HTMLTextAreaElement).value;
    if (b64) {
        var link = document.createElement("a");
        link.href = b64;
        link.download = String(Date.now());
        link.click();
    }
}
function copyContentOf(element) {
    var text = (document.getElementById(element) as HTMLTextAreaElement | HTMLInputElement).value;
    if (text) {
        navigator.clipboard.writeText(text).then(
            () => {},
            () => {
                alert("Copy failed.");
            }
        );
    }
}
