use anyhow::{bail, Ok, Result};

pub fn read_password() -> Result<String> {
    let password: String = rpassword::prompt_password("Your password: ")?
        .trim()
        .to_owned();
    if password.len() < 4 {
        bail!("Password is too short")
    }
    Ok(password)
}
