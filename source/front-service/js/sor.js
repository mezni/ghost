const API_URL = "http://127.0.0.1:3000/api/v1/sor";

// Load SOR list
async function loadSOR() {
    const res = await fetch(API_URL);
    const data = await res.json();

    const tbody = document.querySelector("#sorTable tbody");
    tbody.innerHTML = "";

    data.forEach(sor => {
        const tr = document.createElement("tr");
        tr.innerHTML = `
            <td>${sor.sor_plan_id}</td>
            <td>${sor.country_name}</td>
            <td>${sor.operator_name}</td>
            <td>${sor.routage_type_name}</td>
            <td>${sor.barring}</td>
            <td>${sor.rate}</td>
            <td>
                <button class="btn btn-sm btn-info" onclick="editSOR(${sor.sor_plan_id})">Edit</button>
                <button class="btn btn-sm btn-danger" onclick="deleteSOR(${sor.sor_plan_id})">Delete</button>
            </td>
        `;
        tbody.appendChild(tr);
    });
}

// Create SOR
document.getElementById("sorForm").addEventListener("submit", async (e) => {
    e.preventDefault();
    const form = e.target;
    const payload = {
        country_name: form.country_name.value,
        operator_name: form.operator_name.value,
        routage_type_name: form.routage_type_name.value,
        barring: form.barring.value,
        rate: form.rate.value,
        created_by: "system"
    };

    const res = await fetch(API_URL, {
        method: "POST",
        headers: {"Content-Type": "application/json"},
        body: JSON.stringify(payload)
    });

    if (res.ok) {
        form.reset();
        loadSOR();
        alert("SOR created successfully!");
    } else {
        const err = await res.text();
        alert("Error: " + err);
    }
});

// Delete SOR
async function deleteSOR(id) {
    if (!confirm("Are you sure to delete this SOR?")) return;
    const res = await fetch(`${API_URL}/${id}`, { method: "DELETE" });
    if (res.ok || res.status === 204) {
        loadSOR();
        alert("SOR deleted successfully!");
    } else {
        const err = await res.text();
        alert("Error: " + err);
    }
}

// Edit SOR (simple prompt)
async function editSOR(id) {
    const rate = prompt("Enter new rate:");
    if (!rate) return;

    const payload = { updated_by: "system", rate, country_name: "", operator_name: "", routage_type_name: "" };

    const res = await fetch(`${API_URL}/${id}?routage_type_id=1`, {
        method: "PUT",
        headers: {"Content-Type": "application/json"},
        body: JSON.stringify(payload)
    });

    if (res.ok) {
        loadSOR();
        alert("SOR updated successfully!");
    } else {
        const err = await res.text();
        alert("Error: " + err);
    }
}

// Initial load
loadSOR();
