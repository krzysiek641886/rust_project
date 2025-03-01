/**
 * Event listener for the DOMContentLoaded event.
 * Fetches data from the root URL and updates the content of the element with ID "content".
 * Logs an error message to the console if the fetch operation fails.
 */
document.addEventListener("DOMContentLoaded", function () {
    fetch("/api/backendstatus")
        .then(data => {
            const formContainer = document.getElementById("form-container");
            formContainer.innerHTML = `
                <form id="file-upload-form" action="/upload" method="post" enctype="multipart/form-data">
                    <label for="name">Name:</label>
                    <input type="text" id="name" name="name" required><br>
                    <label for="email">Email:</label>
                    <input type="text" id="email" name="email" required><br>
                    <label for="copies_nbr">Number of copies:</label>
                    <input type="number" id="copies_nbr" name="copies_nbr" required><br>
                    <label for="file">Choose file to upload:</label>
                    <input type="file" id="file" name="file" required>
                    <button type="submit">Upload</button>
                </form>
            `;
        })
        // .catch(error => {
        //     console.error("Error fetching data:", error);
        //     document.getElementById("db_init_status_message").innerText = error.message;
        // });
});