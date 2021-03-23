use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use rand::seq::SliceRandom;
use itertools::Itertools;

#[derive(Debug, Copy, Clone, EnumIter, PartialOrd, PartialEq)]
enum Suit { Karo, Herz, Pik, Treff, }

#[derive(Debug, Copy, Clone, EnumIter, PartialEq, PartialOrd)]
enum RedSuit{ Vier,Drei,Zwei,As,Bub,Reiter,Dame,Koenig, }

#[derive(Debug, Copy, Clone, EnumIter, PartialEq, PartialOrd)]
enum BlackSuit{ Sieben,Acht,Neun,Zehn,Bub,Reiter,Dame,Koenig, }

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum Card
{
    I,II,III,IIII,V,VI,VII,VIII,IX,X,XI,XII,XIII,XIV,XV,XVI,XVII,XVIII,XIX,XX,XXI,Skus,
    Karo( RedSuit ),
    Herz( RedSuit ),
    Pik( BlackSuit ),
    Treff( BlackSuit ),
}

type Koenig = Suit;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, EnumIter)]
enum Player
{
    Vorhand, Player1, Player2, Geber,
}

#[derive(Debug, Copy, Clone)]
enum Players
{
    One(Player),
    Two(Player,Player),
    Three(Player,Player,Player),
    Four(Player,Player,Player,Player),
}

