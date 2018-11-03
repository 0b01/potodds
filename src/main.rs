#![feature(custom_attribute)]
#![feature(vec_remove_item)]

extern crate rand;
#[macro_use]
extern crate stdweb;

use rand::prelude::*;
use stdweb::web::event::ClickEvent;
use stdweb::web::{document, IParentNode, IEventTarget};

const SIZES : [f32; 10] = [
    100.,
    150.,
    200.,
    300.,
    400.,
    500.,
    800.,
    1000.,
    1200.,
    2000.,
];

const BETS: [f32; 10] = [
    0.08,
    0.1,
    0.15,
    0.175,
    0.18,
    0.2,
    0.5,
    0.75,
    0.8,
    1.,
];

pub mod card;
pub mod evaluate;
use card::{Card, Suit, Rank, Flop, Hand};

pub struct Deck {
    deck: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut deck = Vec::new();
        for i in 0..13 {
            for j in 0..4 {
                let suit = Suit::get(j);
                let num = Rank::get(i);
                deck.push(Card(suit, num));
            }
        }

        Self {
            deck,
        }
    }

    pub fn remove_card(&mut self, card: &Card) -> bool {
        self.deck.remove_item(card).is_some()
    }

    pub fn _deal_one_random(&mut self) -> Option<Card> {
        let i : usize = random::<usize>() % self.deck.len() as usize;
        Some(self.deck[i])
    }

    pub fn deal_one_rand(&mut self) -> Option<Card> {
        let card = self._deal_one_random()?;
        self.remove_card(&card);
        Some(card)
    }

    pub fn deal_suit(&mut self, suit: Suit) -> Option<Card> {
        let mut card = self._deal_one_random();
        while card?.0 != suit {
            card = self._deal_one_random();
        }
        self.remove_card(&card?);
        card
    }

    pub fn deal_rank(&mut self, rank: Rank) -> Option<Card> {
        let mut card = self._deal_one_random();
        while card?.1 != rank {
            card = self._deal_one_random();
        }
        self.remove_card(&card?);
        card
    }

    pub fn flush_draw(&mut self) -> Option<(Flop, Hand, Card)> {
        let suit = Suit::get(random());
        // hand
        let card1 = self.deal_suit(suit)?;
        let card2 = self.deal_suit(suit)?;
        let hand = Hand([card1, card2]);
        // board
        let card3 = self.deal_suit(suit)?;
        let card4 = self.deal_suit(suit)?;
        // last card must not be same suit
        let mut card5 = self._deal_one_random()?;
        while card5.0 == suit {
            card5 = self._deal_one_random()?;
        }
        self.remove_card(&card5);
        let flop = Flop ([card3, card4, card5]);

        //turn
        let mut turn = self._deal_one_random()?;
        while turn.0 == suit
           || turn.1 == (hand.0[0]).1
           || turn.1 == (hand.0[1]).1
        {
            turn = self._deal_one_random()?;
        }
        self.remove_card(&turn);

        Some((flop, hand, turn))
    }

    pub fn hole_card(&mut self) -> Option<(Flop, Hand, Card)> {
        let start = random::<u64>() % 8;
        // hand
        let card1 = self.deal_rank(Rank::get(start))?;
        let card2 = self.deal_rank(Rank::get(start+1))?;
        let hand = Hand ([card1, card2]);
        // board
        let card3 = self.deal_rank(Rank::get(start+3))?;
        let card4 = self.deal_rank(Rank::get(start+4))?;
        let mut card5 = self._deal_one_random()?;
        while card5.1 == Rank::get(start+2) {
            card5 = self._deal_one_random()?;
        }
        self.remove_card(&card5);
        let flop = Flop ([card3, card4, card5]);

        //turn
        let mut turn = self._deal_one_random()?;
        while turn.1 == Rank::get(start+2)
           || turn.1 == (hand.0[0]).1
           || turn.1 == (hand.0[1]).1
        {
            turn = self._deal_one_random()?;
        }
        self.remove_card(&turn);

        Some((flop, hand, turn))
    }

    pub fn de_straight(&mut self) -> Option<(Flop, Hand, Card)> {
        let start = random::<u64>() % 7;
        // hand
        let card1 = self.deal_rank(Rank::get(start+1))?;
        let card2 = self.deal_rank(Rank::get(start+2))?;
        let hand = Hand ([card1, card2]);
        // board
        let card3 = self.deal_rank(Rank::get(start+3))?;
        let card4 = self.deal_rank(Rank::get(start+4))?;
        let mut card5 = self.deal_one_rand()?;
        while card5.1 == Rank::get(start+5)
           || card5.1 == Rank::get(start)
        {
            card5 = self.deal_one_rand()?;
        }
        let flop = Flop ([card3, card4, card5]);

        //turn
        let mut turn = self._deal_one_random()?;
        while turn.1 == Rank::get(start)
           || turn.1 == Rank::get(start+5)
           || turn.1 == (hand.0[0]).1
           || turn.1 == (hand.0[1]).1
        {
            turn = self._deal_one_random()?;
        }
        self.remove_card(&turn);

        Some((flop, hand, turn))
    }
}

pub struct Pot {
    pub size: f32,
}

