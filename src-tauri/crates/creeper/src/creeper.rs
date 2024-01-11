use std::error::Error;

const FETCH_URL: &str = "https://en.wikipedia.org/wiki/List_of_countries_by_life_expectancy";

pub fn fetch() -> Result<String, Box<dyn Error>> {
    let resp = reqwest::blocking::get(FETCH_URL)?.text()?;

    Ok(resp)
}

#[cfg(test)]
mod test {
    use super::*;

    fn it_works() -> Result<(), Box<dyn Error>> {
        let resp = fetch()?;
        println!("{:?}", resp);
        Ok(())
    }
}