#[derive(Debug, Copy, Clone)]
enum CallStateA
{
    Init,
    MeinSpiel,
    Rufer(Option<CallKoenig>),
    Piccolo( Players),
    Zwiccolo( Players),
    BesserRuferA(Option<CallKoenig>),
    BesserRuferB( CallKoenig, Card ),
    Dreier,
    Farbensolo,
    SoloDreier,
    Bettler,
    BettlerOuvert,
    PiccoloOuvert( Players),
    ZwiccoloOuvert( Players),
    Trischaken,
    SechserDreier,
    Solo(Option<CallKoenig>),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum CallKoenig
{
    Koenig( Koenig ),
    VierterKoenig,
}

#[derive(Debug, Copy, Clone)]
struct CallStateB
{
    state: CallStateA,
    caller: Option<Player>,
    active: Players,
}

#[derive(Debug, Copy, Clone)]
struct CallStateC
{
    state: CallStateB,
    nextPlayer: Player,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum CallA
{
    MeinSpiel,
    Weiter,
    Rufer,
    Piccolo,
    PiccoloMit,
    Zwiccolo,
    ZwiccoloMit,
    BesserRufer,
    Dreier,
    Farbensolo,
    SoloDreier,
    Bettler,
    BettlerOuvert,
    PiccoloOuvert,
    PiccoloOuvertMit,
    ZwiccoloOuvert,
    ZwiccoloOuvertMit,
    Trischaken,
    SechserDreier,
    Solo,
    Koenig(CallKoenig),
    Kleiner(Card),
    KoenigUltimo,
    Trull,
    Valat,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum CallB
{
    Call( CallA ),
    Kontra( CallA, Player ),
    Rekontra( CallA, Player ),
    Subrekontra( CallA, Player ),
}

fn is_in_active( ps: Players, p: Player ) -> bool
{
    match ps
    {
        Players::One( p1 ) if p1 == p => true,
        Players::Two( p1, p2 ) if p1 == p || p2 == p => true,
        Players::Three( p1, p2, p3 ) if p1 == p || p2 == p || p3 == p => true,
        Players::Four( _, _, _, _ ) => true,
        _ => false,
    }
}

fn other_players( p: Player ) -> Players
{
    match p
    {
        Player::Geber => Players::Three( Player::Vorhand, Player::Player1, Player::Player2 ),
        Player::Player1 => Players::Three( Player::Geber, Player::Vorhand, Player::Player2 ),
        Player::Player2 => Players::Three( Player::Geber, Player::Player1, Player::Vorhand ),
        Player::Vorhand => Players::Three( Player::Geber, Player::Player1, Player::Player2 ),
    }
}

fn without_player( ps: Players, p: Player ) -> Players
{
    match p
    {
        Player::Geber => Players::Three( Player::Vorhand, Player::Player1, Player::Player2 ),
        Player::Player1 => Players::Three( Player::Geber, Player::Vorhand, Player::Player2 ),
        Player::Player2 => Players::Three( Player::Geber, Player::Player1, Player::Vorhand ),
        Player::Vorhand => Players::Three( Player::Geber, Player::Player1, Player::Player2 ),
    }
}

fn next_player( p: Player ) -> Player
{
    match p
    {
        Player::Geber => Player::Vorhand,
        Player::Player1 => Player::Player2,
        Player::Player2 => Player::Geber,
        Player::Vorhand => Player::Player1,
    }
}

fn get_possible_calls( s: CallStateC, turn: Player ) -> Vec< CallB >
{
    match s
    {
        CallStateC{ state, nextPlayer }  => 
        {
            if nextPlayer != turn { return vec![]; }

            match state
            {
                CallStateB{ state, caller, active } =>
                {
                    if !is_in_active(active, nextPlayer ) { return vec![]; }

                    match state
                    {
                        CallStateA::Init => vec![CallA::MeinSpiel, CallA::Rufer, CallA::Piccolo, CallA::Zwiccolo, CallA::BesserRufer, CallA::Dreier, CallA::Farbensolo, CallA::SoloDreier, CallA::Bettler, CallA::BettlerOuvert, CallA::PiccoloOuvert, CallA::ZwiccoloOuvert, CallA::Solo].iter().map(|c|CallB::Call(*c)).collect(),
                        CallStateA::MeinSpiel => 
                            match nextPlayer
                            {
                                Player::Vorhand => vec![CallA::Rufer, CallA::Trischaken, CallA::SechserDreier, CallA::Piccolo, CallA::Zwiccolo, CallA::BesserRufer, CallA::Dreier, CallA::Farbensolo, CallA::SoloDreier, CallA::Bettler, CallA::BettlerOuvert, CallA::PiccoloOuvert, CallA::ZwiccoloOuvert, CallA::Solo].iter().map(|c|CallB::Call(*c)).collect(),
                                _ => vec![CallA::Piccolo, CallA::Zwiccolo, CallA::BesserRufer, CallA::Dreier, CallA::Farbensolo, CallA::SoloDreier, CallA::Bettler, CallA::BettlerOuvert, CallA::PiccoloOuvert, CallA::ZwiccoloOuvert, CallA::Solo].iter().map(|c|CallB::Call(*c)).collect(),
                            },
                        CallStateA::Rufer(koenig) =>
                            match (nextPlayer, caller, koenig)
                            {
                                (np,Some(c), None) if np==c => Koenig::iter().map(|s| CallB::Call(CallA::Koenig(CallKoenig::Koenig(s)))).merge( vec![CallB::Call(CallA::Koenig(CallKoenig::VierterKoenig))]).collect(),
                                (np,Some(c), Some(k)) if np!=c => vec![CallB::Kontra( CallA::Rufer, c), CallB::Kontra( CallA::Koenig(k), c)],
                                _ => vec![],
                            },
                        _ => vec![]
                    }
                }
            }
        }
    }
}

fn apply( s: CallStateC, c: CallB, p: Player ) -> CallStateC
{
    match s
    {
        CallStateC{ state, nextPlayer } => match state
        {
            CallStateB{ state, caller, active } => match c
            {
                CallB::Call( CallA::MeinSpiel ) => CallStateC{ state: CallStateB{ state: CallStateA::MeinSpiel, caller: Some( p ), active: other_players( p ) }, nextPlayer: next_player( p ) },
                CallB::Call( CallA::Weiter ) => CallStateC{ state: CallStateB{ state: state, caller: caller, active: without_player( active, p ) }, nextPlayer: next_player( p ) },
                CallB::Call( CallA::Rufer ) => CallStateC{ state: CallStateB{ state: CallStateA::Rufer(None), caller: Some( p ), active: other_players( p ) }, nextPlayer: next_player( p ) },
                // CallB::Call( CallA::Piccolo ) => CallStateC{ state: CallStateB{ state: CallStateA::MeinSpiel, caller: Some( p ), active: other_players( p ) }, nextPlayer: next_player( p ) },
                // CallB::Call( CallA::PiccoloMit ) => CallStateC{ state: CallStateB{ state: CallStateA::MeinSpiel, caller: Some( p ), active: other_players( p ) }, nextPlayer: next_player( p ) },
                // CallB::Call( CallA::Zwiccolo ) => CallStateC{ state: CallStateB{ state: CallStateA::MeinSpiel, caller: Some( p ), active: other_players( p ) }, nextPlayer: next_player( p ) },
                // CallB::Call( CallA::ZwiccoloMit ) => CallStateC{ state: CallStateB{ state: CallStateA::MeinSpiel, caller: Some( p ), active: other_players( p ) }, nextPlayer: next_player( p ) },
                // CallB::Call( CallA::BesserRufer ) => CallStateC{ state: CallStateB{ state: CallStateA::MeinSpiel, caller: Some( p ), active: other_players( p ) }, nextPlayer: next_player( p ) },
                // CallB::Call( CallA::Dreier ) => CallStateC{ state: CallStateB{ state: CallStateA::MeinSpiel, caller: Some( p ), active: other_players( p ) }, nextPlayer: next_player( p ) },
                // CallB::Call( CallA::Farbensolo ) => CallStateC{ state: CallStateB{ state: CallStateA::MeinSpiel, caller: Some( p ), active: other_players( p ) }, nextPlayer: next_player( p ) },
                // CallB::Call( CallA::SoloDreier ) => CallStateC{ state: CallStateB{ state: CallStateA::SoloDreier, caller: Some( p ), active: other_players( p ) }, nextPlayer: next_player( p ) },
                // CallB::Call( CallA::Bettler ) => CallStateC{ state: CallStateB{ state: CallStateA::Bettler, caller: Some( p ), active: other_players( p ) }, nextPlayer: next_player( p ) },
                // CallB::Call( CallA::BettlerOuvert ) => CallStateC{ state: CallStateB{ state: CallStateA::BettlerOuvert, caller: Some( p ), active: other_players( p ) }, nextPlayer: next_player( p ) },
                // CallB::Call( CallA::PiccoloOuvert ) => CallStateC{ state: CallStateB{ state: CallStateA::PiccoloOuvert, caller: Some( p ), active: other_players( p ) }, nextPlayer: next_player( p ) },
                // CallB::Call( CallA::PiccoloOuvertMit ) => CallStateC{ state: CallStateB{ state: CallStateA::PiccoloOuvertMit, caller: Some( p ), active: other_players( p ) }, nextPlayer: next_player( p ) },
                // CallB::Call( CallA::ZwiccoloOuvert ) => CallStateC{ state: CallStateB{ state: CallStateA::ZwiccoloOuvert, caller: Some( p ), active: other_players( p ) }, nextPlayer: next_player( p ) },
                // CallB::Call( CallA::ZwiccoloOuvertMit ) => CallStateC{ state: CallStateB{ state: CallStateA::ZwiccoloOuvertMit, caller: Some( p ), active: other_players( p ) }, nextPlayer: next_player( p ) },
                // CallB::Call( CallA::Trischaken ) => CallStateC{ state: CallStateB{ state: CallStateA::Trischaken, caller: Some( p ), active: other_players( p ) }, nextPlayer: next_player( p ) },
                // CallB::Call( CallA::SechserDreier ) => CallStateC{ state: CallStateB{ state: CallStateA::SechserDreier, caller: Some( p ), active: other_players( p ) }, nextPlayer: next_player( p ) },
                // CallB::Call( CallA::Solo ) => CallStateC{ state: CallStateB{ state: CallStateA::Solo, caller: Some( p ), active: other_players( p ) }, nextPlayer: next_player( p ) },
                // CallB::Call( CallA::Koenig(CallKoenig) ) => CallStateC{ state: CallStateB{ state: CallStateA::Koenig(CallKoenig), caller: Some( p ), active: other_players( p ) }, nextPlayer: next_player( p ) },
                // CallB::Call( CallA::Kleiner(Card) ) => CallStateC{ state: CallStateB{ state: CallStateA::Kleiner(Card), caller: Some( p ), active: other_players( p ) }, nextPlayer: next_player( p ) },
                // CallB::Call( CallA::KoenigUltimo ) => CallStateC{ state: CallStateB{ state: CallStateA::KoenigUltimo, caller: Some( p ), active: other_players( p ) }, nextPlayer: next_player( p ) },
                // CallB::Call( CallA::Trull ) => CallStateC{ state: CallStateB{ state: CallStateA::KoenigUltimo, caller: Some( p ), active: other_players( p ) }, nextPlayer: next_player( p ) },
                // CallB::Call( CallA::Valat ) => CallStateC{ state: CallStateB{ state: CallStateA::KoenigUltimo, caller: Some( p ), active: other_players( p ) }, nextPlayer: next_player( p ) },
                _ => panic!(""),
            }
        }
    }
}

fn get_full_deck() -> Vec::<Card>{
    vec![
        Card::I,Card::II,Card::III,Card::IIII,Card::V,Card::VI,Card::VII,Card::VIII,Card::IX,Card::X,
        Card::XI,Card::XII,Card::XIII,Card::XIV,Card::XV,Card::XVI,Card::XVII,Card::XVIII,Card::XIX,Card::XX,Card::XXI,Card::Skus,
        Card::Karo( RedSuit::Vier ),Card::Karo( RedSuit::Drei ),Card::Karo( RedSuit::Zwei ),Card::Karo( RedSuit::As ),
        Card::Karo( RedSuit::Bub ),Card::Karo( RedSuit::Reiter ),Card::Karo( RedSuit::Dame ),Card::Karo( RedSuit::Koenig ),
        Card::Herz( RedSuit::Vier ),Card::Herz( RedSuit::Drei ),Card::Herz( RedSuit::Zwei ),Card::Herz( RedSuit::As ),
        Card::Herz( RedSuit::Bub ),Card::Herz( RedSuit::Reiter ),Card::Herz( RedSuit::Dame ),Card::Herz( RedSuit::Koenig ),
        Card::Pik( BlackSuit:: Sieben ),Card::Pik( BlackSuit::Acht ),Card::Pik( BlackSuit::Neun ),Card::Pik( BlackSuit::Zehn ),
        Card::Pik( BlackSuit::Bub ),Card::Pik( BlackSuit::Reiter ),Card::Pik( BlackSuit::Dame ),Card::Pik( BlackSuit::Koenig ),
        Card::Treff( BlackSuit::  Sieben ),Card::Treff( BlackSuit::Acht ),Card::Treff( BlackSuit::Neun ),Card::Treff( BlackSuit::Zehn ),
        Card::Treff( BlackSuit::Bub ),Card::Treff( BlackSuit::Reiter ),Card::Treff( BlackSuit::Dame ),Card::Treff( BlackSuit::Koenig ), ]
}

fn main()
{
    let mut rng = rand::thread_rng();
    let mut deck = get_full_deck();
    deck.shuffle( &mut rng );
    println!("{:?}", deck);

    //let state = CallStateB::State(CallStateA::Init,Player::Vorhand);
    let sa = CallStateA::Rufer( Some( CallKoenig::Koenig( Koenig::Karo ) ) );
    let sb = CallStateB{ state: sa, caller: Some( Player::Vorhand ), active: Players::Three( Player::Geber, Player::Player1, Player::Player2 ) };
    let state = CallStateC{ state: sb, nextPlayer: Player::Player1 };
    println!("{:?}", state);
    let pc = get_possible_calls(state, Player::Player1);
    println!("{:?}", pc);
}
