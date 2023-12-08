use std::{
    cmp::Ordering,
    fs::File,
    io::{self, BufRead},
    path::Path,
    str::FromStr,
};

use anyhow::{bail, ensure, Context, Error};

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord, Hash, Clone, Copy)]
enum Card {
    CardJ,
    Card2,
    Card3,
    Card4,
    Card5,
    Card6,
    Card7,
    Card8,
    Card9,
    CardT,
    CardQ,
    CardK,
    CardA,
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::CardA),
            'K' => Ok(Self::CardK),
            'Q' => Ok(Self::CardQ),
            'J' => Ok(Self::CardJ),
            'T' => Ok(Self::CardT),
            '9' => Ok(Self::Card9),
            '8' => Ok(Self::Card8),
            '7' => Ok(Self::Card7),
            '6' => Ok(Self::Card6),
            '5' => Ok(Self::Card5),
            '4' => Ok(Self::Card4),
            '3' => Ok(Self::Card3),
            '2' => Ok(Self::Card2),
            _ => {
                bail!("invalid card")
            }
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&[Card; 5]> for Type {
    fn from(value: &[Card; 5]) -> Self {
        let mut cards: [Option<(Card, u32)>; 5] = [None; 5];
        let mut distinct_card_types = 0;

        for card in value {
            for entry in &mut cards {
                match entry {
                    Some((c, n)) if c == card => {
                        *n += 1;
                        break;
                    }
                    n @ None => {
                        distinct_card_types += 1;
                        *n = Some((*card, 1));
                        break;
                    }
                    _ => {}
                }
            }
        }

        let sort_fn = |a: &Option<(Card, u32)>, b: &Option<(Card, u32)>| match (a, b) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Greater,
            (Some(_), None) => Ordering::Less,
            (Some(a), Some(b)) => b.1.cmp(&a.1),
        };

        cards.sort_by(sort_fn);

        let mut number_of_jokers = 0;

        for entry in &mut cards {
            if let o @ Some((Card::CardJ, _)) = entry {
                number_of_jokers = o.take().unwrap().1;
            }
        }
        cards.sort_by(sort_fn);

        if number_of_jokers > 0 {
            match &mut cards[0] {
                Some((_, num)) => {
                    *num += number_of_jokers;
                    distinct_card_types -= 1;
                }
                o @ None => {
                    o.replace((Card::CardJ, number_of_jokers));
                }
            }
        }

        match distinct_card_types {
            1 => Self::FiveOfAKind,
            2 => {
                // FourOfAKind or FullHouse
                if cards[0].unwrap().1 == 4 || cards[1].unwrap().1 == 4 {
                    Self::FourOfAKind
                } else {
                    Self::FullHouse
                }
            }
            3 => {
                // ThreeOfAKind or TwoPairs
                if cards[0].unwrap().1 == 3 || cards[1].unwrap().1 == 3 || cards[2].unwrap().1 == 3
                {
                    Self::ThreeOfAKind
                } else {
                    Self::TwoPair
                }
            }
            4 => Self::OnePair,
            5 => Self::HighCard,
            _ => panic!("impossible"),
        }
    }
}

#[derive(PartialEq, Debug, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: i64,
}

impl Hand {
    fn hand_type(&self) -> Type {
        (&self.cards).into()
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();

        let card_str = split.next().context("invalid hand")?;
        ensure!(card_str.len() == 5, "invalid number of cards");

        let mut cards = [Card::Card2; 5];

        for (i, c) in card_str.chars().enumerate() {
            cards[i] = c.try_into()?;
        }

        let bid = split.next().context("invalid hand")?.parse()?;

        Ok(Hand { cards, bid })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => {
                for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                    match a.cmp(b) {
                        a if a != Ordering::Equal => return a,
                        _ => {}
                    }
                }

                Ordering::Equal
            }
            o => o,
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> Result<(), Error> {
    let mut hands = Vec::new();

    if let Ok(lines) = read_lines("input") {
        for line in lines {
            let line = line?;

            let hand: Hand = line.parse()?;
            hands.push(hand);
        }
    }
    hands.sort();

    let mut wins = 0;

    for (i, hand) in hands.iter().enumerate() {
        wins += hand.bid * (i as i64 + 1);
    }

    println!("{wins}");

    Ok(())
}
