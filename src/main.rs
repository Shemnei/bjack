#![allow(rustdoc::private_intra_doc_links)]
#![deny(
    // Documentation
	// TODO: rustdoc::broken_intra_doc_links,
	// TODO: rustdoc::missing_crate_level_docs,
	// TODO: missing_docs,
	// TODO: clippy::missing_docs_in_private_items,

    // Other
	deprecated_in_future,
	exported_private_dependencies,
	future_incompatible,
	missing_copy_implementations,
	missing_debug_implementations,
	private_in_public,
	rust_2018_compatibility,
	rust_2018_idioms,
	trivial_casts,
	trivial_numeric_casts,
	unsafe_code,
	unstable_features,
	unused_import_braces,
	unused_qualifications,

	// clippy attributes
	clippy::missing_const_for_fn,
	clippy::redundant_pub_crate,
	clippy::use_self
)]
#![cfg_attr(docsrs, feature(doc_cfg), feature(doc_alias))]

use std::borrow::Cow;
use std::fmt::{self, Write as _};
use std::io::Write as _;

use rand::prelude::SliceRandom as _;
use rand::thread_rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Suit {
	Club,
	Diamond,
	Heart,
	Spade,
}

impl fmt::Display for Suit {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Suit::Club => f.write_str("♣"),
			Suit::Diamond => f.write_str("♦"),
			Suit::Heart => f.write_str("♥"),
			Suit::Spade => f.write_str("♠"),
		}
	}
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rank {
	Ace,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Jack,
	Queen,
	King,
}

