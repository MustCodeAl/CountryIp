

// Enter an IP address and find the country that IP is registered in. Optional: Find the Ip automatically.



use std::io::stdin;
use std::process::Command;

fn main() {
    
    let mut ip= String::new();
    
    
    println!("Enter an IP address: ");
    stdin().read_line(&mut ip).unwrap();
    
    let ip = ip.trim().to_string();
    
    println!("The IP address is: {}", ip);
    // do a curl request
    // $ curl ipinfo.io/8.8.8.8


    let ipinfo = format!("ipinfo.io/{}", ip);
    let output = Command::new("curl")
        .arg(ipinfo.as_str()).output().expect("failed to execute process");



    // print the result
    println!("The country is: ");
    
    let joutput: serde_json::Value = serde_json::from_str(String::from_utf8_lossy(&output.stdout).to_string().as_str()).unwrap();
    
    let country = joutput.get("country").expect("country not found").as_str().unwrap();

    println!("The country is: {}", country);
    
    
    // println!("{:?}", output);

    let city = joutput.get("city").expect("city not found").as_str().unwrap();
    println!("The city is: {}", city);




}

