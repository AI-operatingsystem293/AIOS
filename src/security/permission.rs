#[derive(Clone, Debug, PartialEq)]
pub enum Permission {
    Filesystem,
    Network,
    Memory,
    Process,
    Shell,
}
