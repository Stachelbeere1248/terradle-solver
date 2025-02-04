#[allow(dead_code)]
mod data;
mod error;
use clap::{Parser, ValueEnum};
use inquire::Select;
use data::{Weapon, Weapons};
use error::Error;
use std::cmp::Ordering;

static TERRADLE_DATA_URL: &str = "https://raw.githubusercontent.com/cxhuy/terradle-web/refs/heads/main/src/lib/data/weapons.json";
const POSSIBLE_BUCKETS: usize = 2 * 3 * 3 * 3 * 3 * 2 * 2 * 3;
type Bucket = usize;

#[derive(Parser)]
struct CliArgs {
    #[arg(long,short,value_enum, default_value = "interactive")]
    mode: Modes,
}

#[derive(ValueEnum, Copy, Clone)]
enum Modes {
    Openers,
    Interactive,
}

impl Default for Modes {
    fn default() -> Self {
        Self::Interactive
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = CliArgs::parse();
    let weapons: Vec<Weapon> = get_data().await.unwrap().weapon_data;
    match args.mode {
        Modes::Openers => openers(weapons),
        Modes::Interactive => interactive(weapons),
    };
}

fn openers(weapons: Vec<Weapon>) {
    let mut scored_weapons = weapons
        .iter()
        .map(|w| (w, score_for(w, weapons.iter())))
        .collect::<Vec<_>>();
    scored_weapons.sort_unstable_by(|(_, s1), (_, s2)| s1.partial_cmp(s2).unwrap());
    println!("{:<30}| Score\n", "Weapon".to_string());
    for (weapon, score) in scored_weapons {
        println!("{:<30}| {score}", weapon.data.name);
    }
}

fn interactive(mut weapons: Vec<Weapon>) {
    println!("Please note that Yes, No, and Correct options refer to the color of the terradle field, independent of the text inside.");
    let mut has_different_buckets = true;
    while has_different_buckets {
        let next_guess = next_guess(&weapons).clone();
        println!("Next best guess: {}", next_guess.data.name);
        let c = bool_inquire(Select::new("Was the class correct?", vec!["Yes", "No"]).prompt().unwrap());
        let d = ord_inquire(Select::new("Was the damage correct?", vec!["Correct", "Lower", "Higher"]).prompt().unwrap());
        let k = ord_inquire(Select::new("Was the knockback correct?", vec!["Correct", "Lower", "Higher"]).prompt().unwrap());
        let s = ord_inquire(Select::new("Was the speed correct?", vec!["Correct", "Slower", "Faster"]).prompt().unwrap());
        let r = ord_inquire(Select::new("Was the rarity correct?", vec!["Correct", "Lower", "Higher"]).prompt().unwrap());
        let a = bool_inquire(Select::new("Was the autoswing correct?", vec!["Yes","No"]).prompt().unwrap());
        let m = bool_inquire(Select::new("Was the usability in recipies correct?", vec!["Yes","No"]).prompt().unwrap());
        let o = ord_inquire(Select::new("Was the obtainability correct?", vec!["Yes","No", "Partially"]).prompt().unwrap());
        println!();
        let rel = Relations {c,d,k,s,r,a,m,o};
        weapons.retain(|w| Relations::from((&w.data, &next_guess.data)) == rel);
        has_different_buckets = !weapons.iter().all(|w| Relations::from((&w.data,&weapons.first().unwrap().data)) == Relations::default());
    }
    let mut s = String::from("Valid final guesses:\n");
    for w in weapons {
        s.push_str(w.data.name.as_str());
    }
    println!("{s}");
}

fn bool_inquire(s: &str) -> bool {
    matches!(s, "Yes")
}

fn ord_inquire(s: &str) -> Ordering {
    match s {
        "Higher" | "Faster" | "Partially" => Ordering::Greater,
        "Lower" | "Slower" | "No" => Ordering::Less,
        _ => Ordering::Equal,
    }
}


fn next_guess<'a>(weapons: &'a Vec<Weapon>) -> &'a Weapon {
    weapons
        .iter()
        .map(|w| (w, score_for(w, weapons.iter())))
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap()
        .0
}

fn score_for<'a, I>(w: &Weapon, weapons: I) -> f64
where
    I: Iterator<Item = &'a Weapon>,
{
    let mut buckets: [u16; POSSIBLE_BUCKETS] = [0; POSSIBLE_BUCKETS];
    weapons.filter(|&w1| !std::ptr::eq(w1, w)).for_each(|w1| {
        let bucket: Bucket = Relations::from((&w1.data, &w.data)).into();
        buckets[bucket] += 1;
    });
    let v = buckets.into_iter().filter(|&b| b != 0).collect::<Vec<_>>();
    v.iter().sum::<u16>() as f64 / v.len() as f64
}

async fn get_data() -> std::result::Result<Weapons, Error> {
    let response = reqwest::get(TERRADLE_DATA_URL).await?.error_for_status()?;
    let des = response.json::<Weapons>().await?;
    Ok(des)
}

#[derive(PartialEq, Eq)]
struct Relations {
    c: bool,
    d: Ordering,
    k: Ordering,
    s: Ordering,
    r: Ordering,
    a: bool,
    m: bool,
    o: Ordering,
}

impl Default for Relations {
    fn default() -> Self {
        Self {
            c: true,
            d: Ordering::Equal,
            k: Ordering::Equal,
            s: Ordering::Equal,
            r: Ordering::Equal,
            a: true,
            m: true,
            o: Ordering::Equal,
        }
    }
}

impl From<(&data::WeaponData, &data::WeaponData)> for Relations {
    fn from(value: (&data::WeaponData, &data::WeaponData)) -> Self {
        let (w1, w) = value;
        let o = match (
            w1.obtained.is_disjoint(&w.obtained),
            w1.obtained.eq(&w.obtained),
        ) {
            (_, true) => Ordering::Equal,        // same obtainability
            (false, false) => Ordering::Greater, // they "share an obtainability"
            (true, false) => Ordering::Less,     // different obtainability
        };
        Self {
            c: w1.damage_type == w.damage_type,
            d: w1.damage.cmp(&w.damage),
            k: w1.knockback.cmp(&w.knockback),
            s: w1.speed.cmp(&w.speed),
            r: w1.rarity.cmp(&w.rarity),
            a: w1.autoswing == w.autoswing,
            m: w1.material == w.material,
            o,
        }
    }
}

impl From<Relations> for Bucket {
    fn from(val: Relations) -> Self {
        let bools = |b: bool| if b { 1 } else { 0 };
        let ords = |o: Ordering| match o {
            Ordering::Less => 0,
            Ordering::Equal => 1,
            Ordering::Greater => 2,
        };

        let mut index = 0_usize;
        index += bools(val.c);
        index *= 3;
        index += ords(val.d);
        index *= 3;
        index += ords(val.k);
        index *= 3;
        index += ords(val.s);
        index *= 3;
        index += ords(val.r);
        index *= 2;
        index += bools(val.a);
        index *= 2;
        index += bools(val.m);
        index *= 3;
        index += ords(val.o);
        assert!(index < POSSIBLE_BUCKETS);
        index
    }
}
