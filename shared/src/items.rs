use crate::{
    Bundle, BundleType, Craftable, Food, Money, Occupation, Stats, ONE_DAY, ONE_HOUR, ONE_MINUTE,
};
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    Sequence,
    PartialOrd,
    Ord,
    Display,
)]
#[strum(serialize_all = "title_case")]
pub enum Item {
    Wood,
    Coal,
    Stone,
    IronOre,
    Iron,
    Nail,
    Chain,
    ChainMail,
    Bow,
    RawMeat,
    CookedMeat,
    Leather,
    Bone,
    Blueberry,
    RawFish,
    CookedFish,
    PufferFish,
    Poison,
    PoisonedBow,
    Parrot,
    String,
    Hemp,
    Wolf,
    LeatherArmor,
    Sword,
    Longsword,
    Spear,
    PoisonedSpear,
    Cat,
    Apple,
    DragonsEgg,
    Dragon,
    Donkey,
    Milk,
    Wheat,
    Egg,
    Bread,
    Flour,
    BlueberryCake,
    Potato,
    BakedPotato,
    Soup,
    Carrot,
    Crossbow,
    Pickaxe,
    Axe,
    Pitchfork,
    ApplePie,
    Bird,
    Sulfur,
    BlackPowder,
    Musket,
    Dynamite,
    Fabric,
    Backpack,
    Helmet,
    Horse,
    Map,
    FishingHat,
    FishingRod,
    Overall,
    Boots,
    Wheel,
    Wheelbarrow,
    Plough,
    Lantern,
    GoldOre,
    Gold,
    GoldenRing,
    Fluorite,           // Intelligence
    Agate,              // Strength
    Sodalite,           // Perception
    Ruby,               // Endurance
    Selenite,           // Agility
    RingOfIntelligence, // Intelligence
    RingOfStrength,     // Strength
    RingOfPerception,   // Perception
    RingOfEndurance,    // Endurance
    RingOfAgility,      // Agility
    CrystalNecklace,
    TigerFang,
    Dagger,
    TigerFangDagger,
    RhinoHorn,
    RhinoHornHelmet,
    BearClaw,
    Gloves,
    BearClawGloves,
    BearClawBoots,
    FishingNet,
    Bag,
    Headlamp,
    // Diamond
    // Diamond Axe
    // Diamond Pickaxe
    // Diamand Sword
    // Enchanted Bow + Sodalite -> +Perception
    // Enchanted Longsword + Agate -> +Strength
    // Enchanted Helmet + Fluorite -> +Intelligence
    // Enchanted Boots + Selenite -> +Agility
    // Enchanted Gloves + Ruby -> +Endurance
    // Magic Lantern
    Diamond,
    DiamondAxe,
    DiamondPickaxe,
    DiamondSword,
    RhinoHornPants,
    DynamiteCrossbow,
}

