// Simple authentication functions
function logout() {
    if (confirm('Are you sure you want to logout?')) {
        localStorage.clear();
        // Redirect to home page - use correct path
        window.location.href = '../index.html';
    }
}

// Remove any auto-redirect code that might be causing the issue
$(document).ready(function() {
    console.log('Auth loaded - no auto redirect');
});