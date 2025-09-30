const API_URL = "http://127.0.0.1:3000/api/v1/sor";
const COUNTRIES_API = "http://127.0.0.1:3000/api/v1/countries";
const OPERATORS_API = "http://127.0.0.1:3000/api/v1/operators/by-country";

let editingSORId = null; // track SOR being edited

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
            <td>${sor.barring ? "Yes" : "No"}</td>
            <td>${sor.rate}</td>
            <td>
                <button class="btn btn-sm btn-info" onclick="editSOR(${sor.sor_plan_id})">Edit</button>
                <button class="btn btn-sm btn-danger" onclick="deleteSOR(${sor.sor_plan_id})">Delete</button>
            </td>
        `;
        tbody.appendChild(tr);
    });
}

// Load countries into select
async function loadCountries() {
    const res = await fetch(COUNTRIES_API);
    const countries = await res.json();
    const select = document.getElementById("countrySelect");
    select.innerHTML = `<option value="" disabled selected>Select a country</option>`;
    countries.forEach(c => {
        const option = document.createElement("option");
        option.value = c.country_id;
        option.textContent = c.country_name;
        select.appendChild(option);
    });
}

// Load operators when country changes
document.getElementById("countrySelect").addEventListener("change", async function() {
    if (editingSORId) return; // don't reload operators when editing
    const countryId = this.value;
    const res = await fetch(`${OPERATORS_API}/${countryId}`);
    const operators = await res.json();

    const operatorSelect = document.getElementById("operatorSelect");
    operatorSelect.innerHTML = `<option value="" disabled selected>Select an operator</option>`;
    operators.forEach(op => {
        const option = document.createElement("option");
        option.value = op.operator_name;
        option.textContent = op.operator_name;
        operatorSelect.appendChild(option);
    });
});

// Open modal for creating new SOR
function openCreateModal() {
    editingSORId = null;
    const form = document.getElementById("sorForm");
    form.reset();
    document.getElementById("countrySelect").disabled = false;
    document.getElementById("operatorSelect").disabled = false;
    $("#sorModal").modal("show");
}

// Edit SOR: fill modal with existing data
async function editSOR(id) {
    const res = await fetch(`${API_URL}/${id}`);
    if (!res.ok) {
        alert("Error fetching SOR");
        return;
    }
    const sor = await res.json();
    editingSORId = id;

    // Fill modal
    const countrySelect = document.getElementById("countrySelect");
    countrySelect.innerHTML = `<option value="${sor.country_id}" selected>${sor.country_name}</option>`;
    countrySelect.disabled = true;

    const operatorSelect = document.getElementById("operatorSelect");
    operatorSelect.innerHTML = `<option value="${sor.operator_name}" selected>${sor.operator_name}</option>`;
    operatorSelect.disabled = true;

    document.getElementById("routageTypeInput").value = sor.routage_type_name;
    document.getElementById("barringCheck").checked = sor.barring;
    document.getElementById("rateInput").value = sor.rate;

    $("#sorModal").modal("show");
}

// Create/Update SOR
document.getElementById("sorForm").addEventListener("submit", async (e) => {
    e.preventDefault();
    const form = e.target;

    const payload = {
        country_name: form.country_name.options[form.country_name.selectedIndex].text,
        operator_name: form.operator_name.value,
        routage_type_name: form.routage_type_name.value,
        barring: document.getElementById("barringCheck").checked,
        rate: form.rate.value,
        created_by: "system",
        updated_by: "system"
    };

    if (editingSORId) {
        // Update existing SOR
        const res = await fetch(`${API_URL}/${editingSORId}`, {
            method: "PUT",
            headers: {"Content-Type": "application/json"},
            body: JSON.stringify(payload)
        });

        if (res.ok) {
            $("#sorModal").modal("hide");
            loadSOR();
            alert("SOR updated successfully!");
            editingSORId = null;
            form.reset();
            document.getElementById("countrySelect").disabled = false;
            document.getElementById("operatorSelect").disabled = false;
        } else {
            const err = await res.text();
            alert("Error: " + err);
        }

    } else {
        // Create new SOR
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

// Initial load
loadCountries();
loadSOR();
