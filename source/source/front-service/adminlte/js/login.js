// ✅ Expose the init function on window
window.loginInit = async function() {
  console.log("🔐 Login page initialized");

  try {
    await checkExistingAuth();
    initLoginForm();
    initEnterKeyHandling();
    initErrorClearing();
    addDemoCredentialsHelper();
  } catch (err) {
    console.error("❌ Failed to init login page:", err);
  }
};

/**
 * Initialize login form and event handlers
 */
function initLoginForm() {
  const loginForm = document.getElementById('login-form');
  const usernameInput = document.getElementById('username');
  
  if (!loginForm) {
    console.warn("⚠️ login-form not found in DOM");
    return;
  }

  if (usernameInput) usernameInput.focus();
  
  loginForm.addEventListener('submit', function(e) {
    e.preventDefault();
    handleLogin();
  });
}

/**
 * Initialize Enter key handling for password field
 */
function initEnterKeyHandling() {
  const passwordInput = document.getElementById('password');
  if (!passwordInput) return;
  passwordInput.addEventListener('keypress', function(e) {
    if (e.key === 'Enter') handleLogin();
  });
}

/**
 * Initialize error message clearing on input
 */
function initErrorClearing() {
  const usernameInput = document.getElementById('username');
  const passwordInput = document.getElementById('password');
  if (usernameInput) usernameInput.addEventListener('input', hideError);
  if (passwordInput) passwordInput.addEventListener('input', hideError);
}

/**
 * Main login handler
 */
async function handleLogin() {
  const username = document.getElementById('username')?.value.trim();
  const password = document.getElementById('password')?.value;
  const remember = document.getElementById('remember')?.checked;

  if (!username || !password) {
    showError('Please enter both username and password');
    return;
  }

  try {
    setLoadingState(true);

    const loginUrl = `${window.API_URL}/auth/login`;
    console.log('🔐 Attempting login to:', loginUrl);

    const response = await fetch(loginUrl, {
      method: 'POST',
      mode: 'cors',
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
      },
      body: JSON.stringify({
        username,
        password,
        remember_me: remember
      })
    });

    console.log('🔐 Response status:', response.status);

    if (!response.ok) {
      const errorText = await response.text();
      console.error('❌ Server error:', errorText);
      throw new Error(`HTTP ${response.status}: ${errorText}`);
    }

    const data = await response.json();
    console.log('✅ Login response:', data);

    if (data.success) {
      await loginSuccess(data, remember);
    } else {
      loginFailed(data.message || 'Invalid username or password');
    }
  } catch (error) {
    console.error('❌ Login error:', error);
    if (error.name === 'TypeError' && error.message.includes('Failed to fetch')) {
      loginFailed(`Cannot connect to server at ${window.API_URL}. Make sure the API is running and CORS is configured properly.`);
    } else {
      loginFailed(error.message || 'Network error. Please try again.');
    }
  } finally {
    setLoadingState(false);
  }
}

/**
 * Handle successful login
 */
async function loginSuccess(authData, remember) {
  hideError();
  const normalizedRole = authData.user.role.toLowerCase();
  console.log('🔑 Normalized user role:', normalizedRole);
  storeAuthData(authData, normalizedRole, remember);
  await updatePostLoginUI(authData, normalizedRole);
  showSuccessFeedback();
  setTimeout(() => redirectToDashboard(), 500);
}

/**
 * Store authentication data in storage
 */
function storeAuthData(authData, normalizedRole, remember) {
  const storageData = {
    isLoggedIn: true,
    username: authData.user.username,
    role: normalizedRole,
    token: authData.token,
    user: {
      ...authData.user,
      role: normalizedRole
    },
    permissions: authData.permissions,
    timestamp: Date.now(),
    remember
  };
  if (remember)
    localStorage.setItem('roamadmin_auth', JSON.stringify(storageData));
  else
    sessionStorage.setItem('roamadmin_auth', JSON.stringify(storageData));
}

/**
 * Update UI after successful login
 */
