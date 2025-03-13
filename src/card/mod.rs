pub mod ability;
use ability::Ability;
use card_state::CardState;
use counter_type::CounterType;
use keyword_abilities::KeywordAbility;
use r#type::Type;
use sub_type::SubType;
use super_type::Supertype;
pub mod card_state;
pub mod counter_type;
pub mod keyword_abilities;
pub mod sub_type;
pub mod super_type;
pub mod r#type;

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Card {
    name: String,
    states: HashSet<CardState>,
    counters: HashMap<CounterType, u32>,
    base_power: Option<i32>,
    base_toughness: Option<i32>,
    override_power: Option<i32>,
    override_toughness: Option<i32>,
    super_types: HashSet<Supertype>,
    types: HashSet<Type>,
    sub_types: HashSet<SubType>,
    keyword_abilities: HashSet<KeywordAbility>,
    abilities: Vec<Ability>,
    equipment: Vec<String>,
    enchantments: Vec<String>,
}

#[allow(dead_code)]
impl Card {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            states: HashSet::new(),
            counters: HashMap::new(),
            base_power: None,
            base_toughness: None,
            override_power: None,
            override_toughness: None,
            super_types: HashSet::new(),
            types: HashSet::new(),
            sub_types: HashSet::new(),
            keyword_abilities: HashSet::new(),
            abilities: Vec::new(),
            equipment: Vec::new(),
            enchantments: Vec::new(),
        }
    }

    pub fn add_state(&mut self, state: CardState) {
        self.states.insert(state);
    }

    pub fn remove_state(&mut self, state: &CardState) {
        self.states.remove(state);
    }

    pub fn has_state(&self, state: &CardState) -> bool {
        self.states.contains(state)
    }

    pub fn add_counter(&mut self, counter: CounterType, amount: u32) {
        *self.counters.entry(counter).or_insert(0) += amount;
    }

    pub fn remove_counter(&mut self, counter: CounterType, amount: u32) {
        let entry = self.counters.entry(counter).or_insert(0);
        if *entry > amount {
            *entry -= amount;
        } else {
            self.counters.remove(&counter);
        }
    }

    pub fn get_counter(&self, counter: CounterType) -> u32 {
        *self.counters.get(&counter).unwrap_or(&0)
    }

    pub fn set_override_power(&mut self, power: i32) {
        self.override_power = Some(power);
    }

    pub fn set_override_toughness(&mut self, toughness: i32) {
        self.override_toughness = Some(toughness);
    }

    pub fn remove_override_power(&mut self) {
        self.override_power = None;
    }

    pub fn remove_override_toughness(&mut self) {
        self.override_toughness = None;
    }

    pub fn get_current_power(&self) -> Option<i32> {
        self.override_power.or(self.base_power)
    }

    pub fn get_current_toughness(&self) -> Option<i32> {
        self.override_toughness.or(self.base_toughness)
    }

    pub fn add_supertype(&mut self, supertype: Supertype) {
        self.super_types.insert(supertype);
    }

    pub fn remove_supertype(&mut self, supertype: &Supertype) {
        self.super_types.remove(supertype);
    }

    pub fn add_type(&mut self, card_type: Type) {
        self.types.insert(card_type);
    }

    pub fn remove_type(&mut self, card_type: &Type) {
        self.types.remove(card_type);
    }

    pub fn add_subtype(&mut self, subtype: SubType) {
        self.sub_types.insert(subtype);
    }

    pub fn remove_subtype(&mut self, subtype: &SubType) {
        self.sub_types.remove(subtype);
    }

    pub fn has_supertype(&self, supertype: &Supertype) -> bool {
        self.super_types.contains(supertype)
    }

    pub fn has_type(&self, card_type: &Type) -> bool {
        self.types.contains(card_type)
    }

    pub fn has_subtype(&self, subtype: &SubType) -> bool {
        self.sub_types.contains(subtype)
    }

    pub fn add_keyword(&mut self, keyword: KeywordAbility) {
        self.keyword_abilities.insert(keyword);
    }

    pub fn has_keyword(&self, keyword: &KeywordAbility) -> bool {
        self.keyword_abilities.contains(keyword)
    }

    pub fn add_ability(&mut self, name: &str, effect: fn(&mut Card)) {
        self.abilities.push(Ability {
            name: name.to_string(),
            effect,
        });
    }

    pub fn equip(&mut self, item: &str) {
        self.equipment.push(item.to_string());
    }

    pub fn enchant(&mut self, enchantment: &str) {
        self.enchantments.push(enchantment.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_creation() {
        let card = Card::new("Goblin Warrior");
        assert_eq!(card.name, "Goblin Warrior");
        assert!(card.states.is_empty());
        assert!(card.counters.is_empty());
        assert!(card.super_types.is_empty());
        assert!(card.types.is_empty());
        assert!(card.sub_types.is_empty());
        assert!(card.keyword_abilities.is_empty());
        assert!(card.abilities.is_empty());
        assert!(card.equipment.is_empty());
        assert!(card.enchantments.is_empty());
    }

    #[test]
    fn test_card_state() {
        let mut card = Card::new("Test Card");
        card.add_state(CardState::Tapped);
        assert!(card.has_state(&CardState::Tapped));
        card.remove_state(&CardState::Tapped);
        assert!(!card.has_state(&CardState::Tapped));
    }

    #[test]
    fn test_counters() {
        let mut card = Card::new("Counter Test");
        card.add_counter(CounterType::PlusOnePlusOne, 2);
        assert_eq!(card.get_counter(CounterType::PlusOnePlusOne), 2);
        card.remove_counter(CounterType::PlusOnePlusOne, 1);
        assert_eq!(card.get_counter(CounterType::PlusOnePlusOne), 1);
    }

    #[test]
    fn test_override_power() {
        let mut card = Card::new("Override Test");
        card.set_override_power(5);
        assert_eq!(card.get_current_power(), Some(5));
        card.remove_override_power();
        assert_eq!(card.get_current_power(), None);
    }

    #[test]
    fn test_supertype() {
        let mut card = Card::new("Supertype Test");
        card.add_supertype(Supertype::Legendary);
        assert!(card.has_supertype(&Supertype::Legendary));
    }

    #[test]
    fn test_card_types() {
        let mut card = Card::new("Type Test");
        card.add_type(Type::Creature);
        assert!(card.has_type(&Type::Creature));
    }

    #[test]
    fn test_card_subtypes() {
        let mut card = Card::new("Subtype Test");
        card.add_subtype(SubType::Attraction);
        assert!(card.has_subtype(&SubType::Attraction));
    }

    #[test]
    fn test_keywords() {
        let mut card = Card::new("Keyword Test");
        card.add_keyword(KeywordAbility::Flying);
        assert!(card.has_keyword(&KeywordAbility::Flying));
    }

    #[test]
    fn test_abilities() {
        fn test_effect(_card: &mut Card) {}
        let mut card = Card::new("Ability Test");
        card.add_ability("Test Ability", test_effect);
        assert_eq!(card.abilities.len(), 1);
    }

    #[test]
    fn test_equipment_and_enchantments() {
        let mut card = Card::new("Equipped");
        card.equip("Sword of Fire and Ice");
        card.enchant("Pacifism");
        assert_eq!(card.equipment.len(), 1);
    }
}
