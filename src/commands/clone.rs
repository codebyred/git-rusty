use anyhow::Context;


pub fn run(url: &str) -> anyhow::Result<()> {
    let body =
        reqwest::blocking::get(format!("{}/info/refs?service=git-upload-pack", url))?.text()?;

    parse_git_packet_lines(&body)?;
    Ok(())
}

fn hex_to_len(s: &str) -> anyhow::Result<usize> {
    let len = usize::from_str_radix(s, 16).with_context(|| "Error at str radix")?;
    Ok(len)
}

fn parse_git_packet_lines(data: &str) -> anyhow::Result<()> {
    let mut i = 0;
    let mut is_first_ref = true;

    while i + 4 <= data.len() {
        let len = hex_to_len(&data[i..i + 4])?;
        i += 4;

        if len == 0 {
            continue;
        }

        let payload_len = len - 4;
        if i + payload_len > data.len() {
            break;
        }

        let payload = &data[i..i + payload_len];
        i += payload_len;

        if payload.starts_with("#") {
            continue;
        }

        if is_first_ref {
            if let Some(null_pos) = payload.chars().position(|c| c == '\0') {
                let (before_null, after_null) = payload.split_at(null_pos);
                let after_null = &after_null[1..]; 

                let parts: Vec<&str> = before_null.splitn(2, |c| c == ' ').collect();
                if parts.len() == 2 {
                    println!("SHA: {}", parts[0]);
                    println!("Ref: {}", parts[1]);
                    println!("Capabilities: {}", after_null);
                } else {
                    println!("Malformed first line: {:?}", before_null);
                }
            } else {
                anyhow::bail!("First ref missing null byte separator");
            }

            is_first_ref = false;
        } else {
            let parts: Vec<&str> = payload.splitn(2, |c| c == ' ').collect();
            if parts.len() == 2 {
                println!("SHA: {}", parts[0]);
                println!("Ref: {}", parts[1]);
            } else {
                anyhow::bail!("Malformed ref line: {:?}", payload);
            }
        }
    }

    Ok(())
    
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hex_to_len_with_0000() {
        const SIZE_IN_DECIMAL: usize = 0;

        match hex_to_len("0000") {
            Ok(val) => assert_eq!(SIZE_IN_DECIMAL, val),
            Err(val) => panic!("{}", val.to_string()),
        }
    }
    #[test]
    fn test_hex_to_len_with_001e() {
        const SIZE_IN_DECIMAL: usize = 30;

        match hex_to_len("001e") {
            Ok(val) => assert_eq!(SIZE_IN_DECIMAL, val),
            Err(val) => panic!("{}", val.to_string()),
        }
    }
}
