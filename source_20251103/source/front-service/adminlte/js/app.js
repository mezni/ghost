const BASE_API = "http://localhost:3000/api/v1";
window.API_URL = BASE_API;
const API_URL = BASE_API; 

// Authentication check function
function checkAuth() {
    const authData = JSON.parse(localStorage.getItem('roamadmin_auth') || sessionStorage.getItem('roamadmin_auth'));
    return !!(authData && authData.isLoggedIn && authData.token);
}

function redirectToLogin() {
    window.location.href = 'login.html';
}

function logout() {
    localStorage.removeItem('roamadmin_auth');
    sessionStorage.removeItem('roamadmin_auth');
    redirectToLogin();
}

const AppUtils = {
  fetchData: async (endpoint) => {
    try {
      const response = await fetch(`${BASE_API}/${endpoint}`);
      if (!response.ok) throw new Error(`API error: ${response.status}`);
      return await response.json();
    } catch (err) {
      console.error("❌ API Fetch Error:", err);
      return null;
    }
  },

  notify: (title, message, type = "info") => {
    alert(`${title}: ${message}`);
  },

  loadHTML: async (url, containerId) => {
    try {
      const res = await fetch(url);
      const html = await res.text();
      const container = document.getElementById(containerId);
      if (container) container.innerHTML = html;
    } catch (err) {
      console.error(`❌ Failed to load ${url}:`, err);
    }
  }
};

// ==========================
// Role-based UI Management
// ==========================
function updateUIForRole(role, permissions) {
    console.log(`🎭 Updating UI for role: ${role}`);
    
    // Get all menu items with role data
    const allRoleItems = document.querySelectorAll('[data-role]');
    const adminItems = document.querySelectorAll('[data-role*="admin"]');
    const operatorItems = document.querySelectorAll('[data-role*="operator"]');
    const superAdminItems = document.querySelectorAll('[data-role*="super_admin"]');
    const viewerItems = document.querySelectorAll('[data-role*="all"]');
    
    // Hide all role-specific items first
    allRoleItems.forEach(item => {
        item.style.display = 'none';
    });
    
    // Show items based on role
    switch(role) {
        case 'super_admin':
            console.log('👑 Showing Super Admin menu items');
            allRoleItems.forEach(item => item.style.display = 'block');
            break;
        case 'admin':
            console.log('⚡ Showing Admin menu items');
            adminItems.forEach(item => item.style.display = 'block');
            operatorItems.forEach(item => item.style.display = 'block');
            viewerItems.forEach(item => item.style.display = 'block');
            break;
        case 'operator':
            console.log('🔧 Showing Operator menu items');
            operatorItems.forEach(item => item.style.display = 'block');
            viewerItems.forEach(item => item.style.display = 'block');
            break;
        case 'viewer':
            console.log('👀 Showing Viewer menu items');
            viewerItems.forEach(item => item.style.display = 'block');
            break;
        default:
            console.log('❓ Unknown role, showing only viewer items');
            viewerItems.forEach(item => item.style.display = 'block');
            break;
    }
    
    // Update username display with role badge
    updateUserDisplay(role);
}

function updateUserDisplay(role) {
    const authData = JSON.parse(localStorage.getItem('roamadmin_auth') || sessionStorage.getItem('roamadmin_auth'));
    if (!authData) return;
    
    const usernameDisplay = document.getElementById('username-display');
    const userFullname = document.getElementById('user-fullname');
    
    if (usernameDisplay) {
        usernameDisplay.textContent = `${authData.username} (${role})`;
    }
    if (userFullname) {
        userFullname.textContent = `${authData.username} - ${formatRoleName(role)}`;
    }
    
    // Add role badge to sidebar
    updateSidebarRoleBadge(role);
}

function formatRoleName(role) {
    const roleNames = {
        'super_admin': 'Super Administrator',
        'admin': 'Administrator',
        'operator': 'Operator',
        'viewer': 'Viewer'
    };
    return roleNames[role] || role;
}

function updateSidebarRoleBadge(role) {
    // Remove existing role badge
    const existingBadge = document.querySelector('.role-badge');
    if (existingBadge) {
        existingBadge.remove();
    }
    
    // Add new role badge
    const brandLink = document.querySelector('.brand-link');
    if (brandLink) {
        const roleBadge = document.createElement('div');
        roleBadge.className = 'role-badge';
        roleBadge.innerHTML = `
            <small class="badge badge-light position-absolute" style="top: 10px; right: 10px; font-size: 0.6rem;">
                ${formatRoleName(role)}
            </small>
        `;
        brandLink.style.position = 'relative';
        brandLink.appendChild(roleBadge);
    }
}

