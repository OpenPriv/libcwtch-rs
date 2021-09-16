use std::thread;

use libcwtch;
use libcwtch::structs::*;
use libcwtch::CwtchLib;

fn main() {
    let bot_home: String = "example_cwtch_dir".to_string();
    match std::fs::remove_dir_all(&bot_home) {
        Err(_e) => (),
        _ => (),
    }
    std::fs::create_dir_all(&bot_home).unwrap();

    let cwtch = libcwtch::new_cwtchlib_go();
    println!("start_cwtch");
    let ret = cwtch.start_cwtch(bot_home.as_str(), "");
    println!("start_cwtch returned {}", ret);

    let event_loop_handle = thread::spawn(move || {
        loop {
            let event_str = cwtch.get_appbus_event();
            println!("event: {}", event_str);

            let event: CwtchEvent = serde_json::from_str(&event_str).unwrap();
            match event.event_type.as_str() {
                "CwtchStarted" => {
                    println!("event CwtchStarted!");
                    println!("Creating bot");
                    cwtch.create_profile("Echobot", "be gay do crime");
                }
                "NewPeer" => {
                    println!(
                        "\n***** {} at {} *****\n",
                        event.data["name"], event.data["Identity"]
                    );

                    // process json for profile, contacts and servers...else {
                    let profile = Profile::new(
                        &event.data["Identity"],
                        &event.data["name"],
                        &event.data["picture"],
                        &event.data["ContactsJson"],
                        &event.data["ServerList"],
                    );
                    print!("profile: {:?}", profile);
                }
                "NewMessageFromPeer" => {
                    let to = event.data["ProfileOnion"].to_string();
                    let conversation = event.data["RemotePeer"].to_string();
                    let message: Message =
                        serde_json::from_str(event.data["Data"].as_str()).unwrap();

                    let response = Message { o: 1, d: message.d };
                    let response_json = serde_json::to_string(&response).unwrap();
                    cwtch.send_message(to.as_str(), conversation.as_str(), response_json.as_str());
                }
                _ => println!("unhandled event!"),
            };
        }
    });

    event_loop_handle.join().unwrap();
}
