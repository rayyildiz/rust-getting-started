use actix::prelude::*;
use rand::prelude::*;

#[derive(Message)]
#[rtype(result = "String")]
pub enum Command {
    SentimentAnalysis,
    LangDetect,
}

pub struct Msg {
    pub text: String,
}

impl Actor for Msg {
    type Context = Context<Self>;
}

impl Handler<Command> for Msg {
    type Result = String;

    fn handle(&mut self, msg: Command, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            Command::SentimentAnalysis => {
                let b: bool = random();
                let response = if b { "positive" } else { "negative" };

                response.to_string()
            }
            Command::LangDetect => {
                let a: u8 = random();
                let lang = match a {
                    0..=25 => "de",
                    26..=50 => "fr",
                    52..=100 => "es",
                    _ => "en",
                };

                lang.to_string()
            }
        }
    }
}
