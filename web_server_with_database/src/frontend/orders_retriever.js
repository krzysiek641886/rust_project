
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
                    <th>Chunks</th>
                </tr>
            </thead>
            <tbody id="orders-tbody">
            </tbody>
        </table>
    `;
}