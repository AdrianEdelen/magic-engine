#[derive(Debug)]
#[allow(dead_code)]
pub struct Ability {
    pub(crate) name: String,
    pub(crate) effect: fn(&mut super::Card),
}
