$(document).ready(function() {
    function loadCountries() {
      $.get('http://localhost:8080/countries', function(data) {
        const tbody = $('#countriesTable tbody');
        tbody.empty();
        data.forEach(c => {
          tbody.append(`
            <tr>
              <td>${c.id}</td>
              <td>${c.name}</td>
              <td>${c.iso_code}</td>
              <td>
                <button class="btn btn-sm btn-warning edit-btn" data-id="${c.id}"><i class="fas fa-edit"></i></button>
                <button class="btn btn-sm btn-danger delete-btn" data-id="${c.id}"><i class="fas fa-trash"></i></button>
              </td>
            </tr>
          `);
        });
      });
    }
  
    loadCountries();
  
    // More handlers for edit, delete, submit can go here
  });
  