impl Craftable for Item {
    fn requires(self) -> Option<Bundle<Item>> {
        match self {
            Item::Iron => Some(Bundle::new().add(Item::IronOre, 1).add(Item::Coal, 1)),
            Item::Nail => Some(Bundle::new().add(Item::Iron, 1).add(Item::Coal, 1)),
            Item::Chain => Some(Bundle::new().add(Item::Iron, 5).add(Item::Coal, 2)),
            Item::ChainMail => Some(Bundle::new().add(Item::Chain, 5)),
            Item::Coal => Some(Bundle::new().add(Item::Wood, 3)),
            Item::Bow => Some(Bundle::new().add(Item::Wood, 3).add(Item::String, 1)),
            Item::CookedMeat => Some(Bundle::new().add(Item::RawMeat, 1).add(Item::Coal, 1)),
            Item::CookedFish => Some(Bundle::new().add(Item::RawFish, 1).add(Item::Coal, 1)),
            Item::Poison => Some(Bundle::new().add(Item::PufferFish, 1)),
            Item::PoisonedBow => Some(Bundle::new().add(Item::Bow, 1).add(Item::Poison, 1)),
            Item::String => Some(Bundle::new().add(Item::Hemp, 3)),
            Item::LeatherArmor => Some(Bundle::new().add(Item::Leather, 8).add(Item::String, 3)),
            Item::Sword => Some(Bundle::new().add(Item::Wood, 1).add(Item::Iron, 5)),
            Item::Longsword => Some(Bundle::new().add(Item::Wood, 1).add(Item::Iron, 10)),
            Item::Spear => Some(Bundle::new().add(Item::Wood, 3).add(Item::Iron, 2)),
            Item::Dagger => Some(Bundle::new().add(Item::Iron, 3)),
            Item::TigerFangDagger => {
                Some(Bundle::new().add(Item::TigerFang, 1).add(Item::Dagger, 1))
            }
            Item::PoisonedSpear => Some(Bundle::new().add(Item::Spear, 1).add(Item::Poison, 1)),
            Item::Dragon => Some(Bundle::new().add(Item::DragonsEgg, 1).add(Item::Coal, 100)),
            Item::BakedPotato => Some(Bundle::new().add(Item::Potato, 1).add(Item::Coal, 1)),
            Item::BlueberryCake => Some(
                Bundle::new()
                    .add(Item::Blueberry, 5)
                    .add(Item::Flour, 3)
                    .add(Item::Egg, 2)
                    .add(Item::Milk, 1),
            ),
            Item::ApplePie => Some(
                Bundle::new()
                    .add(Item::Apple, 5)
                    .add(Item::Flour, 3)
                    .add(Item::Egg, 2)
                    .add(Item::Milk, 1),
            ),
            Item::Bread => Some(Bundle::new().add(Item::Flour, 3)),
            Item::Flour => Some(Bundle::new().add(Item::Wheat, 3)),
            Item::Soup => Some(Bundle::new().add(Item::Potato, 3).add(Item::Carrot, 3)),
            Item::Pickaxe => Some(Bundle::new().add(Item::Wood, 5).add(Item::Iron, 10)),
            Item::Axe => Some(Bundle::new().add(Item::Wood, 5).add(Item::Iron, 10)),
            Item::Pitchfork => Some(Bundle::new().add(Item::Wood, 5).add(Item::Iron, 10)),
            Item::Crossbow => Some(
                Bundle::new()
                    .add(Item::Wood, 5)
                    .add(Item::Iron, 10)
                    .add(Item::Nail, 3),
            ),
            Item::BlackPowder => Some(Bundle::new().add(Item::Coal, 2).add(Item::Sulfur, 1)),
            Item::Musket => Some(
                Bundle::new()
                    .add(Item::Wood, 10)
                    .add(Item::Iron, 20)
                    .add(Item::BlackPowder, 5),
            ),
            Item::Dynamite => Some(
                Bundle::new()
                    .add(Item::BlackPowder, 10)
                    .add(Item::Fabric, 1),
            ),
            Item::DynamiteCrossbow => {
                Some(Bundle::new().add(Item::Dynamite, 1).add(Item::Crossbow, 1))
            }
            Item::Fabric => Some(Bundle::new().add(Item::String, 3)),
            Item::Backpack => Some(Bundle::new().add(Item::String, 2).add(Item::Leather, 5)),
            Item::Bag => Some(Bundle::new().add(Item::String, 1).add(Item::Fabric, 2)),
            Item::Helmet => Some(
                Bundle::new()
                    .add(Item::Iron, 3)
                    .add(Item::Leather, 1)
                    .add(Item::String, 1),
            ),
            Item::RhinoHornHelmet => {
                Some(Bundle::new().add(Item::RhinoHorn, 1).add(Item::Helmet, 1))
            }
            Item::FishingRod => Some(
                Bundle::new()
                    .add(Item::Wood, 3)
                    .add(Item::String, 3)
                    .add(Item::Iron, 1),
            ),
            Item::FishingHat => Some(Bundle::new().add(Item::Fabric, 5)),
            Item::Map => Some(Bundle::new().add(Item::Fabric, 5)),
            Item::Overall => Some(Bundle::new().add(Item::Fabric, 5).add(Item::String, 5)),
            Item::Boots => Some(Bundle::new().add(Item::Leather, 5).add(Item::String, 2)),
            Item::BearClawBoots => Some(Bundle::new().add(Item::BearClaw, 1).add(Item::Boots, 1)),
            Item::Gloves => Some(Bundle::new().add(Item::Leather, 5).add(Item::String, 2)),
            Item::BearClawGloves => Some(Bundle::new().add(Item::BearClaw, 1).add(Item::Gloves, 1)),
            Item::Wheel => Some(
                Bundle::new()
                    .add(Item::Iron, 3)
                    .add(Item::Wood, 5)
                    .add(Item::Nail, 5),
            ),
            Item::Wheelbarrow => Some(
                Bundle::new()
                    .add(Item::Wheel, 1)
                    .add(Item::Iron, 2)
                    .add(Item::Nail, 5),
            ),
            Item::Plough => Some(
                Bundle::new()
                    .add(Item::Wheel, 2)
                    .add(Item::Iron, 10)
                    .add(Item::Nail, 5)
                    .add(Item::Chain, 5),
            ),
            Item::Lantern => Some(Bundle::new().add(Item::Iron, 3).add(Item::String, 1)),
            Item::Gold => Some(Bundle::new().add(Item::GoldOre, 1).add(Item::Coal, 1)),
            Item::GoldenRing => Some(Bundle::new().add(Item::Gold, 3)),
            Item::RingOfIntelligence => Some(
                Bundle::new()
                    .add(Item::GoldenRing, 1)
                    .add(Item::Fluorite, 1),
            ),
            Item::RingOfStrength => {
                Some(Bundle::new().add(Item::GoldenRing, 1).add(Item::Agate, 1))
            }
            Item::RingOfPerception => Some(
                Bundle::new()
                    .add(Item::GoldenRing, 1)
                    .add(Item::Sodalite, 1),
            ),
            Item::RingOfEndurance => {
                Some(Bundle::new().add(Item::GoldenRing, 1).add(Item::Ruby, 1))
            }
            Item::RingOfAgility => Some(
                Bundle::new()
                    .add(Item::GoldenRing, 1)
                    .add(Item::Selenite, 1),
            ),
            Item::CrystalNecklace => Some(
                Bundle::new()
                    .add(Item::String, 1)
                    .add(Item::Fluorite, 1)
                    .add(Item::Agate, 1)
                    .add(Item::Sodalite, 1)
                    .add(Item::Ruby, 1)
                    .add(Item::Selenite, 1),
            ),
            Item::FishingNet => Some(Bundle::new().add(Item::String, 20).add(Item::Iron, 2)),
            Item::Headlamp => Some(Bundle::new().add(Item::Helmet, 1).add(Item::Lantern, 1)),
            Item::DiamondAxe => Some(Bundle::new().add(Item::Axe, 1).add(Item::Diamond, 3)),
            Item::DiamondPickaxe => Some(Bundle::new().add(Item::Pickaxe, 1).add(Item::Diamond, 3)),
            Item::DiamondSword => Some(Bundle::new().add(Item::Sword, 1).add(Item::Diamond, 3)),
            Item::RhinoHornPants => Some(
                Bundle::new()
                    .add(Item::RhinoHorn, 1)
                    .add(Item::LeatherArmor, 1),
            ),
            _ => None,
        }
    }
}

