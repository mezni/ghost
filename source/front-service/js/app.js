// ==============================
// Custom JavaScript for RoamAdmin Dashboard
// ==============================

$(document).ready(function() {
    // Load shared components
    loadComponents();

    // Initialize UI
    initCustomFeatures();

    // Demo button
    initDemoButton();

    // Dynamic page loader
    initPageLoader();
});

// ------------------------------
// Load header, sidebar, footer
// ------------------------------
function loadComponents() {
    $('#header').load('pages/header.html');
    $('#sidebar').load('pages/sidebar.html', function() {
        // Re-init AdminLTE treeview after sidebar load
        $('.has-treeview').each(function() {
            const $tree = $(this);
            $tree.find('> a').on('click', function(e) {
                e.preventDefault();
                $tree.toggleClass('menu-open');
                $tree.find('> .nav-treeview').slideToggle(200);
            });
        });

        // Highlight active menu based on current URL
        highlightActiveSidebarLink();
    });
    $('#footer').load('pages/footer.html');
}

// ------------------------------
// Custom animations and UI hooks
// ------------------------------
function initCustomFeatures() {
    console.log('✅ RoamAdmin Initialized');
    $('.card').addClass('fade-in');

    // Sidebar toggle
    $(document).on('click', '[data-widget="pushmenu"]', function() {
        setTimeout(() => $(document).trigger('sidebarToggled'), 300);
    });

    $(document).on('sidebarToggled', () => console.log('Sidebar toggled'));
}

// ------------------------------
// Demo button functionality
// ------------------------------
function initDemoButton() {
    $(document).on('click', '#demoButton', function() {
        const $btn = $(this);
        const $msg = $('#demoMessage');

        $btn.prop('disabled', true)
            .html('<i class="fas fa-spinner fa-spin me-2"></i>Loading...');

        setTimeout(() => {
            $msg.removeClass('d-none').slideDown();
            setTimeout(() => {
                $btn.prop('disabled', false)
                    .html('<i class="fas fa-rocket me-2"></i>Click Me!');
                setTimeout(() => $msg.slideUp(), 3000);
            }, 1000);
        }, 1500);
    });
}

// ------------------------------
// Dynamic Page Loader
// ------------------------------
function initPageLoader() {
    $(document).on('click', '.nav-sidebar a', function(e) {
        const url = $(this).attr('href');
        if (url && url.startsWith('pages/')) {
            e.preventDefault();

            // Highlight the active item
            $('.nav-link').removeClass('active');
            $(this).addClass('active');

            // Open parent treeview if submenu
            const parent = $(this).closest('.has-treeview');
            if (parent.length) {
                parent.addClass('menu-open');
                parent.find('> .nav-treeview').slideDown(200);
                parent.children('a').addClass('active');
            }

            // Load the page into content area
            $('#content-area').load(url, function(response, status) {
                if (status === "error") {
                    $('#content-area').html(`
                        <div class="alert alert-danger mt-3">
                            <i class="fas fa-exclamation-triangle me-2"></i>
                            Failed to load ${url}.
                        </div>
                    `);
                } else {
                    console.log(`✅ Loaded ${url}`);
                }
            });
        }
    });
}

// ------------------------------
// Highlight active sidebar link based on URL
// ------------------------------
function highlightActiveSidebarLink() {
    const path = window.location.pathname.split("/").pop();
    $('#sidebar a.nav-link').removeClass('active');

    $('#sidebar a.nav-link').each(function() {
        const href = $(this).attr('href');
        if (href === path) {
            $(this).addClass('active');
            const parent = $(this).closest('.has-treeview');
            if (parent.length) {
                parent.addClass('menu-open');
                parent.children('a.nav-link').addClass('active');
                parent.find('> .nav-treeview').show();
            }
        }
    });
}

// ------------------------------
// Utility functions
// ------------------------------
const AppUtils = {
    showNotification: function(title, message, type = 'info') {
        const icons = {
            info: 'fas fa-info-circle',
            success: 'fas fa-check-circle',
            warning: 'fas fa-exclamation-triangle',
            danger: 'fas fa-exclamation-circle'
        };
        const icon = icons[type] || icons.info;

        $.notify({
            title: `<strong>${title}</strong>`,
            message,
            icon
        }, {
            type,
            animate: {
                enter: 'animated fadeInDown',
                exit: 'animated fadeOutUp'
            },
            placement: { from: "top", align: "right" },
            offset: 20,
            delay: 3000
        });
    }
};

window.AppUtils = AppUtils;
