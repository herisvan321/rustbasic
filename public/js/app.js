/* ---------------------------------------------------------
 * 📑 LABEL: SCRIPTS UTAMA (app.js)
 * Menangani notifikasi toast dan data flash dari server.
 * --------------------------------------------------------- */

window.addEventListener('DOMContentLoaded', () => {
    const flashData = document.getElementById('flash-data');
    if (!flashData) return;

    const success = flashData.getAttribute('data-success');
    const error = flashData.getAttribute('data-error');

    if (success) {
        window.dispatchEvent(new CustomEvent('show-toast', { detail: { message: success, type: 'success' } }));
    }
    if (error) {
        window.dispatchEvent(new CustomEvent('show-toast', { detail: { message: error, type: 'error' } }));
    }
});

window.addEventListener('show-toast', (e) => {
    // Mengambil instance data Alpine.js dari elemen body
    const el = document.body;
    // Alpine v3 menggunakan _x_dataStack
    if (el && el._x_dataStack) {
        const data = el._x_dataStack[0];
        data.toast.message = e.detail.message;
        data.toast.type = e.detail.type;
        data.toast.show = true;
        setTimeout(() => data.toast.show = false, 4000);
    } else if (el && el.__x) {
        // Fallback untuk versi lain jika ada
        const data = el.__x.$data;
        data.toast.message = e.detail.message;
        data.toast.type = e.detail.type;
        data.toast.show = true;
        setTimeout(() => data.toast.show = false, 4000);
    }
});
