import createRetrievedOrdersTable from "./orders_retriever.js";
import createDragAndDropArea from "./drag_and_drop_area.js";


var ERROR_CODE_FAILED_PROCESSING_SUBMITTED_FORM = 1006;
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
            <label for="material_type">Material type:</label>
            <select id="material_type" name="material_type" required>
                <option value="PLA">PLA</option>
                <option value="PET">PET</option>
                <option value="ASA">ASA</option>
            </select><br>
            <label for="print_type">Print type:</label>
            <select id="print_type" name="print_type" required>
                <option value="Standard">Standard</option>
                <option value="Precise">Precise</option>
                <option value="ThickLayer">Thick Layer</option>
                <option value="FullFill">Full Fill</option>
            </select><br>
            <label for="file">Choose file to upload:</label>
            <div id="drag-drop-placeholder"></div>
            <button type="submit" id="upload-button">Evaluate Price</button>
        </form>
    `;

    // Insert drag & drop area
    const placeholder = document.getElementById("drag-drop-placeholder");
    const fileInput = createDragAndDropArea(placeholder);

    // Add event listener for form submission
    const form = document.getElementById("file-upload-form");
    form.addEventListener("submit", async function (event) {
        event.preventDefault(); // Prevent the default form submission
        const formData = new FormData(form);
        const file = fileInput.files[0];
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
            material_type: formData.get("material_type"),
            print_type: formData.get("print_type"),
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

function showEvaluationResultPopup(data) {
    // Create a popup window with the evaluation result details
    const popup = document.createElement("div");
    popup.style.position = "fixed";
    popup.style.top = "50%";
    popup.style.left = "50%";
    popup.style.transform = "translate(-50%, -50%)";
    popup.style.background = "#fff";
    popup.style.border = "2px solid #333";
    popup.style.padding = "24px";
    popup.style.zIndex = "10000";
    popup.style.boxShadow = "0 4px 16px rgba(0,0,0,0.2)";
    popup.innerHTML = `
                    <h2>Order Information</h2>
                    <p><strong>Name:</strong> ${data.name}</p>
                    <p><strong>Email:</strong> ${data.email}</p>
                    <p><strong>Number of copies:</strong> ${data.copies_nbr}</p>
                    <p><strong>File Name:</strong> ${data.file_name}</p>
                    <p><strong>Material Type:</strong> ${data.material_type}</p>
                    <p><strong>Print Type:</strong> ${data.print_type}</p>
                    <h2>Estimated Price</h2>
                    <p><strong>Printing price (without delivery):</strong> ${data.price}</p>
                    <button id="close-eval-popup">Close</button>
                    <button id="place-order-button">Place order</button>
                `;
    document.body.appendChild(popup);
    document.getElementById("close-eval-popup").onclick = function () {
        document.body.removeChild(popup);
    };
    // Temporary button to simulate placing an order
    document.getElementById("place-order-button").onclick = function () {
        document.body.removeChild(popup);
    };
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
        try {
            let data = JSON.parse(event.data);
            if (data.type === EVALUATION_RESULT_TYPE) {
                showEvaluationResultPopup(data);
            }
        } catch (e) {
            alert("Error parsing server response. Please check the console for details.");
            return;
        }
        finally {
            console.log("Received message: ", event.data);
        }
    };
    window.ws.onclose = function (event) {
        console.log(event);
        if (event.code === ERROR_CODE_FAILED_PROCESSING_SUBMITTED_FORM) {
            alert("Error with processing the request. Websocket connection closed unexpectedly.");
        }
    };

    // Create the form regardless of the API call status
    createForm();

    // Create the orders table
    createRetrievedOrdersTable();
});
