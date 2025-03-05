// Copyright 2024 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub(crate) const ICON: &[u8] = include_bytes!("../../docs/logo_small.png");

pub(crate) const CSS_STYLE: &str = "
body {
    font-family: Roboto, sans-serif;
    margin: 0;
    padding: 20px;
    background: #f5f5f5;
    color: #333;
}

.header {
    background: #fff;
    padding: 20px;
    border-radius: 8px;
    margin-bottom: 20px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.logo {
    vertical-align: -10%;
}

.header h1 {
    margin: 0;
    color: #2d3748;
}

.summary {
    margin-bottom: 20px;
    padding: 15px;
    background: #fff;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.diff-entry {
    background: #fff;
    margin-bottom: 30px;
    padding: 20px;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.diff-entry h2 {
    margin-top: 0;
    color: #2d3748;
    border-bottom: 2px solid #edf2f7;
    padding-bottom: 10px;
}

.comparison-container {
    display: flex;
    gap: 20px;
    margin-top: 15px;
}

.image-container {
    display: flex;
    gap: 20px;
    flex-wrap: wrap;
    flex: 1;
}

.image-box {
    flex: 1;
    min-width: 250px;
    max-width: 400px;
}

.image-box h3 {
    margin: 0 0 10px 0;
    color: #4a5568;
    font-size: 1rem;
}

.image-box img {
    max-width: 100%;
    border: 1px solid #e2e8f0;
    border-radius: 4px;
}

.stats-container {
    width: 200px;
    flex-shrink: 0;
    background: #f8fafc;
    padding: 15px;
    border-radius: 6px;
    border: 1px solid #e2e8f0;
}

.stat-item {
    margin-bottom: 15px;
}

.stat-label {
    font-size: 0.875rem;
    color: #64748b;
    margin-bottom: 4px;
}

.stat-value {
    font-size: 1.25rem;
    font-weight: 600;
    color: #2d3748;
}

.stat-value.ok {
    color: #77d906;
}

.stat-value.warning {
    color: #d97706;
}

.stat-value.error {
    color: #dc2626;
}

@media (max-width: 1200px) {
    .comparison-container {
        flex-direction: column-reverse;
    }

    .stats-container {
        width: auto;
        display: flex;
        flex-wrap: wrap;
        gap: 20px;
    }

    .stat-item {
        flex: 1;
        min-width: 150px;
        margin-bottom: 0;
    }
}

@media (max-width: 768px) {
    .image-box {
        min-width: 100%;
    }
}

img.zoom:hover {
    cursor: pointer;
    transform: scale(1.05);
}

dialog {
    width: 80%;
    height: 80%;
    max-width: 800px;
    max-height: 820px;
    padding: 0;
    border: none;
    border-radius: 10px;
    box-shadow: 0 0 15px rgba(0, 0, 0, 0.3);
}

.zoomed-image {
    object-fit: contain;
    image-rendering: -moz-crisp-edges;
    image-rendering: -o-crisp-edges;
    image-rendering: -webkit-optimize-contrast;
    -ms-interpolation-mode: nearest-neighbor;
    image-rendering: pixelated;
}

.toggle-switch {
  position: relative;
  display: inline-block;
  width: 60px;
  height: 30px;
  margin-right: 1em;
}
.toggle-switch input {
  opacity: 0;
  width: 0;
  height: 0;
}
.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #ccc;
  transition: .1s;
  border-radius: 34px;
}
.slider:before {
  position: absolute;
  content: \"\";
  height: 22px;
  width: 22px;
  left: 4px;
  bottom: 4px;
  background-color: white;
  transition: .1s;
  border-radius: 50%;
}
input:checked + .slider {
  background-color: #3c3;
}
input:checked + .slider:before {
  transform: translateX(30px);
}

.accept-button {
  padding: 12px 24px;
  margin-bottom: 1em;
  font-size: 16px;
  font-weight: 500;
  color: white;
  background-color: #4CAF50;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  gap: 8px;
}
.accept-button:hover {
  background-color: #45A049;
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}
.accept-button:active {
  transform: translateY(0px);
  box-shadow: none;
}
.accept-button:disabled {
  background-color: #CCCCCC;
  cursor: not-allowed;
  transform: none;
}
#errorMsg {
    background-color: #fef2f2;
    border: 1px solid #f87171;
    border-radius: 6px;
    padding: 16px;
    margin: 12px 0;
    display: none;
    align-items: flex-start;
    gap: 12px;
    max-width: 600px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}
.tabs {
  margin-top: 8px;
  display: flex;
}
.tab {
  margin-left: 10px;
  margin-right: 10px;
  background: none;
  cursor: pointer;
  font-size: 16px;
  color: #666;
}
.tab.active {
  color: #444;
  border-bottom: 2px solid #444;
}

.hint {
    color: #666;
    font-size: 80%;
}
";

pub(crate) const JS_CODE: &str = "
function openImageDialog(img) {
    const dialog = document.getElementById('imageDialog');
    const zoomedImg = document.getElementById('zoomedImage');
    zoomedImg.src = img.src;
    if (img.width < img.height) {
        zoomedImg.style.width = \"100%\";
        zoomedImg.style.height = \"auto\";
    } else {
        zoomedImg.style.width = \"auto\";
        zoomedImg.style.height = \"100%\";
    }
    dialog.showModal();
}

function closeImageDialog() {
    const dialog = document.getElementById('imageDialog');
    dialog.close();
}

document.getElementById('imageDialog').addEventListener('click', function(event) {
    closeImageDialog();
});

var selected = new Set();
function toggle(event) {
    let node = event.target.parentNode.parentNode;
    let name = node.childNodes[2].textContent;
    if (event.target.checked) {
        selected.add(name);
        node.style.color = \"#3a3\";
    } else {
        selected.delete(name);
        node.style.color = \"#333\";
    }
    updateAcceptButton()
}

function updateAcceptButton() {
    let text = document.getElementById('acceptText');
    text.textContent = \"Accept selected cases (\" + selected.size + \" / \" + nTests + \")\";
    let button = document.getElementById('acceptButton');
    button.disabled = (selected.size === 0);
}

function switchDiffTab(id, selected, n) {
    for (let idx = 0; idx < n; idx++) {
        document.getElementById(`tab-diff-${id}-${idx}`).classList.remove('active');
        document.getElementById(`img-diff-${id}-${idx}`).style.display = 'none';
    }
    document.getElementById(`tab-diff-${id}-${selected}`).classList.add('active');
    document.getElementById(`img-diff-${id}-${selected}`).style.display = 'inline';
}

async function acceptTests() {
    let text = document.getElementById('acceptText');
    text.textContent = \"Updating \" + selected.size + \" cases ...\";
    let button = document.getElementById('acceptButton');
    button.disabled = true;

    try {
        const url = '/update';
        const response = await fetch(url, {
            method: 'POST',
            headers: {
               \"Content-Type\": \"application/json\",
            },
            body: JSON.stringify({ accepted_names: Array.from(selected) })
        });
        if (!response.ok) {
          throw new Error(`Response status: ${response.status}`);
        } else {
          for (i = 0; i < nTests; i++) {
                document.getElementById(\"t\" + i).checked = false;
          }
          location.reload();
        }
    } catch (e) {
        let error = document.getElementById('errorMsg');
        error.textContent = e.message;
        error.style.display = \"flex\";
        text.textContent = \"Try update again\";
        button.disabled = false;
    }
}
";
