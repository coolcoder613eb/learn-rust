extern crate irc;

use irc::client::prelude::*;
use irc::error;

use std::default::Default;


use std::io::prelude::*;

fn get_fortune() -> String {
    let output = std::process::Command::new("fortune")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        String::from_utf8_lossy(&output.stderr).to_string()
    }
}

fn process_msg(client: &IrcClient, message: Message) -> error::Result<()> {
    print!("{}", message);
    if let Command::PRIVMSG(ref target, ref msg) = message.command {
        if msg.contains("fortune") {
            let fortune = get_fortune();
            println!("{}", fortune);
            client.send_privmsg(target, fortune.replace("\n","\r\n"))?;
        } else if msg.contains("quit_bot") {
            client.send_quit("bye")?;
        }
    }
    Ok(())
}

fn main() {
    let config = Config {
        nickname: Some("fortunebot".to_owned()),
        alt_nicks: Some(vec!["rustybot".to_owned()]),
        server: Some("irc.oftc.net".to_owned()),
        channels: Some(vec!["#haiku-3rdparty".to_owned()]),
        ..Default::default()
    };
    let config1 = Config {
        nickname: Some("car".to_owned()),
        alt_nicks: Some(vec!["bansxaanas".to_owned()]),
        server: Some("irc.mozilla.org".to_owned()),
        channels: Some(vec!["#ONU".to_owned()]),
        ..Default::default()
    };

    let config2 = Config {
        nickname: Some("batman".to_owned()),
        alt_nicks: Some(vec!["bruce".to_owned()]),
        server: Some("irc.mozilla.org".to_owned()),
        channels: Some(vec!["#ONU".to_owned()]),
        ..Default::default()
    };

    let configs = vec![config];

    let mut reactor = IrcReactor::new().unwrap();

    loop {
        let res = configs.iter().fold(Ok(()), |acc, config| {
            acc.and(
                reactor.prepare_client_and_connect(config).and_then(|client| {
                    client.identify().and(Ok(client))
                }).and_then(|client| {
                    reactor.register_client_with_handler(client, process_msg);
                    Ok(())
                })
            )
        }).and_then(|()| reactor.run());

        match res {
            Ok(_) => break,

            Err(e) => eprintln!("{}", e),
        }
    }

    /*
        let client = IrcClient::from_config(config).unwrap();
        client.identify().unwrap();

        client.for_each_incoming(|message|{
            print!("{}", message);
            if let Command::PRIVMSG(ref target, ref msg) = message.command{
                if msg.contains("hi"){
                    client.send_privmsg(target, "Hi").unwrap();
                }
            }
        }).unwrap();
    */
}
