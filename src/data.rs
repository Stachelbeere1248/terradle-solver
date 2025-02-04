use serde::Deserialize;
use serde::Deserializer;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum Class {
    Melee,
    Ranged,
    Magic,
    Summon,
    #[serde(rename = "")]
    Classless,
}

impl PartialOrd for Class {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Speed {
    #[serde(rename = "Snail")]
    Snail,
    #[serde(rename = "Extremely slow")]
    ExtremelySlow,
    #[serde(rename = "Very slow")]
    VerySlow,
    #[serde(rename = "Slow")]
    Slow,
    #[serde(rename = "Average")]
    Average,
    #[serde(rename = "Fast")]
    Fast,
    #[serde(rename = "Very fast")]
    VeryFast,
    #[serde(rename = "Insanely fast")]
    InsanelyFast,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Knockback {
    #[serde(rename = "No knockback")]
    None,
    #[serde(rename = "Extremely weak")]
    ExtremelyWeak,
    #[serde(rename = "Very weak")]
    VeryWeak,
    #[serde(rename = "Weak")]
    Weak,
    #[serde(rename = "Average")]
    Average,
    #[serde(rename = "Strong")]
    Strong,
    #[serde(rename = "Very strong")]
    VeryStrong,
    #[serde(rename = "Extremely strong")]
    ExtremelyStrong,
    #[serde(rename = "Insane")]
    Insane,
}

#[derive(Debug, Clone, Deserialize, Hash, PartialEq, Eq)]
pub enum ObtainingMethod {
    Drop,
    Crafting,
    Chest,
    Fishing,
    Buy,
}

#[derive(Debug, Clone)]
pub struct SellValue {
    platinum: u16,
    gold: u8,
    silver: u8,
    copper: u8,
}

impl From<SellValue> for u64 {
    fn from(val: SellValue) -> Self {
        val.copper as u64
            + val.silver as u64 * 100
            + val.gold as u64 * 100 * 100
            + val.platinum as u64 * 100 * 100 * 100
    }
}

impl<'de> Deserialize<'de> for SellValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value_str: String = Deserialize::deserialize(deserializer)?;

        let mut platinum = 0;
        let mut gold = 0;
        let mut silver = 0;
        let mut copper = 0;

        let mut units = [
            ("Platinum", &mut platinum),
            ("Gold", &mut gold),
            ("Silver", &mut silver),
            ("Copper", &mut copper),
        ];
        let parts: Vec<&str> = value_str.split_whitespace().collect();

        let mut i = 0;
        while i < parts.len() {
            if let Ok(amount) = parts[i].parse::<u16>() {
                if i + 1 < parts.len() {
                    let currency = parts[i + 1];
                    if let Some(unit) = units.iter_mut().find(|(name, _)| *name == currency) {
                        *(unit.1) = amount;
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
            } else {
                i += 1;
            }
        }

        Ok(SellValue {
            platinum,
            gold: gold as u8,
            silver: silver as u8,
            copper: copper as u8,
        })
    }
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaponData {
    #[serde(deserialize_with = "deserialize_u16_from_string")]
    pub id: u16,
    pub name: String,
    #[serde(deserialize_with = "deserialize_u16_from_string")]
    pub damage: u16,
    pub damage_type: Class,
    pub knockback: Knockback,
    pub speed: Speed,
    #[serde(deserialize_with = "deserialize_u16_from_string")]
    pub rarity: u16,
    pub sell: SellValue,
    pub obtained: std::collections::HashSet<ObtainingMethod>,
    pub material: bool,
    pub autoswing: bool,
}

fn deserialize_u16_from_string<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    struct U16FromString;

    impl serde::de::Visitor<'_> for U16FromString {
        type Value = u16;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "a u16 or a string representing a u16")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            value
                .parse::<u16>()
                .map_err(|_| E::custom(format!("Invalid u16 value: {}", value)))
        }

        fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E> {
            Ok(value)
        }
    }

    deserializer.deserialize_any(U16FromString)
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weapon {
    pub data: WeaponData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weapons {
    pub weapon_data: Vec<Weapon>,
}
