/**
 * Function to create the form and add event listeners.
 */
function createForm() {
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
            <button type="submit" id="upload-button">Upload</button>
        </form>
    `;

    // Add event listener for form submission
    const form = document.getElementById("file-upload-form");
    form.addEventListener("submit", async function (event) {
        event.preventDefault(); // Prevent the default form submission

        const formData = new FormData(form);
        try {
            const response = await fetch("api/upload", {
                method: "POST",
                body: formData
            });
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            alert("File uploaded successfully!");
        } catch (error) {
            alert(`Error uploading file: ${error.message}`);
        }
    });
}

/**
 * Event listener for the DOMContentLoaded event.
 * Fetches data from the root URL and updates the content of the element with ID "content".
 * Logs an error message to the console if the fetch operation fails.
 */
document.addEventListener("DOMContentLoaded", function () {
    fetch("/api/backendstatus")
        .then(response => {
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}, failed to initialize the server correctly.`);
            }
        })
        .then(data => {
            createForm();
        })
        .catch(error => {
            document.getElementById("form-container").innerText = error.message;
        });

    // Create the form regardless of the API call status
    createForm();

    // WebSocket setup
    const ws = new WebSocket("ws://127.0.0.1:8080/api/web_socket_results");
    ws.onopen = function () {
        console.log("WebSocket connection established.");
    };
    ws.onmessage = function (event) {
        alert("Message from server: " + event.data);
    };
    ws.onclose = function () {
        console.log("WebSocket connection closed.");
    };
    ws.onerror = function (error) {
        console.error("WebSocket error:", error);
    };
});