impl Into<usize> for Item {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Hash, PartialEq, Eq, Sequence, PartialOrd, Ord,
)]
pub enum ItemType {
    Tool,
    Clothing,
    Pet,
    Food,
}

impl ItemType {
    pub fn equippable(&self) -> bool {
        matches!(self, Self::Tool | Self::Clothing | Self::Pet)
    }
}

impl std::fmt::Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemType::Tool => write!(f, "Tool"),
            ItemType::Clothing => write!(f, "Clothing"),
            ItemType::Pet => write!(f, "Pet"),
            ItemType::Food => write!(f, "Food"),
        }
    }
}

impl Item {
    pub fn item_type(self) -> Option<ItemType> {
        match self {
            Item::ChainMail
            | Item::LeatherArmor
            | Item::Backpack
            | Item::Helmet
            | Item::FishingHat
            | Item::Overall
            | Item::Boots
            | Item::RingOfIntelligence
            | Item::RingOfStrength
            | Item::RingOfPerception
            | Item::RingOfEndurance
            | Item::RingOfAgility
            | Item::RhinoHornHelmet
            | Item::Gloves
            | Item::BearClawGloves
            | Item::Headlamp
            | Item::BearClawBoots
            | Item::GoldenRing
            | Item::RhinoHornPants
            | Item::CrystalNecklace => Some(ItemType::Clothing),

            Item::Bow
            | Item::PoisonedBow
            | Item::Sword
            | Item::Longsword
            | Item::Spear
            | Item::PoisonedSpear
            | Item::Crossbow
            | Item::Pickaxe
            | Item::Axe
            | Item::Pitchfork
            | Item::Musket
            | Item::Dynamite
            | Item::FishingRod
            | Item::Map
            | Item::Wheelbarrow
            | Item::Plough
            | Item::Lantern
            | Item::FishingNet
            | Item::Dagger
            | Item::TigerFangDagger
            | Item::Bag
            | Item::DiamondAxe
            | Item::DiamondPickaxe
            | Item::DiamondSword
            | Item::DynamiteCrossbow => Some(ItemType::Tool),

            Item::Parrot
            | Item::Wolf
            | Item::Cat
            | Item::Dragon
            | Item::Donkey
            | Item::Bird
            | Item::Horse => Some(ItemType::Pet),

            Item::Apple
            | Item::Blueberry
            | Item::Bread
            | Item::BlueberryCake
            | Item::CookedFish
            | Item::CookedMeat
            | Item::BakedPotato
            | Item::Soup
            | Item::ApplePie => Some(ItemType::Food),
            _ => None,
        }
    }

