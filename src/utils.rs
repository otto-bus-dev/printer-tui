#[derive(Default,Debug, Clone, Copy, PartialEq)]
pub enum TUIMode {
    #[default]
    View,
    Edit,
}
#[derive(Default,Debug, Clone, Copy, PartialEq)]
pub enum EditBlock {
    #[default]
    Title,
    Devices,
    Drivers
}
#[derive(Default,Debug, Clone, Copy, PartialEq)]
pub enum EditMode {
    #[default]
    View,
    Edit,
}

