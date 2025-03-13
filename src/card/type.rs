#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[allow(dead_code)]
pub enum Type {
    Land,
    Creature,
    Artifact,
    Enchantment,
    Planeswalker,
    Battle,
    Instant,
    Sorcery,
    Kindred,
    Tribal, // NOTE: Tribal is deprecated wording, replaced by Kindred - Do we need to have Tribal as a type?
    Phenomenon,
    Plane,
    Scheme,
    Vanguard,
    Conspiracy,
    Bounty,
    Summon,       // replaced by Creature - treat as creature type
    LocalEnchant, // replaced by aura subtype and enchant keyword TODO: how to handle this?
    // NOTE: The following types are treated as instants and are obsoleted with the addition of 'the stack'
    // In theory they aren't used anywhere, but depending on how we source card data (or if someone wanted to create their own card)
    // we may still find them there.
    Interrupt,
    ManaSource,
}