    pub fn provides_stats(self) -> Stats {
        match self {
            Item::ChainMail => Stats {
                agility: -2,
                ..Default::default()
            },
            Item::LeatherArmor => Stats {
                ..Default::default()
            },
            Item::Backpack => Stats {
                agility: -2,
                ..Default::default()
            },
            Item::Musket => Stats {
                agility: -2,
                ..Default::default()
            },
            Item::Parrot => Stats {
                perception: 4,
                intelligence: 4,
                ..Default::default()
            },
            Item::Bird => Stats {
                perception: 4,
                ..Default::default()
            },
            Item::Horse => Stats {
                strength: 4,
                agility: 4,
                endurance: 4,
                ..Default::default()
            },
            Item::Cat => Stats {
                agility: 6,
                perception: 6,
                ..Default::default()
            },
            Item::Boots => Stats {
                endurance: 4,
                ..Default::default()
            },
            Item::Gloves => Stats {
                agility: 4,
                ..Default::default()
            },
            Item::BearClawBoots => Stats {
                endurance: 4,
                strength: 4,
                ..Default::default()
            },
            Item::BearClawGloves => Stats {
                agility: 4,
                strength: 4,
                ..Default::default()
            },
            Item::TigerFangDagger => Stats {
                agility: 4,
                perception: 4,
                ..Default::default()
            },
            Item::Map => Stats {
                intelligence: 2,
                ..Default::default()
            },
            Item::Lantern => Stats {
                perception: 4,
                ..Default::default()
            },
            Item::Headlamp => Stats {
                perception: 6,
                ..Default::default()
            },
            Item::GoldenRing => Stats {
                strength: 1,
                endurance: 1,
                agility: 1,
                intelligence: 1,
                perception: 1,
                ..Default::default()
            },
            Item::RingOfIntelligence => Stats {
                intelligence: 8,
                ..Default::default()
            },
            Item::RingOfStrength => Stats {
                strength: 8,
                ..Default::default()
            },
            Item::RingOfPerception => Stats {
                perception: 8,
                ..Default::default()
            },
            Item::RingOfEndurance => Stats {
                endurance: 8,
                ..Default::default()
            },
            Item::RingOfAgility => Stats {
                agility: 8,
                ..Default::default()
            },
            Item::CrystalNecklace => Stats {
                strength: 6,
                endurance: 6,
                agility: 6,
                intelligence: 6,
                perception: 6,
            },
            Item::RhinoHornPants => Stats {
                strength: 8,
                endurance: 8,
                intelligence: -4,
                ..Default::default()
            },
            _ => Stats::default(),
        }
    }

