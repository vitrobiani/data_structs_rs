use std::collections::HashMap;
use std::fmt;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum BallColors {
    Red,
    Blue,
    Green,
}

impl fmt::Display for BallColors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BallColors::Red => write!(f, "Red"),
            BallColors::Blue => write!(f, "Blue"),
            BallColors::Green => write!(f, "Green"),
        }
    }
}

impl BallColors {
    pub fn all() -> &'static [BallColors] {
        &[BallColors::Red, BallColors::Blue, BallColors::Green]
    }
}

#[derive(Clone, Debug)]
pub struct Urn {
    balls: HashMap<BallColors, f64>,
}

impl Urn {
    pub fn new(pairs: &[(BallColors, i32)]) -> Self {
        let mut hm = HashMap::new();
        for pair in pairs {
            hm.insert(pair.0.clone(), pair.1 as f64);
        }
        Urn { balls: hm }
    }

    pub fn tot_balls(&self) -> f64 {
        self.balls.values().sum()
    }

    pub fn get_color_amount(&self, color: BallColors) -> f64 {
        *self.balls.get(&color).unwrap_or(&0.0)
    }

    pub fn get_colors(&self) -> impl Iterator<Item = &BallColors> {
        self.balls.keys()
    }
}

pub fn calculate_prob_pull_twice_color_no_return(urns: &mut Vec<Urn>, color: BallColors) -> f64 {
    let urn_amount = urns.len() as f64;
    let urn_choice_prob = 1.0 / urn_amount;
    let urns_snapshot = urns.clone(); // Clone urns to use immutably
    let mut sec_pull_prob = 0.0;

    for u in urns.iter_mut() {
        let keys: Vec<BallColors> = u.get_colors().cloned().collect();

        for c in keys.iter() {
            let x = calculate_pulling_single_mult_urns(&urns_snapshot, c.clone());

            if let Some(ball_count) = u.balls.get_mut(c) {
                *ball_count -= 1.0;

                for ur in urns_snapshot.iter() {
                    let y = pulling_from_single_urn(ur, color.clone());
                    sec_pull_prob += urn_choice_prob * urn_choice_prob * x * y;
                }

                *ball_count += 1.0;
            }
        }
    }
    sec_pull_prob
}

pub fn calculate_pulling_single_mult_urns(urns: &Vec<Urn>, color: BallColors) -> f64 {
    let urn_amount = urns.len() as f64;
    let urn_choice_prob = 1.0 / urn_amount;
    let mut pull_prob = 0.0;
    for u in urns.iter() {
        pull_prob += pulling_from_single_urn(u, color.clone());
    }
    pull_prob *= urn_choice_prob;
    pull_prob
}

pub fn pulling_from_single_urn(urn: &Urn, color: BallColors) -> f64 {
    urn.get_color_amount(color.clone()) / urn.tot_balls()
}

pub fn entropy_calculate(probs: &Vec<f64>) -> f64 {
    let mut ent: f64 = 0.0;

    for i in probs.iter() {
        ent += i * i.log2();
    }
    ent *= -1.0;

    ent
}
