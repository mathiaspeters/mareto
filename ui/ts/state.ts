export class State {
    selectedFolder: string | null = null;
    filterString: string = "";
    shouldLimitMinDepth: boolean = false;
    minDepthLimit: number = 0;
    shouldLimitMaxDepth: boolean = false;
    maxDepthLimit: number = 0;
    showFiles: boolean = true;
    showFolders: boolean = true;
    sortOrder: SortOrder = SortOrder.Ascending;
    pathDisplayType: PathDisplayType = PathDisplayType.Relative;
    removeEmptyFolders: boolean = false;
    previewChanges: boolean = true;
    colorTheme: ColorTheme = ColorTheme.System;
}

export enum SortOrder {
    Ascending,
    Descending,
}

export enum PathDisplayType {
    Absolute,
    Relative,
    FileName,
}

export enum ColorTheme {
    System,
    Light,
    Dark,
}