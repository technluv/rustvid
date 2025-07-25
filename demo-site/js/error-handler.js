// Global error handler for demo site
window.addEventListener('error', function(e) {
    console.log('Caught error:', e.message);
    // Prevent the error from breaking the page
    e.preventDefault();
});

// Fix for querySelector errors
document.addEventListener('DOMContentLoaded', function() {
    // Override problematic selectors
    const originalQuerySelector = document.querySelector;
    document.querySelector = function(selector) {
        if (!selector || selector === '#' || selector === '') {
            return null;
        }
        try {
            return originalQuerySelector.call(this, selector);
        } catch (e) {
            console.warn('Invalid selector:', selector);
            return null;
        }
    };
});

// Auto-collect errors for debugging
window.errorLog = [];
window.onerror = function(msg, url, line, col, error) {
    window.errorLog.push({
        message: msg,
        source: url,
        line: line,
        column: col,
        error: error,
        timestamp: new Date().toISOString()
    });
    return true; // Prevent default error handling
};