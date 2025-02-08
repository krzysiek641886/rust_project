/**
 * Event listener for the DOMContentLoaded event.
 * Fetches data from the root URL and updates the content of the element with ID "content".
 * Logs an error message to the console if the fetch operation fails.
 */
document.addEventListener("DOMContentLoaded", function () {
    fetch("/api/greet")
        .then(response => response.json())
        .then(data => {
            document.getElementById("db_init_status_message").innerText = data.status;
        })
        .catch(error => {
            console.error("Error fetching data:", error);
        });

    const formContainer = document.getElementById("form-container");
    formContainer.innerHTML = `
        <form action="/submit" method="post">
            <label for="name">Name:</label>
            <input type="text" id="name" name="name"><br>
            <label for="surname">Surname:</label>
            <input type="text" id="surname" name="surname"><br>
            <label for="age">Age:</label>
            <input type="number" id="age" name="age"><br>
            <input type="submit" value="Submit">
        </form>
    `;
});