async function updatePostLoginUI(authData, normalizedRole) {
  if (typeof handlePostLoginUI === 'function') {
    await handlePostLoginUI({
      ...authData,
      user: { ...authData.user, role: normalizedRole }
    });
  } else if (typeof updateUIForRole === 'function') {
    await updateUIForRole(normalizedRole, authData.permissions || []);
  } else {
    console.warn('⚠️ Role-based UI functions not available');
  }
}

function loginFailed(message) {
  const passwordInput = document.getElementById('password');
  const loginForm = document.getElementById('login-form');
  showError(message);
  if (passwordInput) {
    passwordInput.value = '';
    passwordInput.focus();
  }
  if (loginForm) {
    loginForm.classList.add('shake');
    setTimeout(() => loginForm.classList.remove('shake'), 500);
  }
}

function showError(message) {
  const loginError = document.getElementById('login-error');
  if (loginError) {
    loginError.textContent = message;
    loginError.classList.remove('d-none');
  }
}

function hideError() {
  const loginError = document.getElementById('login-error');
  if (loginError) loginError.classList.add('d-none');
}

function setLoadingState(isLoading) {
  const submitBtn = document.querySelector('#login-form button[type="submit"]');
  if (!submitBtn) return;
  submitBtn.disabled = isLoading;
  submitBtn.innerHTML = isLoading
    ? '<i class="fas fa-spinner fa-spin"></i> Signing In...'
    : 'Sign In';
}

function showSuccessFeedback() {
  const submitBtn = document.querySelector('#login-form button[type="submit"]');
  if (!submitBtn) return;
  submitBtn.innerHTML = '<i class="fas fa-check"></i> Success!';
  submitBtn.disabled = true;
  submitBtn.classList.replace('btn-primary', 'btn-success');
  setTimeout(() => {
    submitBtn.innerHTML = 'Sign In';
    submitBtn.disabled = false;
    submitBtn.classList.replace('btn-success', 'btn-primary');
  }, 400);
}

async function checkExistingAuth() {
  if (await isUserLoggedIn()) {
    console.log('✅ User already logged in, redirecting to dashboard...');
    redirectToDashboard();
  }
}

async function isUserLoggedIn() {
  const authData = JSON.parse(
    localStorage.getItem('roamadmin_auth') ||
    sessionStorage.getItem('roamadmin_auth') ||
    'null'
  );
  return !!(authData && authData.isLoggedIn && authData.token);
}

function redirectToDashboard() {
  window.location.href = 'index.html';
}

function addDemoCredentialsHelper() {
  const loginCardBody = document.querySelector('.login-card-body');
  if (!loginCardBody) return;
  const demoHelper = document.createElement('div');
  demoHelper.className = 'text-center mt-3 small text-muted';
  demoHelper.innerHTML = `
    Demo credentials:<br>
    <strong>admin</strong> / <strong>admin123</strong> (Admin)<br>
    <strong>viewer</strong> / <strong>viewer123</strong> (Viewer)
  `;
  loginCardBody.appendChild(demoHelper);
}

function addLoginStyles() {
  if (document.querySelector('style[data-login-styles]')) return;
  const style = document.createElement('style');
  style.setAttribute('data-login-styles', 'true');
  style.textContent = `
    .shake { animation: shake 0.5s linear; }
    @keyframes shake {
      0%, 100% { transform: translateX(0); }
      10%, 30%, 50%, 70%, 90% { transform: translateX(-5px); }
      20%, 40%, 60%, 80% { transform: translateX(5px); }
    }
    .btn-success {
      background-color: #28a745 !important;
      border-color: #28a745 !important;
    }
    .fa-spinner { animation: spin 1s linear infinite; }
    @keyframes spin {
      0% { transform: rotate(0deg); }
      100% { transform: rotate(360deg); }
    }
  `;
  document.head.appendChild(style);
}

// Add styles when script loads
addLoginStyles();

// ✅ Initialize login when DOM is ready
document.addEventListener('DOMContentLoaded', window.loginInit);
