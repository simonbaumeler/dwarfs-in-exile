use std::convert::TryInto;

use rand_chacha::{rand_core::{RngCore, SeedableRng}, ChaCha8Rng};
use seed::virtual_dom::{AsAtValue, AtValue};
use sha2::{Sha256, Digest};
use shared::{Item, Occupation, QuestType, VillageType};
use strum::Display;

#[derive(Display)]
#[strum(serialize_all = "kebab-case")]
pub enum Image {
    Dwarf(u64),
    Blueberry,
    ChainMail,
    Coal,
    Helmet,
    Nail,
    Stone,
    Wood,
    Apple,
    ApplePie,
    Backpack,
    BlackPowder,
    Cat,
    Chain,
    Dagger,
    Donkey,
    Dragon,
    FishingRod,
    Horse,
    Parrot,
    Poison,
    RawMeat,
    Boots,
    Sulfur,
    Sword,
    Wolf,
    ADwarfInDanger,
    AFishingFriend,
    CollapsedCave,
    DrunkFishing,
    FreeTheVillage,
    KillTheDragon,
    ForTheKing,
    ADwarfGotLost,
    ExploreNewLands,
    ArenaFight,
    Placeholder,
    FeastForAGuest,
    Outpost,
    Dwelling,
    Hamlet,
    Village,
    SmallTown,
    LargeTown,
    SmallCity,
    LargeCity,
    Metropolis,
    Megalopolis,
    Idling,
    Mining,
    Logging,
    Farming,
    Rockhounding,
    Fishing,
    Hunting,
    Gathering,
    Fighting,
    Exploring,
}


impl AsAtValue for Image {
    fn as_at_value(&self) -> seed::prelude::AtValue {
        match self {
            Image::Placeholder => AtValue::Some(format!("/images/placeholder.png")),
            Image::Dwarf(id) => AtValue::Some(format!("/images/dwarf-{}.jpg", id)),
            _ => AtValue::Some(format!("/images/{self}.jpg"))
        }
    }
}

impl Image {
    pub fn dwarf_from_name(name: &str) -> Image {
        let mut hasher = Sha256::new();
        hasher.update(name.as_bytes());
        let slice = &hasher.finalize()[..];
        assert_eq!(slice.len(), 32, "slice length wasn't {}", slice.len());
        let bytes: [u8; 32] = slice.try_into().unwrap();
        let mut rng = ChaCha8Rng::from_seed(bytes);
        Image::Dwarf(rng.next_u64() % 26)
    }
}

impl From<Item> for Image {
    fn from(item: Item) -> Self {
        match item {
            Item::Wood => Image::Wood,
            Item::Stone => Image::Stone,
            Item::Blueberry => Image::Blueberry,
            Item::ChainMail => Image::ChainMail,
            Item::Coal => Image::Coal,
            Item::Nail => Image::Nail,
            Item::Apple => Image::Apple,
            Item::ApplePie => Image::ApplePie,
            Item::Backpack => Image::Backpack,
            Item::BlackPowder => Image::BlackPowder,
            Item::Cat => Image::Cat,
            Item::Chain => Image::Chain,
            Item::Dagger => Image::Dagger,
            Item::Donkey => Image::Donkey,
            Item::Dragon => Image::Dragon,
            Item::FishingRod => Image::FishingRod,
            Item::Helmet => Image::Helmet,
            Item::Horse => Image::Horse,
            Item::Parrot => Image::Parrot,
            Item::Poison => Image::Poison,
            Item::RawMeat => Image::RawMeat,
            Item::Boots => Image::Boots,
            Item::Sulfur => Image::Sulfur,
            Item::Sword => Image::Sword,
            Item::Wolf => Image::Wolf,
            _ => Image::Placeholder,
        }
    }
}

impl From<QuestType> for Image {
    fn from(quest_type: QuestType) -> Self {
        match quest_type {
            QuestType::ADwarfInDanger => Image::ADwarfInDanger,
            QuestType::AFishingFriend => Image::AFishingFriend,
            QuestType::DrunkFishing => Image::DrunkFishing,
            QuestType::CollapsedCave => Image::CollapsedCave,
            QuestType::FreeTheVillage => Image::FreeTheVillage,
            QuestType::ForTheKing => Image::ForTheKing,
            QuestType::KillTheDragon => Image::KillTheDragon,
            QuestType::ExploreNewLands => Image::ExploreNewLands,
            QuestType::ADwarfGotLost => Image::ADwarfGotLost,
            QuestType::FeastForAGuest => Image::FeastForAGuest,
            QuestType::ArenaFight => Image::ArenaFight,
        }
    }
}

impl From<Occupation> for Image {
    fn from(occupation: Occupation) -> Self {
        match occupation {
            Occupation::Idling => Image::Idling,
            Occupation::Mining => Image::Mining,
            Occupation::Logging => Image::Logging,
            Occupation::Rockhounding => Image::Rockhounding,
            Occupation::Farming => Image::Farming,
            Occupation::Fishing => Image::Fishing,
            Occupation::Hunting => Image::Hunting,
            Occupation::Gathering => Image::Gathering,
            Occupation::Fighting => Image::Fighting,
            Occupation::Exploring => Image::Exploring,
        }
    }
}

impl From<VillageType> for Image {
    fn from(village_type: VillageType) -> Self {
        match village_type {
            VillageType::Outpost => Image::Outpost,
            VillageType::Dwelling => Image::Dwelling,
            VillageType::Hamlet => Image::Hamlet,
            VillageType::Village => Image::Village,
            VillageType::SmallTown => Image::SmallTown,
            VillageType::LargeTown => Image::LargeTown,
            VillageType::SmallCity => Image::SmallCity,
            VillageType::LargeCity => Image::LargeCity,
            VillageType::Metropolis => Image::Metropolis,
            VillageType::Megalopolis => Image::Megalopolis,
        }
    }
}