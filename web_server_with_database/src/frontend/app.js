var ERROR_CODE_FAIELD_PROCESSING_SUBMITTED_FORM = 1006;
var EVALUATION_RESULT_TYPE = "evaluation_result";

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
        const file = formData.get("file");
        if (!file) {
            alert("No file selected.");
            return;
        }
        if (!file.size) {     
            alert("Empty file selected.");
            return;
        }

        // Calculate the number of chunks (e.g., 1 MB per chunk)
        const CHUNK_SIZE = 64 * 1024; // 64 kB
        const nbrTotalChunks = Math.ceil(file.size / CHUNK_SIZE);
        const metadata = {
            name: formData.get("name"),
            email: formData.get("email"),
            copies_nbr: parseInt(formData.get("copies_nbr"), 10),
            file_name: file.name,
            nbr_of_chunks: nbrTotalChunks,
        };

        const arrayBufferChunks = [];
        for (let i = 0; i < nbrTotalChunks; i++) {
            const start = i * CHUNK_SIZE;
            const end = Math.min(start + CHUNK_SIZE, file.size);
            const chunk = file.slice(start, end);
            arrayBufferChunks.push(await chunk.arrayBuffer());
        }

        // Send metadata first
        ws.send(JSON.stringify(metadata));

        // Then send binary chunks
        for (const chunk of arrayBufferChunks) {
            ws.send(chunk);
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

    // WebSocket setup
    window.ws = new WebSocket("ws://127.0.0.1:8080/api/websocket_evaluation");
    window.ws.onopen = function () {
        console.log("WebSocket connection established.");
    };
    window.ws.onmessage = function (event) {
        // Parse the received JSON string
        try {
            let data = JSON.parse(event.data);
            if (data.type === EVALUATION_RESULT_TYPE) {
                alert("Evaluation result: " + event.data);
            }
        }  catch(e) {
            // Do nothing
        }
        finally {
            console.log("Received message: ", event.data);
        }
    };
    window.ws.onclose = function (event) {
        if (event.code === ERROR_CODE_FAIELD_PROCESSING_SUBMITTED_FORM) {
            alert("Error with processing the request. Websocket connection closed unexpectedly.");
        }
    };

    // Create the form regardless of the API call status
    createForm();
});
