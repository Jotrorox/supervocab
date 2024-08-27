const tokenInput = document.getElementById("token");
const submitButton = document.getElementById("submit");

tokenInput.addEventListener("input", () => {
    if (tokenInput.value.length === 43) {
        submitButton.style.display = "block";
    } else {
        submitButton.style.display = "none";
    }
});

submitButton.addEventListener("click", async () => {
    if (tokenInput.value.length !== 43) {
        alert("The Token isn't valid")
        return
    }

    const token = tokenInput.value
    const url = window.location.href + "register/" + token
    await fetch(url)
})