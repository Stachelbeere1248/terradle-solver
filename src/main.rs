mod error;
#[allow(dead_code)]
mod data;
use error::Error;
use data::Weapons;
use std::cmp::Ordering;

static TERRADLE_DATA_URL: &str = "https://raw.githubusercontent.com/cxhuy/terradle-web/refs/heads/main/src/lib/data/weapons.json";
const POSSIBLE_BUCKETS: usize = 2 * 3 * 3 * 3 * 3 * 2 * 2 * 3;
type Bucket = usize;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let weapons = get_data().await.unwrap().weapon_data;
    let mut weapon_scores = weapons.iter().map(|w| {
        let mut buckets: [u16;POSSIBLE_BUCKETS] = [0;POSSIBLE_BUCKETS];
        weapons.iter().filter(|&w1| !std::ptr::eq(w1,w)).for_each(|w1| {
            let bucket: Bucket = Relations::from((&w1.data,&w.data)).into();
            buckets[bucket] += 1;
        });
        let v = buckets.into_iter().filter(|&b| b != 0);
        (w, v.clone().sum::<u16>() as f64 / v.count() as f64)
    })
    .collect::<Vec<_>>();
    weapon_scores.sort_unstable_by(|(_, s1),(_,s2)| s1.partial_cmp(s2).unwrap());
    println!("{:<30}| Score\n", "Weapon".to_string());
    for (weapon, score) in weapon_scores {
        println!("{:<30}| {score}", weapon.data.name);
    }
}

async fn get_data() -> std::result::Result<Weapons, Error> {
    let response = reqwest::get(TERRADLE_DATA_URL).await?.error_for_status()?;
    let des = response.json::<Weapons>().await?;
    Ok(des)
}

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

impl From<(&data::WeaponData, &data::WeaponData)> for Relations {
   fn from(value: (&data::WeaponData, &data::WeaponData)) -> Self {
       let (w1, w) = value;
       let o = match (w1.obtained.is_disjoint(&w.obtained), w1.obtained.eq(&w.obtained)) {
               (_, true) => Ordering::Equal, // same obtainability
               (false, false) => Ordering::Greater, // they "share an obtainability"
               (true, false) => Ordering::Less, // different obtainability
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
        let bools = |b: bool| if b {1} else {0};
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