    pub fn nutritional_value(self) -> Option<Food> {
        if self.item_type() == Some(ItemType::Food) {
            let nutrition = self.item_rarity_num() / 100 * (self.crafting_depth() + 1);
            Some(nutrition.max(1))
        } else {
            None
        }
    }

    pub fn money_value(self) -> Money {
        self.item_rarity_num() / 2000
    }

    // sefulness from 0 - 10
    pub fn usefulness_for(self, occupation: Occupation) -> u64 {
        match (self, occupation) {
            (Item::Crossbow, Occupation::Hunting | Occupation::Fighting) => 7,
            (Item::DynamiteCrossbow, Occupation::Hunting | Occupation::Fighting) => 10,
            (Item::Bow, Occupation::Hunting | Occupation::Fighting) => 5,
            (Item::PoisonedBow, Occupation::Hunting | Occupation::Fighting) => 8,
            (Item::Spear, Occupation::Hunting | Occupation::Fighting) => 4,
            (Item::PoisonedSpear, Occupation::Hunting | Occupation::Fighting) => 7,
            (Item::Sword, Occupation::Fighting) => 6,
            (Item::DiamondSword, Occupation::Fighting) => 10,
            (Item::Longsword, Occupation::Fighting) => 7,
            (Item::Dagger, Occupation::Fighting) => 5,
            (Item::TigerFangDagger, Occupation::Fighting) => 8,
            (Item::Dragon, Occupation::Hunting) => 4,
            (Item::Dragon, Occupation::Fighting) => 10,
            (Item::Donkey, Occupation::Gathering) => 6,
            (Item::Donkey, Occupation::Farming) => 6,
            (Item::Donkey, Occupation::Exploring) => 6,
            (Item::Wolf, Occupation::Hunting) => 10,
            (Item::Wolf, Occupation::Fighting) => 4,
            (Item::Axe, Occupation::Logging) => 6,
            (Item::Axe, Occupation::Fighting) => 3,
            (Item::DiamondAxe, Occupation::Logging) => 10,
            (Item::DiamondAxe, Occupation::Fighting) => 3,
            (Item::Pickaxe, Occupation::Mining | Occupation::Rockhounding) => 6,
            (Item::DiamondPickaxe, Occupation::Mining | Occupation::Rockhounding) => 10,
            (Item::Pitchfork, Occupation::Farming) => 6,
            (Item::ChainMail, Occupation::Fighting) => 8,
            (Item::LeatherArmor, Occupation::Fighting) => 4,
            (Item::RhinoHornPants, Occupation::Fighting) => 6,
            (Item::Bird, Occupation::Mining | Occupation::Rockhounding) => 3,
            (Item::Musket, Occupation::Hunting) => 10,
            (Item::Musket, Occupation::Fighting) => 6,
            (Item::Dynamite, Occupation::Fighting) => 5,
            (Item::Dynamite, Occupation::Mining) => 10,
            (Item::Backpack, Occupation::Gathering) => 7,
            (Item::Bag, Occupation::Gathering) => 5,
            (
                Item::Helmet | Item::RhinoHornHelmet,
                Occupation::Mining | Occupation::Logging | Occupation::Rockhounding,
            ) => 4,
            (Item::Helmet, Occupation::Fighting) => 6,
            (Item::Headlamp, Occupation::Mining | Occupation::Rockhounding) => 8,
            (Item::RhinoHornHelmet, Occupation::Fighting) => 8,
            (Item::Horse, Occupation::Fighting | Occupation::Exploring) => 4,
            (Item::Horse, Occupation::Farming | Occupation::Logging) => 7,
            (Item::Map, Occupation::Exploring) => 8,
            (Item::Map, Occupation::Gathering) => 6,
            (Item::FishingHat, Occupation::Fishing) => 6,
            (Item::FishingRod, Occupation::Fishing) => 6,
            (Item::FishingNet, Occupation::Fishing) => 10,
            (Item::Overall, Occupation::Farming | Occupation::Logging) => 8,
            (
                Item::Boots | Item::BearClawBoots,
                Occupation::Hunting | Occupation::Gathering | Occupation::Exploring,
            ) => 4,
            (
                Item::Gloves | Item::BearClawGloves,
                Occupation::Mining | Occupation::Logging | Occupation::Rockhounding,
            ) => 4,
            (Item::BearClawBoots | Item::BearClawGloves, Occupation::Fighting) => 6,
            (Item::Wheelbarrow, Occupation::Gathering) => 8,
            (Item::Plough, Occupation::Farming) => 10,
            (Item::Lantern, Occupation::Mining | Occupation::Rockhounding) => 4,
            _ => 0,
        }
    }

