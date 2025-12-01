// const BASE_API = "http://localhost:3000/api/v1";
//const BASE_API = "http://host.docker.internal:3000/api/v1";
const BASE_API = "/api/v1";
window.API_URL = BASE_API;
const API_URL = BASE_API;

// ==========================
// Authentication utilities
// ==========================
function checkAuth() {
  const authData = JSON.parse(
    localStorage.getItem("roamadmin_auth") ||
    sessionStorage.getItem("roamadmin_auth")
  );
  return !!(authData && authData.isLoggedIn && authData.token);
}

function redirectToLogin() {
  window.location.href = "login.html";
}

function logout() {
  localStorage.removeItem("roamadmin_auth");
  sessionStorage.removeItem("roamadmin_auth");
  redirectToLogin();
}

// ==========================
// App Utilities
// ==========================
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

  const allRoleItems = document.querySelectorAll("[data-role]");
  const adminItems = document.querySelectorAll('[data-role*="admin"]');
  const operatorItems = document.querySelectorAll('[data-role*="operator"]');
  const superAdminItems = document.querySelectorAll('[data-role*="super_admin"]');
  const viewerItems = document.querySelectorAll('[data-role*="all"]');

  // Hide all
  allRoleItems.forEach((item) => (item.style.display = "none"));

  // Show items by role
  switch (role) {
    case "super_admin":
      allRoleItems.forEach((item) => (item.style.display = "block"));
      break;
    case "admin":
      adminItems.forEach((item) => (item.style.display = "block"));
      operatorItems.forEach((item) => (item.style.display = "block"));
      viewerItems.forEach((item) => (item.style.display = "block"));
      break;
    case "operator":
      operatorItems.forEach((item) => (item.style.display = "block"));
      viewerItems.forEach((item) => (item.style.display = "block"));
      break;
    case "viewer":
      viewerItems.forEach((item) => (item.style.display = "block"));
      break;
    default:
      viewerItems.forEach((item) => (item.style.display = "block"));
      break;
  }

  updateUserDisplay(role);
}

function updateUserDisplay(role) {
  const authData = JSON.parse(
    localStorage.getItem("roamadmin_auth") ||
    sessionStorage.getItem("roamadmin_auth")
  );
  if (!authData) return;

  const usernameDisplay = document.getElementById("username-display");
  const userFullname = document.getElementById("user-fullname");

  if (usernameDisplay) {
    usernameDisplay.textContent = `${authData.username} (${role})`;
  }
  if (userFullname) {
    userFullname.textContent = `${authData.username} - ${formatRoleName(role)}`;
  }

  updateSidebarRoleBadge(role);
}

function formatRoleName(role) {
  const roleNames = {
    super_admin: "Super Administrator",
    admin: "Administrator",
    operator: "Operator",
    viewer: "Viewer"
  };
  return roleNames[role] || role;
}

function updateSidebarRoleBadge(role) {
  const existingBadge = document.querySelector(".role-badge");
  if (existingBadge) existingBadge.remove();

  const brandLink = document.querySelector(".brand-link");
  if (brandLink) {
    const roleBadge = document.createElement("div");
    roleBadge.className = "role-badge";
    roleBadge.innerHTML = `
      <small class="badge badge-light position-absolute" 
             style="top:10px; right:10px; font-size:0.6rem;">
        ${formatRoleName(role)}
      </small>`;
    brandLink.style.position = "relative";
    brandLink.appendChild(roleBadge);
  }
}

// ==========================
// Layout Loader
// ==========================
async function loadLayout() {
  await AppUtils.loadHTML("pages/header.html", "header");
  await AppUtils.loadHTML("pages/footer.html", "footer");
  await AppUtils.loadHTML("pages/sidebar.html", "sidebar");

  await new Promise((resolve) => setTimeout(resolve, 100));

  initTreeview();
  setupLogoutHandler();

  const authData = JSON.parse(
    localStorage.getItem("roamadmin_auth") ||
    sessionStorage.getItem("roamadmin_auth")
  );
  if (authData && authData.role) {
    updateUIForRole(authData.role, authData.permissions || []);
  }
}

// ==========================
// Sidebar + Navigation
// ==========================
function setupLogoutHandler() {
  document.addEventListener("click", function (e) {
    if (e.target && e.target.id === "logout-btn") {
      e.preventDefault();
      logout();
    }
  });
}

function initTreeview() {
  document.querySelectorAll("#sidebar .nav-item > a").forEach((link) => {
    const submenu = link.nextElementSibling;
    if (submenu && submenu.classList.contains("nav-treeview")) {
      link.addEventListener("click", (e) => {
        e.preventDefault();
        const isVisible = submenu.style.display === "block";
        document
          .querySelectorAll("#sidebar .nav-treeview")
          .forEach((ul) => (ul.style.display = "none"));
        submenu.style.display = isVisible ? "none" : "block";
      });
    }
  });
}

async function loadPage(page) {
  const container = document.getElementById("content-area");
  if (!container) return;

  try {
    const res = await fetch(`pages/${page}.html`);
    const html = await res.text();
    container.innerHTML = html;

    await new Promise((resolve) => requestAnimationFrame(resolve));

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

    const titleEl = document.getElementById("page-title");
    if (titleEl) titleEl.textContent = titleMap[page] || "Dashboard";

    const breadcrumbEl = document.querySelector(".breadcrumb .active");
    if (breadcrumbEl) breadcrumbEl.textContent = titleMap[page] || "Dashboard";

    document.querySelectorAll("#sidebar a.nav-link").forEach((link) => {
      link.classList.remove("active");
      const hrefPage = link
        .getAttribute("href")
        ?.replace(".html", "")
        .replace("#", "");
      if (hrefPage === page) link.classList.add("active");
    });

    const scriptPath = `js/${page}.js`;
    document.querySelectorAll(`script[data-page-script]`).forEach((s) => s.remove());

    const script = document.createElement("script");
    script.src = scriptPath;
    script.setAttribute("data-page-script", page);
    script.onload = () => {
      console.log(`✅ Loaded ${scriptPath}`);
      const initFunc = `${page}Init`;
      if (typeof window[initFunc] === "function") window[initFunc]();
    };
    script.onerror = () => {
      const initFunc = `${page}Init`;
      if (typeof window[initFunc] === "function") window[initFunc]();
    };
    document.head.appendChild(script);
  } catch (err) {
    console.error(`❌ Failed to load page ${page}:`, err);
    container.innerHTML = `<div class="text-danger">Failed to load page: ${err.message}</div>`;
  }
}

function handleNavigation() {
  const page = location.hash.replace("#", "") || "dashboard";
  console.log(`📄 handleNavigation -> ${page}`);
  loadPage(page);
}

window.addEventListener("hashchange", handleNavigation);

// ==========================
// Initial load with safe auth check
// ==========================
window.addEventListener("DOMContentLoaded", async () => {
  const currentPage = window.location.pathname.split("/").pop();

  // ✅ Skip auth check if on login page
  if (currentPage === "login.html") {
    console.log("🚪 Login page detected, skipping auth check...");
    return;
  }

  if (!checkAuth()) {
    console.log("❌ Not authenticated, redirecting to login...");
    redirectToLogin();
    return;
  }

  await loadLayout();
  handleNavigation();
});

// Make functions available to login.js
window.updateUIForRole = updateUIForRole;
window.handlePostLoginUI = function (authData) {
  updateUIForRole(authData.user.role, authData.permissions || []);
};
