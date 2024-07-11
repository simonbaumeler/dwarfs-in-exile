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
    BakedPotato,
    Lantern,
    Pickaxe,
    Potato,
    BlueberryCake,
    Wheat,
    BearClaw,
    Map,
    Egg,
    Bow,
    Crossbow,
    RhinoHorn,
    Axe,
    BearClawGloves,
    Pitchfork,
    RhinoHornHelmet,
    Spear,
    CookedMeat,
    RawFish,
    DragonEgg,
    Milk,
    Soup,
    Plough,
    PoisonedBow,
    BearClawBoots,
    Dynamite,
    Bird,
    Bread,
    TigerFang,
    Longsword,
    TigerFangDagger,
    Flour,
    CookedFish,
    PoisonedSpear,
    FishingNet,
    Overall,
    String,
    Wheel,
    FishingHat,
    Ruby,
    RingOfEndurance,
    Hemp,
    Musket,
    Fluorite,
    RingOfIntelligence,
    RingOfStrength,
    Agate,
    Sodalite,
    RingOfPerception,
    RingOfAgility,
    Selenite,
    CrystalNecklace,
    Wheelbarrow,
    GoldenRing,
    Bone,
    IronOre,
    GoldOre,
    Iron,
    Gold,
    Pufferfish,
    LeatherArmor,
    Carrot,
    Bag,
    Fabric,
    Gloves,
    Leather,
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
            Item::BakedPotato => Image::BakedPotato,
            Item::Lantern => Image::Lantern,
            Item::Pickaxe => Image::Pickaxe,
            Item::Potato => Image::Potato,
            Item::BlueberryCake => Image::BlueberryCake,
            Item::Wheat => Image::Wheat,
            Item::BearClaw => Image::BearClaw,
            Item::Map => Image::Map,
            Item::Egg => Image::Egg,
            Item::Bow => Image::Bow,
            Item::Axe => Image::Axe,
            Item::Crossbow => Image::Crossbow,
            Item::RhinoHorn => Image::RhinoHorn,
            Item::BearClawGloves => Image::BearClawGloves,
            Item::Pitchfork => Image::Pitchfork,
            Item::RhinoHornHelmet => Image::RhinoHornHelmet,
            Item::Spear => Image::Spear,
            Item::CookedMeat => Image::CookedMeat,
            Item::RawFish => Image::RawFish,
            Item::DragonsEgg => Image::DragonEgg,
            Item::Milk => Image::Milk,
            Item::Soup => Image::Soup,
            Item::Plough => Image::Plough,
            Item::PoisonedBow => Image::PoisonedBow,
            Item::BearClawBoots => Image::BearClawBoots,
            Item::Dynamite => Image::Dynamite,
            Item::Bird => Image::Bird,
            Item::Bread => Image::Bread,
            Item::TigerFang => Image::TigerFang,
            Item::Longsword => Image::Longsword,
            Item::TigerFangDagger => Image::TigerFangDagger,
            Item::Flour => Image::Flour,
            Item::CookedFish => Image::CookedFish,
            Item::PoisonedSpear => Image::PoisonedSpear,
            Item::FishingNet => Image::FishingNet,
            Item::Overall => Image::Overall,
            Item::String => Image::String,
            Item::Wheel => Image::Wheel,
            Item::FishingHat => Image::FishingHat,
            Item::Hemp => Image::Hemp,
            Item::Ruby => Image::Ruby,
            Item::RingOfEndurance => Image::RingOfEndurance,
            Item::RingOfIntelligence => Image::RingOfIntelligence,
            Item::RingOfStrength => Image::RingOfStrength,
            Item::Musket => Image::Musket,
            Item::Fluorite => Image::Fluorite,
            Item::Agate => Image::Agate,
            Item::Sodalite => Image::Sodalite,
            Item::RingOfPerception => Image::RingOfPerception,
            Item::Selenite => Image::Selenite,
            Item::RingOfAgility => Image::RingOfAgility,
            Item::CrystalNecklace => Image::CrystalNecklace,
            Item::Wheelbarrow => Image::Wheelbarrow,
            Item::GoldenRing => Image::GoldenRing,
            Item::Bone => Image::Bone,
            Item::IronOre => Image::IronOre,
            Item::GoldOre => Image::GoldOre,
            Item::Iron => Image::Iron,
            Item::Gold => Image::Gold,
            Item::PufferFish => Image::Pufferfish,
            Item::LeatherArmor => Image::LeatherArmor,
            Item::Carrot => Image::Carrot,
            Item::Leather => Image::Leather,
            Item::Fabric => Image::Fabric,
            Item::Gloves => Image::Gloves,
            Item::Bag => Image::Bag,
            Item::Headlamp => Image::HeadLamp,
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