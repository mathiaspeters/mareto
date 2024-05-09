# Next up
- [x] Free-text filtering with option to toggle regex and case insensitivity
- [ ] Make the filter regex and case insensitive buttons change color depending on if they are active or not (standard button color if active, background color if not active)
- [x] Theme selection
- [ ] Refactor data model to make apply changes possible

# Later
- [ ] Performance
    - [ ] It's currently very slow to change settings when there are plenty of files in the selected folder 
- [ ] Editor improvements
    - [ ] Disallow removing lines. If user tries to remove a line, instead make the line empty but keep it there
    - [ ] Disallow adding lines in between existing items
    - [ ] New lines can be added at the end of the list, for which new objects would be added in the folder structure
- [ ] Implement apply changes
    - [ ] Rename files accordingly
    - [ ] Make remove empty folders setting work 
    - [ ] Make preview changes setting work 
    - [ ] Show error if one occurred during apply changes
- [ ] Create config file to persist user settings and potentially current state 
- [ ] Create help page to explain how it's working
- [ ] Show invalid regex error to the user

# Future
- [ ] Find and replace
- [ ] Multiple cursors in editor
- [ ] Automated testing
- [ ] CI