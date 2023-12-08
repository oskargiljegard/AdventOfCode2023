use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Rank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    rank: Rank,
    bid: i32,
}

fn num_cards_to_rank(num_cards: &[usize]) -> Rank {
    match num_cards {
        [5] => Rank::FiveOfAKind,
        [4, 1] => Rank::FourOfAKind,
        [3, 2] => Rank::FullHouse,
        [3, 1, 1] => Rank::ThreeOfAKind,
        [2, 2, 1] => Rank::TwoPair,
        [2, 1, 1, 1] => Rank::OnePair,
        [1, 1, 1, 1, 1] => Rank::HighCard,
        _ => panic!("No match exists")
    }
}

fn calc_rank_joker(cards: &Vec<char>) -> Rank {
    let mut occurrences: HashMap<&char, usize> = cards.iter().counts();
    let num_jokers = *occurrences.get(&'J').unwrap_or(&0);
    occurrences.remove(&'J');
    let mut num_cards = occurrences.into_values().collect::<Vec<_>>();
    num_cards.sort();
    num_cards.reverse();
    if num_cards.is_empty() {
        num_cards.push(num_jokers);
    } else {
        num_cards[0] += num_jokers;
    }
    //*num_cards[0] += *num_jokers;
    num_cards_to_rank(num_cards.as_slice())
}

fn calc_rank_no_joker(cards: &Vec<char>) -> Rank {
    let occurrences: HashMap<_, usize> = cards.iter().counts();
    let mut num_cards = occurrences.into_values().collect::<Vec<_>>();
    num_cards.sort();
    num_cards.reverse();
    num_cards_to_rank(num_cards.as_slice())
}

fn parse_hand<F>(line: &str, calc_rank: F) -> Hand 
                where F: Fn(&Vec<char>) -> Rank {
    let (cards_str, bid_str) = line.split_once(" ").unwrap();
    let cards: Vec<char> = cards_str.chars().collect();
    let bid = bid_str.parse::<i32>().unwrap();
    let rank = calc_rank(&cards);
    Hand { cards, bid, rank }
}

fn compare_hands(h1: &Hand, h2: &Hand, values: &str) -> std::cmp::Ordering {
    let rank_ordering = h1.rank.cmp(&h2.rank);
    if rank_ordering == std::cmp::Ordering::Equal {
        for i in 0..h1.cards.len() {
            let v1 = values.find(h1.cards[i]).unwrap();
            let v2 = values.find(h2.cards[i]).unwrap();
            let card_ordering = v1.cmp(&v2);
            if card_ordering != std::cmp::Ordering::Equal {
                return card_ordering;
            }
        }
        return std::cmp::Ordering::Equal;
    } else {
        return rank_ordering
    }
}

fn part1() {
    let input = include_str!("input1.txt");
    let mut hands: Vec<Hand> = input.lines().map(|l| parse_hand(l, calc_rank_no_joker)).collect();
    hands.sort_by(|h1, h2| compare_hands(h1, h2, "23456789TJQKA"));
    let sum = hands.iter().enumerate().map(|(i, h)| ((i as i32) + 1) * (&h).bid).sum::<i32>();
    println!("Part 1: {}", sum);
}

fn part2() {
    let input = include_str!("input1.txt");
    let mut hands: Vec<Hand> = input.lines().map(|l| parse_hand(l, calc_rank_joker)).collect();
    hands.sort_by(|h1, h2| compare_hands(h1, h2, "J23456789TQKA"));
    let sum = hands.iter().enumerate().map(|(i, h)| ((i as i32) + 1) * (&h).bid).sum::<i32>();
    println!("Part 2: {}", sum);
}

fn main() {
    part1();
    part2();
}
