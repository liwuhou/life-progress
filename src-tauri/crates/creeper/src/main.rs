mod creeper;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = creeper::fetch_data()?;
    // println!("{:?}", res);
    Ok(())
}
