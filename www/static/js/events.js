/**
 * Chaflan Event Manager - Client-side logic
 * 
 * This file contains functions for handling user authentication and event management.
 */

document.addEventListener('DOMContentLoaded', () => {
    // Initialize Login Form
    const loginForm = document.getElementById('loginForm');
    if (loginForm) {
        loginForm.addEventListener('submit', handleLogin);
    }

    // Initialize Add Event Form
    const addEventForm = document.getElementById('addEventForm');
    if (addEventForm) {
        addEventForm.addEventListener('submit', handleAddEvent);
    }
});

/**
 * Handles user login via JSON POST.
 * On success, redirects to the events list.
 * @param {Event} e - The form submit event
 */
async function handleLogin(e) {
    e.preventDefault();
    const user = document.getElementById('user').value;
    const password = document.getElementById('password').value;
    const messageDiv = document.getElementById('message');

    try {
        const response = await fetch('/login', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ user, password })
        });

        if (response.ok) {
            // Note: Token is returned in JSON but also set as an HttpOnly cookie (BFF)
            messageDiv.style.color = 'green';
            messageDiv.textContent = 'Login successful! Redirecting...';
            setTimeout(() => window.location.href = '/events', 1000);
        } else {
            const error = await response.text();
            messageDiv.style.color = 'red';
            messageDiv.textContent = 'Login failed: ' + error;
        }
    } catch (err) {
        console.error('Login error:', err);
        messageDiv.style.color = 'red';
        messageDiv.textContent = 'An error occurred. Please try again.';
    }
}

/**
 * Handles new event creation via JSON POST.
 * Formats dates to the expected ISO format.
 * @param {Event} e - The form submit event
 */
async function handleAddEvent(e) {
    e.preventDefault();
    const messageDiv = document.getElementById('message');
    
    // Format dates to ISO string that Diesel expects (YYYY-MM-DDTHH:MM:SS)
    const formatDate = (val) => val ? val + ":00" : null;

    const payload = {
        id: "00000000-0000-0000-0000-000000000000", // Server generates real UUID
        name: document.getElementById('name').value,
        venue: document.getElementById('venue').value,
        address: document.getElementById('address').value || null,
        url: document.getElementById('url').value,
        starts_at: formatDate(document.getElementById('starts_at').value),
        ends_at: formatDate(document.getElementById('ends_at').value),
        contactname: document.getElementById('contactname').value || null,
        comments: document.getElementById('comments').value || null,
        image: null
    };

    try {
        const response = await fetch('/events/add', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(payload)
        });

        if (response.ok) {
            messageDiv.style.color = 'green';
            messageDiv.textContent = 'Event created successfully! Redirecting...';
            setTimeout(() => window.location.href = '/events', 1500);
        } else {
            const error = await response.text();
            messageDiv.style.color = 'red';
            messageDiv.textContent = 'Failed to create event: ' + error;
        }
    } catch (err) {
        console.error('Add event error:', err);
        messageDiv.style.color = 'red';
        messageDiv.textContent = 'An error occurred. Please try again.';
    }
}
