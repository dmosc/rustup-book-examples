#[derive(Debug, Clone, Copy)]
enum ShirtColor {
    Blue,
    Red,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_blue = 0;
        let mut num_red = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }
        if num_blue > num_red {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Red],
    };
    let user_pref = Some(ShirtColor::Red);
    let giveaway = store.giveaway(user_pref);
    println!("User with preference {:?} gets {:?}", user_pref, giveaway);

    let user_pref = None;
    let giveaway = store.giveaway(user_pref);
    println!("User with preference {:?} gets {:?}", user_pref, giveaway);
}
