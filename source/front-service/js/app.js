/**
 * AdminLTE Starter App
 * Handles loading header, footer, sidebar, and page content dynamically
 */

document.addEventListener('DOMContentLoaded', async () => {
  console.log('AdminLTE Starter initialized');

  await loadLayoutPart('pages/header.html', '#header');
  await loadLayoutPart('pages/sidebar.html', '#sidebar');
  await loadLayoutPart('pages/footer.html', '#footer');

  loadPage('dashboard'); // default page
});

// Load partials like header, sidebar, footer
async function loadLayoutPart(url, selector) {
  try {
    const response = await fetch(url);
    const html = await response.text();
    document.querySelector(selector).innerHTML = html;
  } catch (err) {
    console.error(`Failed to load ${url}:`, err);
  }
}

// Load pages dynamically
async function loadPage(page) {
  const content = document.getElementById('content-area');
  try {
    const response = await fetch(`pages/${page}.html`);
    const html = await response.text();
    content.innerHTML = html;
    document.getElementById('page-title').textContent = page.charAt(0).toUpperCase() + page.slice(1);

    // Initialize page-specific scripts
if (page === 'countries') {
  const script = document.createElement('script');
  script.src = 'js/countries.js';
  script.onload = () => {
    CountriesApp.init(); // <-- explicitly initialize after loading
  };
  document.body.appendChild(script);
}


  } catch (err) {
    content.innerHTML = `<div class="alert alert-danger">Page not found: ${page}</div>`;
  }
}