impl fmt::Display for Rank {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Rank::Ace => f.write_str("A"),
			Rank::Two => f.write_str("2"),
			Rank::Three => f.write_str("3"),
			Rank::Four => f.write_str("4"),
			Rank::Five => f.write_str("5"),
			Rank::Six => f.write_str("6"),
			Rank::Seven => f.write_str("7"),
			Rank::Eight => f.write_str("8"),
			Rank::Nine => f.write_str("9"),
			Rank::Ten => f.write_str("10"),
			Rank::Jack => f.write_str("J"),
			Rank::Queen => f.write_str("Q"),
			Rank::King => f.write_str("K"),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Card(Suit, Rank);

impl Card {
	pub const CLUB_ACE: Self = Self(Suit::Club, Rank::Ace);
	pub const CLUB_EIGHT: Self = Self(Suit::Club, Rank::Eight);
	pub const CLUB_FIVE: Self = Self(Suit::Club, Rank::Five);
	pub const CLUB_FOUR: Self = Self(Suit::Club, Rank::Four);
	pub const CLUB_JACK: Self = Self(Suit::Club, Rank::Jack);
	pub const CLUB_KING: Self = Self(Suit::Club, Rank::King);
	pub const CLUB_NINE: Self = Self(Suit::Club, Rank::Nine);
	pub const CLUB_QUEEN: Self = Self(Suit::Club, Rank::Queen);
	pub const CLUB_SEVEN: Self = Self(Suit::Club, Rank::Seven);
	pub const CLUB_SIX: Self = Self(Suit::Club, Rank::Six);
	pub const CLUB_TEN: Self = Self(Suit::Club, Rank::Ten);
	pub const CLUB_THREE: Self = Self(Suit::Club, Rank::Three);
	pub const CLUB_TWO: Self = Self(Suit::Club, Rank::Two);
	pub const DIAMOND_ACE: Self = Self(Suit::Diamond, Rank::Ace);
	pub const DIAMOND_EIGHT: Self = Self(Suit::Diamond, Rank::Eight);
	pub const DIAMOND_FIVE: Self = Self(Suit::Diamond, Rank::Five);
	pub const DIAMOND_FOUR: Self = Self(Suit::Diamond, Rank::Four);
	pub const DIAMOND_JACK: Self = Self(Suit::Diamond, Rank::Jack);
	pub const DIAMOND_KING: Self = Self(Suit::Diamond, Rank::King);
	pub const DIAMOND_NINE: Self = Self(Suit::Diamond, Rank::Nine);
	pub const DIAMOND_QUEEN: Self = Self(Suit::Diamond, Rank::Queen);
	pub const DIAMOND_SEVEN: Self = Self(Suit::Diamond, Rank::Seven);
	pub const DIAMOND_SIX: Self = Self(Suit::Diamond, Rank::Six);
	pub const DIAMOND_TEN: Self = Self(Suit::Diamond, Rank::Ten);
	pub const DIAMOND_THREE: Self = Self(Suit::Diamond, Rank::Three);
	pub const DIAMOND_TWO: Self = Self(Suit::Diamond, Rank::Two);
	pub const HEART_ACE: Self = Self(Suit::Heart, Rank::Ace);
	pub const HEART_EIGHT: Self = Self(Suit::Heart, Rank::Eight);
	pub const HEART_FIVE: Self = Self(Suit::Heart, Rank::Five);
	pub const HEART_FOUR: Self = Self(Suit::Heart, Rank::Four);
	pub const HEART_JACK: Self = Self(Suit::Heart, Rank::Jack);
	pub const HEART_KING: Self = Self(Suit::Heart, Rank::King);
	pub const HEART_NINE: Self = Self(Suit::Heart, Rank::Nine);
	pub const HEART_QUEEN: Self = Self(Suit::Heart, Rank::Queen);
	pub const HEART_SEVEN: Self = Self(Suit::Heart, Rank::Seven);
	pub const HEART_SIX: Self = Self(Suit::Heart, Rank::Six);
	pub const HEART_TEN: Self = Self(Suit::Heart, Rank::Ten);
	pub const HEART_THREE: Self = Self(Suit::Heart, Rank::Three);
	pub const HEART_TWO: Self = Self(Suit::Heart, Rank::Two);
	pub const SPADE_ACE: Self = Self(Suit::Spade, Rank::Ace);
	pub const SPADE_EIGHT: Self = Self(Suit::Spade, Rank::Eight);
	pub const SPADE_FIVE: Self = Self(Suit::Spade, Rank::Five);
	pub const SPADE_FOUR: Self = Self(Suit::Spade, Rank::Four);
	pub const SPADE_JACK: Self = Self(Suit::Spade, Rank::Jack);
	pub const SPADE_KING: Self = Self(Suit::Spade, Rank::King);
	pub const SPADE_NINE: Self = Self(Suit::Spade, Rank::Nine);
	pub const SPADE_QUEEN: Self = Self(Suit::Spade, Rank::Queen);
	pub const SPADE_SEVEN: Self = Self(Suit::Spade, Rank::Seven);
	pub const SPADE_SIX: Self = Self(Suit::Spade, Rank::Six);
	pub const SPADE_TEN: Self = Self(Suit::Spade, Rank::Ten);
	pub const SPADE_THREE: Self = Self(Suit::Spade, Rank::Three);
	pub const SPADE_TWO: Self = Self(Suit::Spade, Rank::Two);

	pub const fn suit(&self) -> &Suit {
		&self.0
	}

	pub const fn rank(&self) -> &Rank {
		&self.1
	}
}

impl fmt::Display for Card {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let base = match self.suit() {
			Suit::Club => '\u{1F0A1}',
			Suit::Diamond => '\u{1F0B1}',
			Suit::Heart => '\u{1F0C1}',
			Suit::Spade => '\u{1F0D1}',
		};

		let mut add = *self.rank() as u8 as u32;
		if self.rank() > &Rank::Jack {
			add += 1;
		}

		f.write_char(char::from_u32(base as u32 + add).unwrap())
	}
}

pub trait Deck {
	fn shuffle(&mut self);

	fn draw(&mut self) -> Option<Card>;

	fn len(&self) -> usize;

	fn is_empty(&self) -> bool {
		self.len() == 0
	}
}

