use Bot;

pub trait Method {
    type Response;
    const URL: &'static str;

    fn call(&self, bot: &Bot) -> Result<Self::Response> {
        bot.call(Self::URL, self)
    }
}