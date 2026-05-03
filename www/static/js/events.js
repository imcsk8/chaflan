/**
 * Chaflan Event Manager - Client-side logic using jQuery
 * 
 * This file contains functions for handling user authentication and event management.
 */

$(document).ready(() => {
    // Initialize Login Form
    $('#loginForm').on('submit', handleLogin);

    // Initialize Add Event Form
    $('#addEventForm').on('submit', handleAddEvent);
});

/**
 * Handles user login via JSON POST.
 * On success, redirects to the events list.
 * @param {Event} e - The form submit event
 */
async function handleLogin(e) {
    e.preventDefault();
    const user = $('#user').val();
    const password = $('#password').val();
    const $messageDiv = $('#message');

    try {
        const response = await fetch('/login', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ user, password })
        });

        if (response.ok) {
            $messageDiv.css('color', 'green').text('Login successful! Redirecting...');
            setTimeout(() => window.location.href = '/events', 1000);
        } else {
            const error = await response.text();
            $messageDiv.css('color', 'red').text('Login failed: ' + error);
        }
    } catch (err) {
        console.error('Login error:', err);
        $messageDiv.css('color', 'red').text('An error occurred. Please try again.');
    }
}

/**
 * Handles new event creation via JSON POST.
 * Formats dates to the expected ISO format.
 * @param {Event} e - The form submit event
 */
async function handleAddEvent(e) {
    e.preventDefault();
    const $messageDiv = $('#message');
    
    // Format dates to ISO string that Diesel expects (YYYY-MM-DDTHH:MM:SS)
    const formatDate = (val) => val ? val + ":00" : null;

    const payload = {
        id: "00000000-0000-0000-0000-000000000000",
        name: $('#name').val(),
        venue: $('#venue').val(),
        address: $('#address').val() || null,
        url: $('#url').val(),
        starts_at: formatDate($('#starts_at').val()),
        ends_at: formatDate($('#ends_at').val()),
        contactname: $('#contactname').val() || null,
        comments: $('#comments').val() || null,
        image: null
    };

    try {
        const response = await fetch('/events/add', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(payload)
        });

        if (response.ok) {
            const eventId = await response.json();
            const imageFile = $('#image')[0].files[0];

            // If an image was selected, upload it now
            if (imageFile) {
                $messageDiv.css('color', 'blue').text('Event created. Uploading image...');
                try {
                    const imgResponse = await fetch(`/events/${eventId}/image`, {
                        method: 'POST',
                        body: imageFile
                    });
                    if (!imgResponse.ok) {
                        const imgErr = await imgResponse.text();
                        console.error('Image upload failed:', imgErr);
                    }
                } catch (imgErr) {
                    console.error('Error during image upload:', imgErr);
                }
            }

            $messageDiv.css('color', 'green').text('Event created successfully! Redirecting...');
            setTimeout(() => window.location.href = '/events', 1500);
        } else {
            const error = await response.text();
            $messageDiv.css('color', 'red').text('Failed to create event: ' + error);
        }
    } catch (err) {
        console.error('Add event error:', err);
        $messageDiv.css('color', 'red').text('An error occurred. Please try again.');
    }
}
