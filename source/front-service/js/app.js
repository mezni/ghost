// Function to load HTML content into specified element by ID
function loadHTMLContent(url, elementId) {
    fetch(url)
        .then(response => response.text()) // Get the text content of the HTML file
        .then(html => {
            const element = document.getElementById(elementId); // Get the target element by ID
            element.innerHTML = html; // Inject the HTML into the element
        })
        .catch(error => {
            console.error('Error loading HTML content:', error);
        });
}

// Function to dynamically load page content and set active class
function loadPage(page, linkElement) {
    const content = document.getElementById('main-content');

    // Remove the active class from all links
    const allLinks = document.querySelectorAll('.nav-link');
    allLinks.forEach(link => {
        link.classList.remove('active');
    });

    // Add the active class to the clicked link
    linkElement.classList.add('active');

    // Load the content based on the selected page
    switch (page) {
        case 'dashboard':
            fetch('pages/dashboard.html')
                .then(response => response.text())
                .then(html => {
                    content.innerHTML = html;
                    initializeDashboard(); // Ensure this function exists to initialize the dashboard
                });
            break;
        
        case 'roamin':
            fetch('pages/roamin.html')
                .then(response => response.text())
                .then(html => {
                    content.innerHTML = html;
                    loadGlobalRoamersInData(); // Call the function to load Roam IN data
                    loadGlobalRoamersInByCountryData();
                });
            break;

        case 'roamout': // New case for Roamers OUT
            fetch('pages/roamout.html')
                .then(response => response.text())
                .then(html => {
                    content.innerHTML = html;
                    loadGlobalRoamersOutData(); // Call the function to load Roam OUT data
                    loadGlobalRoamersOutByCountryData();
                });
            break;
        
        default:
            content.innerHTML = "<h3>Page not found!</h3>";
            break;
    }
}

// Load Header, Sidebar, Footer, and Dashboard on page load
document.addEventListener('DOMContentLoaded', () => {
    loadHTMLContent('pages/header.html', 'header-placeholder');
    loadHTMLContent('pages/sidebar.html', 'sidebar-placeholder');
    loadHTMLContent('pages/footer.html', 'footer-placeholder');
    loadHTMLContent('pages/dashboard.html', 'main-content'); // Load dashboard on startup
});