impl Pot {
    pub fn new(size: f32) -> Self {
        Pot {
            size
        }
    }
    pub fn rand_pot() -> Self {
        Pot::new(SIZES[random::<usize>() % 10])
    }
    pub fn rand_bet(&self) -> f32 {
        self.size * BETS[random::<usize>() % 10]
    }
    pub fn odds(&self, bet: f32) -> f32 {
        bet / (bet + self.size)
    }
}

struct Scenario {
    pot: Pot,
    bet: f32,
    flop: Option<Flop>,
    hand: Option<Hand>,
    turn: Option<Card>,
    equity: Option<f32>,
    turn_equity: Option<f32>,
}


fn append_txt(txt: &str) {
    js! {
        document.querySelector("#txtbox").innerHTML += @{txt} + "<br />";
    }
}

fn clear_txt() {
    js! {
        document.querySelector("#txtbox").innerHTML = "";
    }
}

impl Scenario {
    fn new() -> Self {
        let pot = Pot::rand_pot();
        let mut ret = Self {
            pot,
            bet: 0.,
            flop: None,
            hand: None,
            equity: None,
            turn: None,
            turn_equity: None,
        };
        ret.init();
        ret
    }

    fn init(&mut self) {
        match random::<u8>() % 3 {
            0 => self.hole_card(),
            1 => self.de_straight_draw(),
            2 => self.flush_draw(),
            _ => unimplemented!(),
        };
    }

    fn flop(&mut self) {
        self.bet = self.pot.rand_bet();
        append_txt(&format!("Pot ${}, bet ${}", self.pot.size, self.bet));
        append_txt(&format!("Flop: {}", self.flop.unwrap()));
        append_txt(&format!("Hand: {}", self.hand.unwrap()));
        append_txt(&format!("
        <div class='flop'>
            <button>Call</button>
            <button>Fold</button>
        </div>
        "));
    }

    fn answer(&mut self) {
        let pot_odds = self.pot.odds(self.bet);
        append_txt(&format!("<div class='answer'>Pot odds: {:.2}, equity: {:.2}</div>", pot_odds, self.equity.unwrap()));
        let eqt = self.turn_equity.unwrap();
        let potsize = self.pot.size + self.bet;
        let ev = potsize * eqt - self.bet * (1. - eqt);
        append_txt(&format!("<div class='answer'>EV = ${:.2} x {:.2} - ${:.2} * {:.2} = {:.2}</div>", potsize, eqt, self.bet, 1. - eqt, ev));
        self.pot.size += self.bet * 2.;
    }

    fn turn(&mut self) {
        self.bet = self.pot.rand_bet();
        append_txt(&format!("Turn: {}.", self.turn.unwrap()));
        append_txt(&format!("Pot ${:.2}, Bet ${:.2}", self.pot.size, self.bet));
        append_txt(&format!("
            <div class='turn'>
                <button>Call</button>
                <button>Fold</button>
            </div>
        "));
    }

    fn flush_draw(&mut self) {
        append_txt("Scenario: Flush Draw");
        let mut deck = Deck::new();
        let (flop, hand, turn) = deck.flush_draw().unwrap();
        self.flop = Some(flop);
        self.hand = Some(hand);
        self.turn = Some(turn);
        self.equity = Some(9./47.);
        self.turn_equity = Some(9./46.);
    }

    fn de_straight_draw(&mut self) {
        append_txt("Scenario: Straight Draw");
        let mut deck = Deck::new();
        let (flop, hand, turn) = deck.de_straight().unwrap();
        self.flop = Some(flop);
        self.hand = Some(hand);
        self.turn = Some(turn);
        self.equity = Some(8./47.);
        self.turn_equity = Some(8./46.);
    }

    fn hole_card(&mut self) {
        append_txt("Scenario: Straight Draw");
        let mut deck = Deck::new();
        let (flop, hand, turn) = deck.hole_card().unwrap();
        self.flop = Some(flop);
        self.hand = Some(hand);
        self.turn = Some(turn);
        self.equity = Some(4./47.);
        self.turn_equity = Some(4./46.);
    }
}

use std::rc::Rc;
use std::cell::RefCell;

fn remove(selector: &str) {
    js! {
        let x = document.querySelector(@{selector});
        while(x) {x.remove();x = document.querySelector(@{selector}); };
    };

}

fn main() {
    stdweb::initialize();
    let button = document().query_selector( "#new" ).unwrap().unwrap();
    button.add_event_listener( move |_: ClickEvent| {
        clear_txt();
        let s = Rc::new(RefCell::new(Scenario::new()));
        let s_clone = s.clone();
        s_clone.borrow_mut().flop();
        let s_clone = s.clone();
        let answer_btn = document().query_selector( ".flop" ).unwrap().unwrap();
        answer_btn.add_event_listener( move |_: ClickEvent| {
            remove(".flop");
            s_clone.borrow_mut().answer();
            s_clone.borrow_mut().turn();

            let sc = s_clone.clone();
            let ans_btn = document().query_selector( ".turn" ).unwrap().unwrap();
            ans_btn.add_event_listener( move |_: ClickEvent| {
                remove(".turn");
                sc.borrow_mut().answer();
            });
        });
    });
    stdweb::event_loop();
}