pub type Deck52 = ArrayDeck<52>;
#[rustfmt::skip]
pub const DECK_52: Deck52 = Deck52::new([
    Card::CLUB_ACE, Card::CLUB_TWO, Card::CLUB_THREE, Card::CLUB_FOUR,
    Card::CLUB_FIVE, Card::CLUB_SIX, Card::CLUB_SEVEN, Card::CLUB_EIGHT,
    Card::CLUB_NINE, Card::CLUB_TEN, Card::CLUB_JACK, Card::CLUB_QUEEN,
    Card::CLUB_KING,
    Card::DIAMOND_ACE, Card::DIAMOND_TWO, Card::DIAMOND_THREE,
    Card::DIAMOND_FOUR, Card::DIAMOND_FIVE, Card::DIAMOND_SIX,
    Card::DIAMOND_SEVEN, Card::DIAMOND_EIGHT, Card::DIAMOND_NINE,
    Card::DIAMOND_TEN, Card::DIAMOND_JACK, Card::DIAMOND_QUEEN,
    Card::DIAMOND_KING,
    Card::HEART_ACE, Card::HEART_TWO, Card::HEART_THREE, Card::HEART_FOUR,
    Card::HEART_FIVE, Card::HEART_SIX, Card::HEART_SEVEN, Card::HEART_EIGHT,
    Card::HEART_NINE, Card::HEART_TEN, Card::HEART_JACK, Card::HEART_QUEEN,
    Card::HEART_KING,
    Card::SPADE_ACE, Card::SPADE_TWO, Card::SPADE_THREE, Card::SPADE_FOUR,
    Card::SPADE_FIVE, Card::SPADE_SIX, Card::SPADE_SEVEN, Card::SPADE_EIGHT,
    Card::SPADE_NINE, Card::SPADE_TEN, Card::SPADE_JACK, Card::SPADE_QUEEN,
    Card::SPADE_KING,
]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArrayDeck<const SIZE: usize> {
	// index on which the last valid card resides. All cards 0..valid_idx should
	// be valid.
	valid_idx: usize,
	cards: [Card; SIZE],
}

impl<const SIZE: usize> ArrayDeck<SIZE> {
	pub const fn new(cards: [Card; SIZE]) -> Self {
		Self { valid_idx: SIZE, cards }
	}

	pub fn cards(&self) -> Iter<'_> {
		Iter::new(&self.cards[..self.valid_idx])
	}
}

impl<const SIZE: usize> Deck for ArrayDeck<SIZE> {
	fn shuffle(&mut self) {
		(&mut self.cards[..self.valid_idx]).shuffle(&mut thread_rng())
	}

	fn draw(&mut self) -> Option<Card> {
		if self.is_empty() {
			None
		} else {
			self.valid_idx -= 1;
			Some(self.cards[self.valid_idx])
		}
	}

