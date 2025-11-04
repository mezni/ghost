// js/login.js - Handles login form submission and authentication
document.addEventListener('DOMContentLoaded', function() {
    const loginForm = document.getElementById('login-form');
    const loginError = document.getElementById('login-error');
    const usernameInput = document.getElementById('username');
    const passwordInput = document.getElementById('password');
    
    // Check if user is already logged in (redirect if true)
    if (checkExistingAuth()) {
        redirectToDashboard();
        return;
    }
    
    // Focus on username field when page loads
    if (usernameInput) {
        usernameInput.focus();
    }
    
    // Handle form submission
    if (loginForm) {
        loginForm.addEventListener('submit', function(e) {
            e.preventDefault();
            handleLogin();
        });
    }
    
    // Handle Enter key press
    if (passwordInput) {
        passwordInput.addEventListener('keypress', function(e) {
            if (e.key === 'Enter') {
                handleLogin();
            }
        });
    }
    
    async function handleLogin() {
        const username = usernameInput.value.trim();
        const password = passwordInput.value;
        const remember = document.getElementById('remember').checked;
        
        // Basic validation
        if (!username || !password) {
            showError('Please enter both username and password');
            return;
        }
        
        try {
            // Show loading state
            setLoadingState(true);
            
            // Call the Rust API for authentication
            const response = await fetch('http://localhost:3000/api/v1/auth/login', {
                method: 'POST',
                mode: 'cors', // Explicitly set CORS mode
                headers: {
                    'Content-Type': 'application/json',
                    'Accept': 'application/json',
                },
                body: JSON.stringify({
                    username: username,
                    password: password,
                    remember_me: remember
                })
            });

            console.log('Response status:', response.status);
            console.log('Response headers:', response.headers);

            if (!response.ok) {
                // Handle HTTP errors
                const errorText = await response.text();
                console.error('Server error:', errorText);
                throw new Error(`HTTP ${response.status}: ${errorText}`);
            }

            const data = await response.json();
            console.log('Login response:', data);
            console.log('🔑 User role from API:', data.user?.role);

            if (data.success) {
                // Successful login
                loginSuccess(data, remember);
            } else {
                // Failed login
                loginFailed(data.message || 'Invalid username or password');
            }
        } catch (error) {
            console.error('Login error:', error);
            if (error.name === 'TypeError' && error.message.includes('Failed to fetch')) {
                loginFailed('Cannot connect to server. Make sure the API is running on localhost:3000 and CORS is configured properly.');
            } else {
                loginFailed(error.message || 'Network error. Please try again.');
            }
        } finally {
            setLoadingState(false);
        }
    }

    function loginSuccess(authData, remember) {
        // Hide any existing error
        hideError();
        
        // Normalize role to handle case variations from API
        const normalizedRole = authData.user.role.toLowerCase();
        console.log('🔑 Normalized user role:', normalizedRole);
        
        // Store authentication data with normalized role
        const storageData = {
            isLoggedIn: true,
            username: authData.user.username,
            role: normalizedRole, // Store normalized role
            token: authData.token,
            user: {
                ...authData.user,
                role: normalizedRole // Also normalize in user object
            },
            permissions: authData.permissions,
            timestamp: new Date().getTime(),
            remember: remember
        };
        
        console.log('💾 Storing auth data with normalized role:', normalizedRole);
        
        if (remember) {
            localStorage.setItem('roamadmin_auth', JSON.stringify(storageData));
        } else {
            sessionStorage.setItem('roamadmin_auth', JSON.stringify(storageData));
        }
        
        // Update UI based on normalized role
        if (typeof handlePostLoginUI === 'function') {
            console.log('🎯 Calling handlePostLoginUI with normalized role:', normalizedRole);
            // Create a modified authData with normalized role
            const modifiedAuthData = {
                ...authData,
                user: {
                    ...authData.user,
                    role: normalizedRole
                }
            };
            handlePostLoginUI(modifiedAuthData);
        } else if (typeof updateUIForRole === 'function') {
            console.log('🎯 Calling updateUIForRole with normalized role:', normalizedRole);
            updateUIForRole(normalizedRole, authData.permissions || []);
        } else {
            console.warn('⚠️ Role-based UI functions not available');
        }
        
        // Show success feedback
        showSuccessFeedback();
        
        // Redirect to dashboard after a brief delay
        setTimeout(() => {
            redirectToDashboard();
        }, 500);
    }
    
    function loginFailed(message) {
        showError(message || 'Invalid username or password. Please try again.');
        
        // Clear password field and focus on it
        passwordInput.value = '';
        passwordInput.focus();
        
        // Add shake animation for visual feedback
        loginForm.classList.add('shake');
        setTimeout(() => {
            loginForm.classList.remove('shake');
        }, 500);
    }
    
    function showError(message) {
        if (loginError) {
            loginError.textContent = message;
            loginError.classList.remove('d-none');
        }
    }
    
    function hideError() {
        if (loginError) {
            loginError.classList.add('d-none');
        }
    }
    
    function setLoadingState(isLoading) {
        const submitBtn = loginForm.querySelector('button[type="submit"]');
        if (isLoading) {
            submitBtn.disabled = true;
            submitBtn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> Signing In...';
        } else {
            submitBtn.disabled = false;
            submitBtn.innerHTML = 'Sign In';
        }
    }
    
    function showSuccessFeedback() {
        // Add a quick success visual feedback
        const submitBtn = loginForm.querySelector('button[type="submit"]');
        const originalText = submitBtn.innerHTML;
        
        submitBtn.innerHTML = '<i class="fas fa-check"></i> Success!';
        submitBtn.disabled = true;
        submitBtn.classList.remove('btn-primary');
        submitBtn.classList.add('btn-success');
        
        setTimeout(() => {
            submitBtn.innerHTML = originalText;
            submitBtn.disabled = false;
            submitBtn.classList.remove('btn-success');
            submitBtn.classList.add('btn-primary');
        }, 400);
    }
    
    // Clear error when user starts typing again
    if (usernameInput) {
        usernameInput.addEventListener('input', hideError);
    }
    
    if (passwordInput) {
        passwordInput.addEventListener('input', hideError);
    }
    
    // Utility functions
    function checkExistingAuth() {
        const authData = JSON.parse(localStorage.getItem('roamadmin_auth') || sessionStorage.getItem('roamadmin_auth'));
        return !!(authData && authData.isLoggedIn && authData.token);
    }
    
    function redirectToDashboard() {
        window.location.href = 'index.html';
    }
    
    // Demo credentials helper (optional - remove in production)
    const demoHelper = document.createElement('div');
    demoHelper.className = 'text-center mt-3 small text-muted';
    demoHelper.innerHTML = `
        Demo credentials:<br>
        <strong>superadmin</strong> / <strong>superadmin123</strong> (Super Admin)<br>
        <strong>admin</strong> / <strong>admin123</strong> (Admin)<br>
        <strong>operator1</strong> / <strong>operator123</strong> (Operator)<br>
        <strong>viewer1</strong> / <strong>viewer123</strong> (Viewer)
    `;
    document.querySelector('.login-card-body').appendChild(demoHelper);
});

// Add some basic styles for animations
const style = document.createElement('style');
style.textContent = `
    .shake {
        animation: shake 0.5s linear;
    }
    
    @keyframes shake {
        0%, 100% { transform: translateX(0); }
        10%, 30%, 50%, 70%, 90% { transform: translateX(-5px); }
        20%, 40%, 60%, 80% { transform: translateX(5px); }
    }
    
    .btn-success {
        background-color: #28a745 !important;
        border-color: #28a745 !important;
    }
    
    .fa-spinner {
        animation: spin 1s linear infinite;
    }
    
    @keyframes spin {
        0% { transform: rotate(0deg); }
        100% { transform: rotate(360deg); }
    }
`;
document.head.appendChild(style);