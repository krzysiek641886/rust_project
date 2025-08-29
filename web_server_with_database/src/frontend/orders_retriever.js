
export default function createRetrievedOrdersTable() {
    const formContainer = document.getElementById("results-container");
    formContainer.innerHTML = `
        <table id="orders-table">
            <thead>
                <tr>
                    <th>Date</th>
                    <th>Name</th>
                    <th>Email</th>
                    <th>Copies</th>
                    <th>File Name</th>
                    <th>Price</th>
                    <th>Material Type</th>
                    <th>Print Type</th>
                    <th>Status</th>
                </tr>
            </thead>
            <tbody id="orders-tbody">
            </tbody>
        </table>
    `;

    // Add event listener for the request orders button
    const requestOrdersBtn = document.getElementById("request-orders-btn");
    requestOrdersBtn.addEventListener("click", async function () {
        try {
            const response = await fetch("/api/orders");
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            const orders = await response.json();
            populateOrdersTable(orders);
        } catch (error) {
            console.error("Error fetching orders:", error);
            alert("Failed to fetch orders. Please try again.");
        }
    });
}

function populateOrdersTable(orders) {
    const tbody = document.getElementById("orders-tbody");
    tbody.innerHTML = ""; // Clear existing rows

    orders.forEach(order => {
        const row = document.createElement("tr");
        // Set row ID to order.date
        row.id = `order-${order.date}`;

        const dateTd = document.createElement("td");
        dateTd.textContent = order.date;
        row.appendChild(dateTd);

        const nameTd = document.createElement("td");
        nameTd.textContent = order.name;
        row.appendChild(nameTd);

        const emailTd = document.createElement("td");
        emailTd.textContent = order.email;
        row.appendChild(emailTd);

        const copiesTd = document.createElement("td");
        copiesTd.textContent = order.copies_nbr;
        row.appendChild(copiesTd);

        const fileNameTd = document.createElement("td");
        fileNameTd.textContent = order.file_name;
        row.appendChild(fileNameTd);

        const priceTd = document.createElement("td");
        priceTd.textContent = order.price.toFixed(2) + " PLN";
        row.appendChild(priceTd);

        const materialTd = document.createElement("td");
        materialTd.textContent = order.material_type;
        row.appendChild(materialTd);

        const printTypeTd = document.createElement("td");
        printTypeTd.textContent = order.print_type;
        row.appendChild(printTypeTd);

        const statusTd = document.createElement("td");
        createStatusDropdown(statusTd, order);
        row.appendChild(statusTd);

        tbody.appendChild(row);
    });
}

function createStatusDropdown(parent, order) {
    const statusSelect = document.createElement("select");
    statusSelect.className = "status-select";
    statusSelect.dataset.orderId = order.id;

    const statuses = ["New", "InProgress", "Completed", "Canceled"];
    statuses.forEach(status => {
        const option = document.createElement("option");
        option.value = status;
        option.textContent = status.charAt(0).toUpperCase() + status.slice(1);
        if (status === order.status) {
            option.selected = true;
        }
        statusSelect.appendChild(option);
    });

    statusSelect.addEventListener("change", async function () {
        try {
            const response = await fetch(`/api/orders/modify`, {
                method: "PUT",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({ datetime: order.date, new_status: this.value })
            });

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
        } catch (error) {
            console.error("Error updating status:", error);
            alert("Failed to update status. Please try again.");
        }
    });

    parent.appendChild(statusSelect);
}