// ==========================
// Initialize common layout
// ==========================
async function loadLayout() {
  await AppUtils.loadHTML("pages/header.html", "header");
  await AppUtils.loadHTML("pages/footer.html", "footer");
  await AppUtils.loadHTML("pages/sidebar.html", "sidebar");
  
  // Wait for sidebar to be fully loaded before initializing
  await new Promise(resolve => setTimeout(resolve, 100));
  
  initTreeview();
  setupLogoutHandler();
  
  // Update UI based on role AFTER sidebar is loaded
  const authData = JSON.parse(localStorage.getItem('roamadmin_auth') || sessionStorage.getItem('roamadmin_auth'));
  if (authData && authData.role) {
    console.log('🔄 Updating UI with role from auth data:', authData.role);
    updateUIForRole(authData.role, authData.permissions || []);
  }
}

// ==========================
// Setup logout handler
// ==========================
function setupLogoutHandler() {
  document.addEventListener('click', function(e) {
    if (e.target && e.target.id === 'logout-btn') {
      e.preventDefault();
      logout();
    }
  });
}

// ==========================
// Sidebar treeview toggle
// ==========================
function initTreeview() {
  document.querySelectorAll("#sidebar .nav-item > a").forEach(link => {
    const submenu = link.nextElementSibling;
    if (submenu && submenu.classList.contains("nav-treeview")) {
      link.addEventListener("click", (e) => {
        e.preventDefault();
        const isVisible = submenu.style.display === "block";
        document.querySelectorAll("#sidebar .nav-treeview").forEach(ul => ul.style.display = "none");
        submenu.style.display = isVisible ? "none" : "block";
      });
    }
  });
}

// ==========================
// Load page content
// ==========================
async function loadPage(page) {
  const container = document.getElementById("content-area");
  if (!container) return;

  try {
    const res = await fetch(`pages/${page}.html`);
    const html = await res.text();
    container.innerHTML = html;

    // Wait for browser to parse HTML
    await new Promise(resolve => requestAnimationFrame(resolve));

    // Update title and breadcrumb
    const titleEl = document.getElementById("page-title");
    const titleMap = {
      dashboard: "Dashboard",
      roamin: "Roam IN",
      roamout: "Roam OUT", 
      sorperformance: "SoR Performance",
      countries: "Countries",
      operators: "Operators",
      networks: "Networks",
      sorplans: "SOR Plans",
      prefixes: "Prefixes",
    };
    if (titleEl) titleEl.textContent = titleMap[page] || "Dashboard";

    const breadcrumbEl = document.querySelector(".breadcrumb .active");
    if (breadcrumbEl) breadcrumbEl.textContent = titleMap[page] || "Dashboard";

    // Sidebar highlight
    document.querySelectorAll("#sidebar a.nav-link").forEach(link => {
      link.classList.remove("active");
      const hrefPage = link.getAttribute("href")?.replace(".html", "").replace("#", "");
      if (hrefPage === page) link.classList.add("active");
    });

    // Load page-specific JS dynamically
    const scriptPath = `js/${page}.js`;

    // Remove previous page script
    document.querySelectorAll(`script[data-page-script]`).forEach(s => s.remove());

    const script = document.createElement('script');
    script.src = scriptPath;
    script.setAttribute("data-page-script", page);
    script.onload = () => {
      console.log(`✅ Loaded ${scriptPath}`);
      const initFunc = `${page}Init`;
      if (typeof window[initFunc] === "function") {
        console.log(`🚀 Calling ${initFunc}()`);
        window[initFunc]();
      }
    };
    script.onerror = () => {
      console.warn(`⚠️ Could not load ${scriptPath}, trying cached init function`);
      const initFunc = `${page}Init`;
      if (typeof window[initFunc] === "function") {
        console.log(`🚀 Calling ${initFunc}() from cache`);
        window[initFunc]();
      }
    };
    document.head.appendChild(script);

  } catch (err) {
    console.error(`❌ Failed to load page ${page}:`, err);
    container.innerHTML = `<div class="text-danger">Failed to load page: ${err.message}</div>`;
  }
}

// ==========================
// Handle navigation
// ==========================
function handleNavigation() {
  const page = location.hash.replace("#", "") || "dashboard";
  console.log(`📄 handleNavigation -> ${page}`);
  loadPage(page);
}

window.addEventListener("hashchange", handleNavigation);

// ==========================
// Initial load with authentication check
// ==========================
window.addEventListener("DOMContentLoaded", async () => {
  // Check authentication before loading anything
  if (!checkAuth()) {
    redirectToLogin();
    return;
  }

  await loadLayout();
  handleNavigation();
});

// Make these functions available globally for login.js
window.updateUIForRole = updateUIForRole;
window.handlePostLoginUI = function(authData) {
    updateUIForRole(authData.user.role, authData.permissions || []);
};