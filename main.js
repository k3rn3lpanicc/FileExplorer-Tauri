import './style.css'
import { invoke } from '@tauri-apps/api/tauri'
import { convertFileSrc } from '@tauri-apps/api/tauri';
let current_path = "";
let is_search = false;
let icon_map = {
  "txt": "icons/txt.png",
  "pdf": "icons/pdf.png",
  "py": "icons/py.png",
  "png": "icons/png2.png",
  "cs": "icons/cs.png",
  "jpg": "icons/jpg.png",
  "dll": "icons/dll.png",
}

function setLoadingStart() {
  document.getElementById("loading").style.visibility = "visible";
}
function setLoadingFinish() {
  document.getElementById("loading").style.visibility = "hidden";
}

async function search(path, pattern) {
  setLoadingStart();
  return await invoke("search", { "path": path, "pattern": pattern });
}

async function recursiveSearch(path, pattern) {
  setLoadingStart();
  return await invoke("recursive_search_entrypoint", { "path": path, "pattern": pattern });
}

async function setContent(path, isDrive = false) {
  is_search = false;
  current_path = path;
  while (path.endsWith("\\")) path = path.slice(0, -1);
  path += "\\";
  // clear the app children
  document.getElementById("app").innerHTML = "";
  // create the Back button
  if (!isDrive)
    document.getElementById("app").innerHTML += `
  <div class = "folder" id = "back">
    <img src="/icon.png" alt="folder" class = "folderImg">
    <div class = "folderName"><- Back</div>
  </div>
  `;
  let folders = [];
  let files = [];
  try {
    folders = isDrive ? await invoke('get_drives') : await invoke("get_folders", { "input": path });
    files = isDrive ? [] : await invoke("get_files", { "input": path });
    document.getElementById("filenameText").value = path;
  } catch (e) {
    console.log(e);
    document.getElementById("app").innerHTML = ``;
    return;
  }
  for (let i = 0; i < folders.length; i++) {
    document.getElementById("app").innerHTML += `
    <div title="${folders[i].split("\\").slice(-1)}\n${folders[i]}" class = "folder" id = "${folders[i]}">
      <img src="/icon.png" alt="folder" class = "folderImg">
      <label class = "folderName">${isDrive ? folders[i] : folders[i].split("\\")[folders[i].split("\\").length - 1]}</label>
    </div>
    `;
  }
  for (let i = 0; i < files.length; i++) {
    let extension = files[i].split(".")[files[i].split(".").length - 1];
    let icon = "";
    if (extension in icon_map) icon = icon_map[extension];
    else icon = "icons/file.png";
    let is_pic = false;
    if (extension.toLowerCase() == "png" || extension.toLowerCase() == "jpg") {
      icon = convertFileSrc(files[i],);
      is_pic = true;
    }
    document.getElementById("app").innerHTML += `
    <div title="${files[i].split("\\").slice(-1)}\n${files[i]}" class = "file" id = "${files[i]}">
      <img src="${icon}" alt="file" class = "${is_pic ? "fileImg" : "fileIcon"}">
      <label class = "fileName">${files[i].split("\\")[files[i].split("\\").length - 1]}</label>
    </div>
    `;
  }
  for (let i = 0; i < files.length; i++) {
    document.getElementById(files[i]).addEventListener("dblclick", async function () {
      await invoke("open_explorer", { "path": files[i] });
    });
  }
  for (let i = 0; i < folders.length; i++) {
    document.getElementById(folders[i]).addEventListener("click", async function () {
      await setContent(folders[i] + "\\", false);
    });
  }
  if (!isDrive)
    document.getElementById("back").addEventListener("click", async function () {
      while (path.endsWith("\\")) path = path.slice(0, -1);
      let _last = path.split("\\");
      _last.pop();
      _last = _last.join("\\");
      await setContent(_last == "" ? "" : _last + "\\", _last == "" ? true : false);
    });
}

const myDiv = document.getElementById('myDiv');
myDiv.addEventListener('contextmenu', (e) => {
  console.log("lol");
  e.preventDefault(); // Prevent the default context menu from appearing
  const menu = document.createElement('div'); // Create a new <ul> element for the context menu
  menu.id = 'context_menu'; // Give the menu an ID
  menu.innerHTML = `
        <label class="option">Option 1</label>
        <label class="option">Option 2</label>
        <label class="option">Option 3</label>
      `; // Add some options to the menu
  menu.style.position = 'absolute'; // Position the menu at the mouse pointer
  menu.style.left = `${e.pageX}px`;
  menu.style.top = `${e.pageY}px`;
  document.body.appendChild(menu); // Add the menu to the page
  // Remove the menu when the user clicks outside of it
  const removeMenu = () => {
    document.body.removeChild(menu);
    document.removeEventListener('click', removeMenu);
  };
  document.addEventListener('click', removeMenu);
});



