const tokenInput = document.getElementById('token');
const submitButton = document.getElementById('submit');

tokenInput.addEventListener('input', () => {
    if (tokenInput.value.trim() !== '') {
        submitButton.style.display = 'block';
    } else {
        submitButton.style.display = 'none';
    }
});