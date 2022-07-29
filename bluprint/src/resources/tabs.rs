use super::map_file::MapFile;

#[derive(Default)]
pub struct Tabs {
    tabs: Vec<MapFile>,
    current_tab: Option<usize>,
}

impl Tabs {
    pub fn current_tab(&self) -> Option<&MapFile> {
        if let Some(tab) = self.current_tab {
            Some(self.tabs.get(tab).expect("Tab index out of bounds"))
        } else {
            None
        }
    }

    pub fn current_tab_idx(&self) -> Option<&usize> {
        self.current_tab.as_ref()
    }

    pub fn switch_to_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.current_tab = Some(index);
        }
    }

    pub fn new_tab(&mut self, map_file: MapFile) {
        self.tabs.push(map_file);
        self.current_tab = Some(self.tabs.len() - 1);
    }

    pub fn close_tab(&mut self, index: usize) {
        self.tabs.remove(index);
        if let Some(tab) = self.current_tab {
            if tab >= self.tabs.len() {
                self.current_tab = if self.tabs.is_empty() {
                    None
                } else {
                    Some(self.tabs.len() - 1)
                };
            }
        }
    }

    pub fn close_current(&mut self) {
        if let Some(current_tab_idx) = self.current_tab_idx() {
            self.close_tab(*current_tab_idx);
        }
    }

    pub fn empty(&self) -> bool {
        self.tabs.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &MapFile> {
        self.tabs.iter()
    }
}
