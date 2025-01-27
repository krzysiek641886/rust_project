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
});