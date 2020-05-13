use cookie::CookieJar;

#[derive(Default)]
pub struct CookieParserData {
    cookie_jar: CookieJar,
}

impl CookieParserData {
    pub fn new() -> Self {
        CookieParserData {
            cookie_jar: CookieJar::new(),
        }
    }

    pub fn cookie_jar(&self) -> &CookieJar {
        &self.cookie_jar
    }

    pub fn cookie_jar_mut(&mut self) -> &mut CookieJar {
        &mut self.cookie_jar
    }
}