    pub fn item_probability(self, occupation: Occupation) -> Option<ItemProbability> {
        match occupation {
            Occupation::Mining => match self {
                Item::Stone => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_MINUTE / 2,
                }),
                Item::IronOre => Some(ItemProbability {
                    starting_from_tick: ONE_MINUTE * 3,
                    expected_ticks_per_drop: ONE_MINUTE * 3,
                }),
                Item::Coal => Some(ItemProbability {
                    starting_from_tick: ONE_MINUTE * 2,
                    expected_ticks_per_drop: ONE_MINUTE * 2,
                }),
                Item::Sulfur => Some(ItemProbability {
                    starting_from_tick: ONE_HOUR,
                    expected_ticks_per_drop: ONE_HOUR,
                }),
                Item::GoldOre => Some(ItemProbability {
                    starting_from_tick: ONE_HOUR * 2,
                    expected_ticks_per_drop: ONE_HOUR * 2,
                }),
                _ => None,
            },
            Occupation::Rockhounding => match self {
                Item::Fluorite => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_DAY,
                }),
                Item::Agate => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_DAY,
                }),
                Item::Sodalite => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_DAY,
                }),
                Item::Ruby => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_DAY,
                }),
                Item::Selenite => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_DAY,
                }),
                Item::Diamond => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_DAY,
                }),
                _ => None,
            },
            Occupation::Logging => match self {
                Item::Wood => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_MINUTE / 2,
                }),
                Item::Apple => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_MINUTE * 5,
                }),
                Item::Parrot => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_DAY,
                }),
                Item::Bird => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_DAY,
                }),
                _ => None,
            },
            Occupation::Hunting => match self {
                Item::RawMeat => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_MINUTE * 5,
                }),
                Item::Leather => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_MINUTE * 5,
                }),
                Item::Bone => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_MINUTE * 10,
                }),
                _ => None,
            },
            Occupation::Gathering => match self {
                Item::Blueberry => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_MINUTE,
                }),
                Item::Apple => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_MINUTE * 2,
                }),
                Item::Hemp => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_MINUTE * 3,
                }),
                _ => None,
            },
            Occupation::Fishing => match self {
                Item::RawFish => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_MINUTE * 5,
                }),
                Item::PufferFish => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_HOUR,
                }),
                Item::Boots => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_HOUR * 2,
                }),
                Item::Gloves => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_HOUR * 2,
                }),
                Item::GoldenRing => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_HOUR * 6,
                }),
                _ => None,
            },
            Occupation::Fighting => match self {
                Item::Wolf => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_DAY,
                }),
                Item::TigerFang => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_DAY,
                }),
                Item::BearClaw => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_DAY,
                }),
                Item::RhinoHorn => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_DAY,
                }),
                _ => None,
            },
            Occupation::Exploring => match self {
                Item::Cat => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_DAY * 7,
                }),
                Item::Parrot => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_DAY,
                }),
                Item::Bird => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_DAY,
                }),
                Item::Donkey => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_DAY,
                }),
                Item::Horse => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_DAY,
                }),
                _ => None,
            },
            Occupation::Farming => match self {
                Item::Milk => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_MINUTE * 5,
                }),
                Item::Egg => Some(ItemProbability {
                    starting_from_tick: 0,
                    expected_ticks_per_drop: ONE_MINUTE * 5,
                }),
                Item::Wheat => Some(ItemProbability {
                    starting_from_tick: ONE_HOUR,
                    expected_ticks_per_drop: ONE_MINUTE * 10,
                }),
                Item::Potato => Some(ItemProbability {
                    starting_from_tick: ONE_HOUR * 3,
                    expected_ticks_per_drop: ONE_MINUTE * 10,
                }),
                Item::Carrot => Some(ItemProbability {
                    starting_from_tick: ONE_HOUR * 3,
                    expected_ticks_per_drop: ONE_MINUTE * 10,
                }),
                _ => None,
            },
            Occupation::Idling => None,
        }
    }

    pub fn item_rarity_num(self) -> u64 {
        let mut rarity = None;

        let mut update_rarity = |new_rarity| {
            if let Some(rarity) = &mut rarity {
                if new_rarity < *rarity {
                    *rarity = new_rarity;
                }
            } else {
                rarity = Some(new_rarity);
            }
        };

        for occupation in enum_iterator::all::<Occupation>() {
            if let Some(item_probability) = self.item_probability(occupation) {
                update_rarity(item_probability.expected_ticks_per_drop);
            }
        }

        if let Some(requires) = self.requires() {
            update_rarity(
                requires
                    .iter()
                    .map(|(item, n)| item.item_rarity_num() * *n)
                    .sum(),
            )
        }

        rarity.unwrap_or(25000)
    }

    pub fn crafting_depth(self) -> u64 {
        let mut depth = 0;

        let mut update_depth = |new_depth| {
            depth = depth.max(new_depth);
        };

        if let Some(requires) = self.requires() {
            if let Some(max_depth) = requires.iter().map(|(item, _)| item.crafting_depth()).max() {
                update_depth(max_depth + 1)
            }
        }

        depth
    }

    pub fn item_rarity(self) -> ItemRarity {
        ItemRarity::from(self.item_rarity_num())
    }
}

#[derive(Debug, PartialEq, Eq, Display, PartialOrd, Ord)]
#[strum(serialize_all = "title_case")]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl From<u64> for ItemRarity {
    fn from(value: u64) -> Self {
        if value < 200 {
            ItemRarity::Common
        } else if value < 1000 {
            ItemRarity::Uncommon
        } else if value < 5000 {
            ItemRarity::Rare
        } else if value < 25000 {
            ItemRarity::Epic
        } else {
            ItemRarity::Legendary
        }
    }
}

pub struct ItemProbability {
    pub starting_from_tick: u64,
    pub expected_ticks_per_drop: u64,
}

impl BundleType for Item {}