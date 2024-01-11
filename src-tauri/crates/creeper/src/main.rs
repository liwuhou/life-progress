mod creeper;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = creeper::fetch()?;
    println!("{}", res);
    Ok(())
}
