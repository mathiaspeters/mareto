Note: This roadmap is a living document that can and will be changed

# 1.0
- [x] Free-text filtering with option to toggle regex and case insensitivity
- [x] Make the filter regex and case insensitive buttons change color depending on if they are active or not (standard button color if active, background color if not active)
- [x] Theme selection
- [ ] Performance
    - [ ] It's currently very slow to change settings when there are plenty of files in the selected folder 
        - [ ] When the currently active filters are updated, if it's strictly more restrictive, only the visible items need to be checked and if the new filters are strictly more permissive, only the non-visible items need to be checked
        - [ ] Sorting
            - [ ] Changing from either sort order to no sorting doesn't require a sorting
            - [ ] Sorting only needs to be done when changing the view type or when changing the sorting option to either ascending or descending
        - [ ] Explore multi-threading and SIMD
            - [ ] Filtering can be done on a background thread to not slow down the UI
            - [ ] Using rayon could helps speed it up
            - [ ] Making the item filtering code more SIMD-friendly could help
- [ ] Implement editor handling
    - [ ] React to editor events
    - [ ] Disallow removing lines. If user tries to remove a line, instead make the line empty but keep it there
    - [ ] Disallow adding lines in between existing items
    - [ ] A single newline can be added at the end of the editor. Once the user has put some content in that line, another newline can be added at the end. New items would be created for these 
- [ ] Implement apply changes
    - [ ] Rename files accordingly
    - [ ] Make remove empty folders setting work 
    - [ ] Make preview changes setting work 
    - [ ] Show error if one occurred during apply changes
- [ ] Create config file to persist user settings and potentially current state 
- [ ] Create help page to explain how it's working
- [x] Show invalid regex error to the user
- [ ] Automated testing
- [ ] CI
- [ ] Test manually on macos and windows

# Post-1.0
- [ ] Find and replace
- [ ] Multiple cursors in editor