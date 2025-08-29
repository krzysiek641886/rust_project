

/**
 * Creates and returns a drag & drop area with a hidden file input.
 * @param {HTMLElement} parent - The parent element to append the drop area to.
 * @returns {HTMLInputElement} The file input element.
 */
export default function createDragAndDropArea(parent) {
    const dropArea = document.createElement("div");
    dropArea.id = "drop-area";
    dropArea.style.display = "flex";
    dropArea.style.alignItems = "center";
    dropArea.style.justifyContent = "center";
    dropArea.style.width = "100%";
    dropArea.style.height = "120px";
    dropArea.style.margin = "10px 0";
    dropArea.style.border = "2px dashed #888";
    dropArea.style.borderRadius = "8px";
    dropArea.style.background = "#f9f9f9";
    dropArea.style.fontSize = "1.2em";
    dropArea.style.cursor = "pointer";
    dropArea.style.textAlign = "center";

    const dropAreaText = document.createElement("span");
    dropAreaText.id = "drop-area-text";
    dropAreaText.style.width = "100%";
    dropAreaText.textContent = "Drag & Drop file here or click to select";

    const fileInput = document.createElement("input");
    fileInput.type = "file";
    fileInput.id = "file";
    fileInput.name = "file";
    fileInput.required = true;
    fileInput.style.display = "none";

    dropArea.appendChild(dropAreaText);
    dropArea.appendChild(fileInput);
    parent.appendChild(dropArea);

    // Highlight drop area on drag events
    ["dragenter", "dragover"].forEach(eventName => {
        dropArea.addEventListener(eventName, (e) => {
            e.preventDefault();
            e.stopPropagation();
            dropArea.style.background = "#e0e0e0";
            dropArea.style.borderColor = "#333";
        });
    });

    ["dragleave", "drop"].forEach(eventName => {
        dropArea.addEventListener(eventName, (e) => {
            e.preventDefault();
            e.stopPropagation();
            dropArea.style.background = "#f9f9f9";
            dropArea.style.borderColor = "#888";
        });
    });

    // Handle file drop
    dropArea.addEventListener("drop", (e) => {
        e.preventDefault();
        e.stopPropagation();
        const files = e.dataTransfer.files;
        if (files.length > 0) {
            fileInput.files = files;
            dropAreaText.textContent = files[0].name;
        }
    });

    // Open file dialog on click
    dropArea.addEventListener("click", () => {
        fileInput.click();
    });

    // Show file name when selected via dialog
    fileInput.addEventListener("change", () => {
        if (fileInput.files.length > 0) {
            dropAreaText.textContent = fileInput.files[0].name;
        } else {
            dropAreaText.textContent = "Drag & Drop file here or click to select";
        }
    });

    return fileInput;
}