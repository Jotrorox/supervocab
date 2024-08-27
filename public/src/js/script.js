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

    fetch(window.location.href + "register/" + token)
        .then(response => {
            if (response.status === 400) {
                alert("The Token is invalid")
            } else {
                alert("It worked you are now signed up")
            }
        })
})