import { invoke } from "@tauri-apps/api/tauri";
import { State } from "./ts/state";

let filterInput: HTMLElement | null;
let clearFilterButton: HTMLElement | null;
let regexButton: HTMLElement | null;

const state = new State();

const NO_FOLDER_SELECTED_LABEL = "No folder selected";


//async function greet() {
//  if (greetMsgEl && greetInputEl) {
//    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//    greetMsgEl.textContent = await invoke("greet", {
//      name: greetInputEl.value,
//    });
//  }
//}

window.addEventListener("DOMContentLoaded", () => {
    myfunc("folder-name-txt", (txt) => {
        txt.innerHTML = state.selectedFolder == null ?
            NO_FOLDER_SELECTED_LABEL :
            state.selectedFolder;
    })

    myeventfunc("open-folder-btn", "click", (e) => {
        console.log("openFolderButton: " + e);
    });

    myeventfunc("apply-changes-btn", "click", (e) => {
        console.log("applyChangesButton clicked.\n Current state:\n" + JSON.stringify(state));
    });

    setcheckbox("show-files-chkbox", state.showFiles);
    mychkboxeventfunc("show-files-chkbox", "change", (checked) => {
        state.showFiles = checked;
    });

    setcheckbox("show-folders-chkbox", state.showFolders);
    mychkboxeventfunc("show-folders-chkbox", "change", (checked) => {
        state.showFolders = checked;
    });

    setcheckbox("remove-empty-chkbox", state.removeEmptyFolders);
    mychkboxeventfunc("remove-empty-chkbox", "change", (checked) => {
        state.removeEmptyFolders = checked;
    });

    setcheckbox("preview-changes-chkbox", state.previewChanges);
    mychkboxeventfunc("preview-changes-chkbox", "change", (checked) => {
        state.previewChanges = checked;
    });
});

const myfunc = (id: string, op: (e: HTMLElement) => void) => {
    let elem = document.getElementById(id);
    if (elem != null) {
        op(elem);
    }
}

const setcheckbox = (id: string, value: boolean) => {
    myfunc(id, (elem) => {
        let chkbox = elem as HTMLInputElement;
        chkbox.checked = value;
    })
}

const myeventfunc = (id: string, eventType: string, op: (e: Event) => void) => {
    document.getElementById(id)?.addEventListener(eventType, (event) => op(event));
}

const mychkboxeventfunc = (id: string, eventType: string, op: (checked: boolean) => void) => {
    document.getElementById(id)?.addEventListener(eventType, (event) => {
        let target = event.currentTarget as HTMLInputElement;
        op(target?.checked ?? false);
    });
}