const app = document.getElementById('app');
app.addEventListener('contextmenu', (e) => {
  console.log("lol");
  e.preventDefault(); // Prevent the default context menu from appearing
  const menu = document.createElement('div'); // Create a new <ul> element for the context menu
  menu.id = 'context_menu'; // Give the menu an ID
  menu.innerHTML = `
        <label class="option" id="refresh_content">Refresh</label>
        <label class="option">Open in Terminal</label>
        <label class="option">Open with code</label>
        <label class="option" id="open_windows">Open in windows</label>
      `; // Add some options to the menu
  menu.style.position = 'absolute'; // Position the menu at the mouse pointer
  menu.style.left = `${e.pageX}px`;
  menu.style.top = `${e.pageY}px`;
  document.body.appendChild(menu); // Add the menu to the page
  document.getElementById("refresh_content").addEventListener("click", async function () {
    await setContent(current_path, current_path == "" ? true : false);
  });
  document.getElementById("open_windows").addEventListener("click", async function () {
    let path = current_path;
    while (path.endsWith("\\")) path = path.slice(0, -1);
    await invoke("open_explorer", { "path": path });
  });
  // Remove the menu when the user clicks outside of it
  const removeMenu = () => {
    document.body.removeChild(menu);
    document.removeEventListener('click', removeMenu);
  };
  document.addEventListener('click', removeMenu);
});

let drives = await invoke('get_drives');
await setContent(drives[0], false);
while (myDiv.childElementCount > 1) {
  myDiv.removeChild(myDiv.lastChild);
}
for (let i = 0; i < drives.length; i++) {
  document.getElementById("myDiv").innerHTML += `
  <div><img src="disk.png" class="disk"><label class="drive" id="${drives[i]}">Local Disk (${drives[i]})</label></div>
  `;
}
for (let i = 0; i < drives.length; i++) {
  document.getElementById(drives[i]).addEventListener("click", async function () {
    await setContent(drives[i], false);
  }
  );
}

document.getElementById("filenameText").addEventListener("keyup", function (event) {
  if (event.key == "Enter") {
    setContent(document.getElementById("filenameText").value, false);
  }
}
);

setLoadingFinish();

document.getElementById("search").addEventListener("click", async function () {
  is_search = true;
  let last_path = current_path;
  let pattern = document.getElementById("searchText").value;
  let res;
  if (document.getElementById("recursive").checked) {
    res = await recursiveSearch(current_path, pattern);
  }
  else {
    res = await search(current_path, pattern);
  }
  setLoadingFinish();
  let files = res[0];
  let folders = res[1];
  document.getElementById("app").innerHTML = "";
  document.getElementById("app").innerHTML += `
  <div class = "folder" id = "back">
    <img src="/icon.png" alt="folder" class = "folderImg">
    <div class = "folderName"><- Back</div>
  </div>
  `;
  for (let i = 0; i < folders.length; i++) {
    document.getElementById("app").innerHTML += `
    <div title="${folders[i].split("\\").slice(-1)}" class = "folder" id = "${folders[i]}">
      <img src="/icon.png" alt="folder" class = "folderImg">
      <label class = "folderName">${folders[i].split("\\")[folders[i].split("\\").length - 1]}</label>
    </div>
    `;
  }
  for (let i = 0; i < files.length; i++) {
    let extension = files[i].split(".")[files[i].split(".").length - 1];
    let icon = "";
    if (extension in icon_map) icon = icon_map[extension];
    else icon = "icons/file.png";
    let is_pic = false;
    if (extension.toLowerCase() == "png" || extension.toLowerCase() == "jpg") {
      icon = convertFileSrc(files[i],);
      is_pic = true;
    }
    document.getElementById("app").innerHTML += `
    <div title="${files[i].split("\\").slice(-1)}" class = "file" id = "${files[i]}">
      <img src="${icon}" alt="file" class = "${is_pic ? "fileImg" : "fileIcon"}">
      <label class = "fileName">${files[i].split("\\")[files[i].split("\\").length - 1]}</label>
    </div>
    `;
  }
  for (let i = 0; i < folders.length; i++) {
    document.getElementById(folders[i]).addEventListener("click", async function () {
      await setContent(folders[i] + "\\", false);
    });
  }
  document.getElementById("back").addEventListener("click", async function () {
    await setContent(last_path, false);
  });
});
