import init, { Komment } from "/pkg/komment.js";

const WORKER_URL = "https://komment.s42.workers.dev";

async function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

window.komment = async function(repo) {
    const container = document.querySelector('.komment');
    if (!container) {
        console.error("Komment: <div class='komment'></div> not found.");
        return;
    }

    container.innerHTML = "Loading discussion...";
    
    // 1. Setup Login Header (Logout is now inline at bottom)
    const controls = document.createElement('div');
    controls.className = 'header-controls';
    controls.style.cssText = "display:flex; justify-content:flex-end; margin-bottom:20px; min-height:40px; align-items:center;";
    
    const loginBtn = document.createElement('button');
    loginBtn.id = 'login-btn';
    loginBtn.innerText = 'Login with GitHub';
    loginBtn.style.display = 'none';

    controls.appendChild(loginBtn);
    container.parentNode.insertBefore(controls, container);

    let instance;
    const term = window.location.href.split('?')[0];

    async function handleOAuth() {
        const params = new URLSearchParams(window.location.search);
        const code = params.get("code");
        if (code) {
            try {
                const res = await fetch(`${WORKER_URL}/api/token`, {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify({ code })
                });
                const data = await res.json();
                if (data.access_token) {
                    localStorage.setItem("gh_token", data.access_token);
                }
                window.history.replaceState({}, document.title, window.location.pathname);
            } catch (err) { console.error("Komment Auth Error:", err); }
        }
    }

    async function refresh(retryCount = 0) {
        const token = localStorage.getItem("gh_token");
        try {
            const data = await instance.fetch_discussion();
            instance.render(container.id || 'komment-widget', data);
            attachHandlers();
        } catch (e) {
            if (e === "DISCUSSION_NOT_FOUND" && token) {
                if (retryCount < 3) {
                    container.innerText = "Waiting for GitHub...";
                    await sleep(2000);
                    return refresh(retryCount + 1);
                } else {
                    await autoCreate();
                }
            } else if (e === "DISCUSSION_NOT_FOUND" || e.toString().includes("401")) {
                container.innerHTML = `<div class="info"><p>Please login with GitHub to view or post comments.</p></div>`;
                loginBtn.style.display = 'flex';
            } else {
                container.innerHTML = `<div class="error">Error: ${e}</div>`;
            }
        }
    }

    async function autoCreate() {
        container.innerText = "Initializing thread...";
        try {
            const [owner, name] = repo.split("/");
            await instance.create_discussion(owner, name, "General", term, `Comments for ${term}`);
            await sleep(2000);
            await refresh();
        } catch (err) { container.innerHTML = `<div class="error">Failed to create discussion: ${err}</div>`; }
    }

    function attachHandlers() {
        // Post Comment
        const submitBtn = document.getElementById("komment-submit");
        if (submitBtn) {
            submitBtn.onclick = async () => {
                const textarea = document.getElementById("komment-textarea");
                const body = textarea.value.trim();
                if (!body) return;
                submitBtn.disabled = true;
                const originalText = submitBtn.innerText;
                submitBtn.innerText = "Posting...";
                try {
                    const discId = container.getAttribute("data-discussion-id");
                    await instance.post_comment(discId, body);
                    textarea.value = "";
                    await refresh();
                } catch (err) { alert("Failed to post: " + err); }
                finally { submitBtn.disabled = false; submitBtn.innerText = originalText; }
            };
        }

        // Inline Logout
        const logoutBtn = document.getElementById("logout-btn-inline");
        if (logoutBtn) {
            logoutBtn.onclick = () => {
                localStorage.removeItem("gh_token");
                window.location.reload();
            };
        }

        // Delegate Edit/Delete/Save/Cancel
        container.onclick = async (e) => {
            const btn = e.target;
            const id = btn.getAttribute("data-id");
            if (!id) return;

            if (btn.classList.contains("komment-edit-btn")) {
                document.getElementById(`body-${id}`).style.display = "none";
                document.getElementById(`edit-form-${id}`).style.display = "block";
            } else if (btn.classList.contains("komment-cancel-btn")) {
                document.getElementById(`body-${id}`).style.display = "block";
                document.getElementById(`edit-form-${id}`).style.display = "none";
            } else if (btn.classList.contains("komment-save-btn")) {
                const body = document.getElementById(`textarea-${id}`).value.trim();
                if (!body) return;
                btn.disabled = true;
                try { await instance.update_comment(id, body); await refresh(); }
                catch (err) { alert("Error: " + err); btn.disabled = false; }
            } else if (btn.classList.contains("komment-delete-btn")) {
                if (!confirm("Delete this comment?")) return;
                btn.disabled = true;
                try { await instance.delete_comment(id); await refresh(); }
                catch (err) { alert("Error: " + err); btn.disabled = false; }
            }
        };
    }

    // Initialize
    if (!container.id) container.id = 'komment-widget';
    await init();
    await handleOAuth();
    
    const token = localStorage.getItem("gh_token");
    if (!token) {
        loginBtn.style.display = 'flex';
    }

    loginBtn.onclick = () => {
        const CLIENT_ID = "Iv23liQokIChd3ylSI7R";
        window.location.href = `https://github.com/login/oauth/authorize?client_id=${CLIENT_ID}&scope=public_repo&redirect_uri=${encodeURIComponent(term)}`;
    };

    instance = new Komment({
        repo: repo,
        mapping: "title",
        term: term,
        token: token,
        api_url: `${WORKER_URL}/api/graphql`,
        category: "General"
    });

    await refresh();
};
