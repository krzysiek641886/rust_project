
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
        row.innerHTML = `
            <td>${order.name}</td>
            <td>${order.email}</td>
            <td>${order.copies_nbr}</td>
            <td>${order.material_type}</td>
            <td>${order.file_name}</td>
            <td>$${order.price.toFixed(2)}</td>
        `;
        tbody.appendChild(row);
    });
}