	fn len(&self) -> usize {
		self.valid_idx
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VecDeck(Vec<Card>);

impl VecDeck {
	pub const fn new(cards: Vec<Card>) -> Self {
		Self(cards)
	}
}

impl Deck for VecDeck {
	fn shuffle(&mut self) {
		self.0.shuffle(&mut thread_rng());
	}

	fn draw(&mut self) -> Option<Card> {
		self.0.pop()
	}

	fn len(&self) -> usize {
		self.0.len()
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Iter<'a> {
	idx: usize,
	cards: &'a [Card],
}

impl<'a> Iter<'a> {
	pub const fn new(cards: &'a [Card]) -> Self {
		Self { idx: 0, cards }
	}
}

impl<'a> Iterator for Iter<'a> {
	type Item = &'a Card;

	fn next(&mut self) -> Option<Self::Item> {
		if self.idx == self.cards.len() {
			None
		} else {
			let card = &self.cards[self.idx];
			self.idx += 1;
			Some(card)
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Decision {
	Hit,
	Stand,
	DoubleDown,
	Split,
	Surrender,
}

impl fmt::Display for Decision {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		// TODO: real
		fmt::Debug::fmt(&self, f)
	}
}

pub trait Score {
	type Output;

	fn score(&self, hand: &Hand) -> Self::Output;
	fn is_bust(&self, hand: &Hand) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlackjackScore;

impl Score for BlackjackScore {
	type Output = (u8, Option<u8>);

	fn score(&self, hand: &Hand) -> Self::Output {
		let score =
			hand.cards().iter().map(|c| ((*c.rank() as u8) + 1).min(10)).sum();

		if score <= 11 && hand.cards().iter().any(|c| c.rank() == &Rank::Ace) {
			(score, Some(score + 10))
		} else {
			(score, None)
		}
	}

	fn is_bust(&self, hand: &Hand) -> bool {
		self.score(hand).0 > 21
	}
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hand {
	cards: Vec<Card>,
}

impl Hand {
	pub fn add(&mut self, card: Card) {
		self.cards.push(card);
	}

	pub fn score<S>(&self, score: &S) -> S::Output
	where
		S: Score,
	{
		score.score(self)
	}

	pub fn is_bust<S>(&self, score: &S) -> bool
	where
		S: Score,
	{
		score.is_bust(self)
	}

	pub fn cards(&self) -> &[Card] {
		&self.cards
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Event<'a> {
	Player { player: usize, event: PlayerEvent<'a> },
	Dealer(DealerEvent<'a>),
	Win(WinEvent),
}

impl<'a> Event<'a> {
	pub const fn player(player: usize, event: PlayerEvent<'a>) -> Self {
		Self::Player { player, event }
	}

	pub const fn dealer(event: DealerEvent<'a>) -> Self {
		Self::Dealer(event)
	}

	pub const fn win(event: WinEvent) -> Self {
		Self::Win(event)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlayerEvent<'a> {
	TurnStart,
	TurnEnd,
	ChooseDecision(Decision),
	ExecuteDecision(Decision),
	InvalidDecision { invalid_decision: Decision, reason: Cow<'static, str> },
	Draw(Card),
	InitialHand { hand: &'a Hand, score: (u8, Option<u8>) },
	HandChange { hand: &'a Hand, score: (u8, Option<u8>) },
	Bust { score: u8 },
	Surrender,
	Blackjack,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DealerEvent<'a> {
	TurnStart,
	TurnEnd,
	Decision(Decision),
	Draw(Card),
	InitialHand { hand: &'a Hand, score: (u8, Option<u8>) },
	HandChange { hand: &'a Hand, score: (u8, Option<u8>) },
	Bust { score: u8 },
	Blackjack,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WinEvent {
	Dealer,
	Players(Vec<usize>),
	Push(Vec<usize>),
	Lose(Vec<usize>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Blackjack<D, S> {
	dealer: Hand,
	hands: Vec<Hand>,
	deck: D,
	score: S,
}

impl<D> Blackjack<D, BlackjackScore>
where
	D: Deck,
{
	pub fn with_players(players: usize, deck: D) -> Self {
		Self::with_hands(Hand::default(), vec![Hand::default(); players], deck)
	}

	pub fn with_hands(dealer: Hand, hands: Vec<Hand>, mut deck: D) -> Self {
		deck.shuffle();

		Self { dealer, hands, deck, score: BlackjackScore }
	}

	pub fn dealer(&self) -> &Hand {
		&self.dealer
	}

	pub fn player(&self, player: usize) -> Option<&Hand> {
		self.hands.get(player)
	}

	pub fn score(
		&self,
		player: usize,
	) -> Option<<BlackjackScore as Score>::Output> {
		self.hands.get(player).map(|hand| hand.score(&self.score))
	}

	pub fn players(&self) -> usize {
		self.hands.len()
	}

	pub fn run<I, E>(mut self, mut input: I, mut event: E)
	where
		I: FnMut(usize, &Self) -> Decision,
		E: for<'a> FnMut(Event<'a>),
	{
		let mut out_of_the_game = Vec::with_capacity(self.hands.len());

		// initial draws
		// TODO: maybe first 1 for each and then give second round
		for player in 0..self.hands.len() {
			let card1 = self.deck.draw().unwrap();
			let card2 = self.deck.draw().unwrap();
			let hand = &mut self.hands[player];
			hand.add(card1);
			hand.add(card2);
			event(Event::player(
				player,
				PlayerEvent::InitialHand {
					hand,
					score: hand.score(&self.score),
				},
			));
		}

		let card1 = self.deck.draw().unwrap();
		let card2 = self.deck.draw().unwrap();
		let hand = &mut self.dealer;
		hand.add(card1);
		hand.add(card2);
		event(Event::dealer(DealerEvent::InitialHand {
			hand,
			score: hand.score(&self.score),
		}));

		// check blackjack
		let player_blackjacks = (0..self.hands.len())
			.filter(|&player| {
				self.hands[player].score(&self.score).1 == Some(21)
			})
			.collect::<Vec<_>>();

		for player in &player_blackjacks {
			event(Event::player(*player, PlayerEvent::Blackjack));
		}

		let dealer_blackjack = self.dealer.score(&self.score).1 == Some(21);

		if dealer_blackjack {
			event(Event::dealer(DealerEvent::Blackjack));
		}

		if player_blackjacks.is_empty() {
			if dealer_blackjack {
				event(Event::win(WinEvent::Dealer));
				event(Event::win(WinEvent::Lose(
					(0..self.hands.len()).collect(),
				)));
				return;
			}
		} else {
			let losers = (0..self.hands.len())
				.filter(|player| !player_blackjacks.contains(player))
				.collect();

			let win_event = if dealer_blackjack {
				WinEvent::Push(player_blackjacks)
			} else {
				WinEvent::Players(player_blackjacks)
			};

			event(Event::win(win_event));
			event(Event::win(WinEvent::Lose(losers)));
			return;
		}

		// player
		for player in 0..self.hands.len() {
			let mut decisions = 0;

			event(Event::player(player, PlayerEvent::TurnStart));

			while !self.hands[player].is_bust(&self.score) {
				let decision = input(player, &self);
				decisions += 1;
				event(Event::player(
					player,
					PlayerEvent::ChooseDecision(decision),
				));

				match decision {
					Decision::Hit => {
						event(Event::player(
							player,
							PlayerEvent::ExecuteDecision(decision),
						));
						let card = self.deck.draw().unwrap();
						event(Event::player(player, PlayerEvent::Draw(card)));
						let hand = &mut self.hands[player];
						hand.add(card);
						event(Event::player(
							player,
							PlayerEvent::HandChange {
								hand,
								score: hand.score(&self.score),
							},
						));
					}
					Decision::Stand => {
						event(Event::player(
							player,
							PlayerEvent::ExecuteDecision(decision),
						));
						break;
					}
					Decision::DoubleDown => {
						event(Event::player(
							player,
							PlayerEvent::ExecuteDecision(decision),
						));
						let card = self.deck.draw().unwrap();
						event(Event::player(player, PlayerEvent::Draw(card)));
						let hand = &mut self.hands[player];
						hand.add(card);
						event(Event::player(
							player,
							PlayerEvent::HandChange {
								hand,
								score: hand.score(&self.score),
							},
						));
						break;
					}
					Decision::Split => todo!(),
					Decision::Surrender => {
						if decisions == 1 {
							event(Event::player(
								player,
								PlayerEvent::ExecuteDecision(decision),
							));
							Event::player(player, PlayerEvent::Surrender);
							out_of_the_game.push(player);
							break;
						} else {
							event(Event::player(
								player,
								PlayerEvent::InvalidDecision {
									invalid_decision: decision,
									reason: Cow::Borrowed(
										"Can only surrender at the start of \
										 a game",
									),
								},
							));
						}
					}
				}
			}

			if self.hands[player].is_bust(&self.score) {
				let score = self.hands[player].score(&self.score);
				event(Event::player(
					player,
					PlayerEvent::Bust { score: score.0 },
				));
				out_of_the_game.push(player);
			}

			event(Event::player(player, PlayerEvent::TurnEnd));
		}

		if out_of_the_game.len() == self.hands.len() {
			event(Event::win(WinEvent::Dealer));
			return;
		}

		// dealer
		event(Event::dealer(DealerEvent::TurnStart));

		while {
			let score = self.dealer.score(&self.score);
			score.1.unwrap_or(score.0) < 17
		} {
			event(Event::dealer(DealerEvent::Decision(Decision::Hit)));
			let card = self.deck.draw().unwrap();
			event(Event::dealer(DealerEvent::Draw(card)));
			let hand = &mut self.dealer;
			hand.add(card);
			event(Event::dealer(DealerEvent::HandChange {
				hand,
				score: hand.score(&self.score),
			}));
		}

		let dealer_bust = self.dealer.is_bust(&self.score);
		if dealer_bust {
			event(Event::dealer(DealerEvent::Bust {
				score: self.dealer.score(&self.score).0,
			}));

			let players = (0..self.hands.len())
				.filter(|player| !out_of_the_game.contains(player))
				.collect();

			event(Event::win(WinEvent::Players(players)));
			event(Event::win(WinEvent::Lose(out_of_the_game)));
			return;
		} else {
			event(Event::dealer(DealerEvent::Decision(Decision::Stand)));
		}

		event(Event::dealer(DealerEvent::TurnEnd));

		// win
		let scores: Vec<(usize, u8)> = (0..self.hands.len())
			.filter(|player| !out_of_the_game.contains(player))
			.map(|player| {
				let score = self.hands[player].score(&self.score);
				(player, score.1.unwrap_or(score.0))
			})
			.collect();

		let dealer_score = self.dealer.score(&self.score);
		let dealer_score = dealer_score.1.unwrap_or(dealer_score.0);

		let winner = scores
			.iter()
			.filter(|(_, score)| score > &dealer_score)
			.map(|(player, _)| *player)
			.collect::<Vec<_>>();
		let draws = scores
			.iter()
			.filter(|(_, score)| score == &dealer_score)
			.map(|(player, _)| *player)
			.collect::<Vec<_>>();
		let losers = scores
			.iter()
			.filter(|(_, score)| score < &dealer_score)
			.map(|(player, _)| *player)
			.chain(out_of_the_game)
			.collect::<Vec<_>>();

		if winner.is_empty() {
			if draws.is_empty() {
				event(Event::win(WinEvent::Dealer));
			} else {
				event(Event::win(WinEvent::Push(draws)));
			}
		} else {
			event(Event::win(WinEvent::Players(winner)));
			if !draws.is_empty() {
				event(Event::win(WinEvent::Push(draws)));
			}
		}
		if !losers.is_empty() {
			event(Event::win(WinEvent::Lose(losers)));
		}
	}
}

fn main() {
	let bjack = Blackjack::with_players(4, DECK_52);

	let mut last_player = usize::MAX;

	// TODO: cleanup
	bjack.run(
		|player, game| {
			if player != last_player {
				let score = game.score(player).unwrap();
				println!(
					">> Player {} hand {} (Score: {}{})",
					player,
					game.player(player)
						.unwrap()
						.cards()
						.iter()
						.map(|c| c.to_string())
						.collect::<Vec<_>>()
						.join(" "),
					score.0,
					score
						.1
						.map(|s| format!("/{}", s))
						.unwrap_or_else(String::new)
				);
				last_player = player;
			}

			let mut buf = String::new();
			loop {
				println!(
					"Options [H]it, [S]tand, [D]oubleDown, S[P]lit, \
					 Su[R]render]"
				);
				print!(">> Player {} choose: ", player);
				std::io::stdout().flush().unwrap();

				let _ = std::io::stdin().read_line(&mut buf).unwrap();

				buf.make_ascii_lowercase();
				match buf.as_str().trim() {
					"h" => return Decision::Hit,
					"s" => return Decision::Stand,
					"d" => return Decision::DoubleDown,
					"p" => return Decision::Split,
					"r" => return Decision::Surrender,
					_ => buf.clear(),
				};
			}
		},
		|event| {
			match event {
				Event::Player { player, event } => match event {
					PlayerEvent::TurnStart => {
						println!("--- >> Player {} << ---", player)
					}
					PlayerEvent::TurnEnd => {
						println!("-------------------------")
					}
					PlayerEvent::ChooseDecision(_decision) => {}
					PlayerEvent::ExecuteDecision(_decision) => {}
					PlayerEvent::InvalidDecision {
						invalid_decision: _invalid_decision,
						reason,
					} => println!(
						"!! Player {} invalid decision: {}",
						player, reason
					),
					PlayerEvent::Draw(card) => {
						println!(">> Player {} drew {}", player, card)
					}
					PlayerEvent::InitialHand { hand, score } => println!(
						">> Player {} initial hand {} (Score: {}{})",
						player,
						hand.cards()
							.iter()
							.map(|c| c.to_string())
							.collect::<Vec<_>>()
							.join(" "),
						score.0,
						score
							.1
							.map(|s| format!("/{}", s))
							.unwrap_or_else(String::new)
					),
					PlayerEvent::HandChange { hand, score } => println!(
						">> Player {} hand {} (Score: {}{})",
						player,
						hand.cards()
							.iter()
							.map(|c| c.to_string())
							.collect::<Vec<_>>()
							.join(" "),
						score.0,
						score
							.1
							.map(|s| format!("/{}", s))
							.unwrap_or_else(String::new)
					),
					PlayerEvent::Bust { score } => {
						println!(
							">> Player {} went bust ({:?})",
							player, score
						)
					}
					PlayerEvent::Surrender => {
						println!(">> Player {} surrendered", player)
					}
					PlayerEvent::Blackjack => {
						println!(">> Player {} got a blackjack", player)
					}
				},
				Event::Dealer(event) => match event {
					DealerEvent::TurnStart => println!("--- >> Dealer << ---"),
					DealerEvent::TurnEnd => {
						println!("-------------------------")
					}
					DealerEvent::Decision(decision) => {
						println!(">> Dealer chose to {}", decision)
					}
					DealerEvent::Draw(card) => {
						println!(">> Dealer drew {}", card)
					}
					DealerEvent::InitialHand { hand, score } => println!(
						">> Dealer initial hand {} (Score: {}{})",
						hand.cards()
							.iter()
							.map(|c| c.to_string())
							.collect::<Vec<_>>()
							.join(" "),
						score.0,
						score
							.1
							.map(|s| format!("/{}", s))
							.unwrap_or_else(String::new)
					),
					DealerEvent::HandChange { hand, score } => println!(
						">> Dealer hand {} (Score: {}{})",
						hand.cards()
							.iter()
							.map(|c| c.to_string())
							.collect::<Vec<_>>()
							.join(" "),
						score.0,
						score
							.1
							.map(|s| format!("/{}", s))
							.unwrap_or_else(String::new)
					),
					DealerEvent::Bust { score } => {
						println!(">> Dealer went bust ({:?})", score)
					}
					DealerEvent::Blackjack => {
						println!(">> Dealer got a blackjack")
					}
				},
				Event::Win(event) => match event {
					WinEvent::Dealer => println!("$$ Dealer won"),
					WinEvent::Players(players) => {
						println!("$$ Players {:?} won", players)
					}
					WinEvent::Push(players) => {
						println!("$$ Dealer and Players {:?} push", players)
					}
					WinEvent::Lose(players) => {
						println!("$$ Players {:?} lose", players)
					}
				},
			};
		},
	);
}
