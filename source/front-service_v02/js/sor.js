const API_URL = "http://127.0.0.1:3000/api/v1/sor";
const COUNTRIES_API = "http://127.0.0.1:3000/api/v1/countries";
const OPERATORS_API = "http://127.0.0.1:3000/api/v1/operators/by-country";
const ROUTAGE_TYPES_API = "http://127.0.0.1:3000/api/v1/routagetypes";

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

// Load routage types into select
async function loadRoutageTypes(selectedId = null) {
    const res = await fetch(ROUTAGE_TYPES_API);
    const routageTypes = await res.json();
    const select = document.getElementById("routageTypeSelect");
    select.innerHTML = `<option value="" disabled selected>Select a routage type</option>`;
    routageTypes.forEach(rt => {
        const option = document.createElement("option");
        option.value = rt.routage_type_id;
        option.textContent = rt.routage_type_name;
        if (selectedId && selectedId === rt.routage_type_id) {
            option.selected = true;
        }
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
        option.value = op.operator_id; // use operator_id for backend
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

    // reload routage types for fresh dropdown
    loadRoutageTypes();

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

    const countrySelect = document.getElementById("countrySelect");
    countrySelect.innerHTML = `<option value="${sor.country_id}" selected>${sor.country_name}</option>`;
    countrySelect.disabled = true;

    const operatorSelect = document.getElementById("operatorSelect");
    operatorSelect.innerHTML = `<option value="${sor.operator_id}" selected>${sor.operator_name}</option>`;
    operatorSelect.disabled = true;

    await loadRoutageTypes(sor.routage_type_id);

    document.getElementById("barringCheck").checked = sor.barring;
    document.getElementById("rateInput").value = sor.rate;

    $("#sorModal").modal("show");
}

// Create/Update SOR
document.getElementById("sorForm").addEventListener("submit", async (e) => {
    e.preventDefault();

    const countrySelect = document.getElementById("countrySelect");
    const operatorSelect = document.getElementById("operatorSelect");
    const routageSelect = document.getElementById("routageTypeSelect");

    const payload = {
        country_name: countrySelect.options[countrySelect.selectedIndex].text,
        operator_name: operatorSelect.options[operatorSelect.selectedIndex].text,
        routage_type_id: parseInt(routageSelect.value),
        routage_type_name: routageSelect.options[routageSelect.selectedIndex].text,
        barring: document.getElementById("barringCheck").checked,
        rate: document.getElementById("rateInput").value,
        created_by: "system",
        updated_by: "system"
    };

    if (editingSORId) {
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
        } else {
            const err = await res.text();
            alert("Error: " + err);
        }

    } else {
        const res = await fetch(API_URL, {
            method: "POST",
            headers: {"Content-Type": "application/json"},
            body: JSON.stringify(payload)
        });

        if (res.ok) {
            $("#sorModal").modal("hide");
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
loadRoutageTypes();
loadSOR();
