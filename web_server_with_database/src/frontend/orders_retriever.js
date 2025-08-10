
export default function createRetrievedOrdersTable() {
    const formContainer = document.getElementById("results-container");
    formContainer.innerHTML = `
        <table id="orders-table">
            <thead>
                <tr>
                    <th>Name</th>
                    <th>Email</th>
                    <th>Copies</th>
                    <th>Material Type</th>
                    <th>File Name</th>
                    <th>Price</th>
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

        const nameTd = document.createElement("td");
        nameTd.textContent = order.name;
        row.appendChild(nameTd);

        const emailTd = document.createElement("td");
        emailTd.textContent = order.email;
        row.appendChild(emailTd);

        const copiesTd = document.createElement("td");
        copiesTd.textContent = order.copies_nbr;
        row.appendChild(copiesTd);

        const materialTd = document.createElement("td");
        materialTd.textContent = order.material_type;
        row.appendChild(materialTd);

        const fileNameTd = document.createElement("td");
        fileNameTd.textContent = order.file_name;
        row.appendChild(fileNameTd);

        const priceTd = document.createElement("td");
        priceTd.textContent = order.price.toFixed(2) + " PLN";
        row.appendChild(priceTd);

        tbody.appendChild(row);
    });
}