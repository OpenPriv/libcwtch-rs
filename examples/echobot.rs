use std::thread;

use libcwtch;
use libcwtch::structs::*;
use libcwtch::CwtchLib;

fn main() {
    let bot_home = "example_cwtch_dir";
    match std::fs::remove_dir_all(&bot_home) {
        // this will always error on first run as we haven't created the directory, it's just a function to clear it on successive runs, so this error is to be ignored
        _ => (),
    }
    std::fs::create_dir_all(&bot_home).expect("Error creating bot_home directory");

    let cwtch = libcwtch::new_cwtchlib_go();
    println!("start_cwtch");
    let ret = cwtch.start_cwtch(bot_home, "");
    println!("start_cwtch returned {}", ret);

    let event_loop_handle = thread::spawn(move || {
        loop {
            let event_str = cwtch.get_appbus_event();
            println!("event: {}", event_str);

            let event: CwtchEvent =
                serde_json::from_str(&event_str).expect("Error parsing Cwtch event");
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
                    let profile = match Profile::new(
                        &event.data["Identity"],
                        &event.data["name"],
                        &event.data["picture"],
                        &event.data["ContactsJson"],
                        &event.data["ServerList"],
                    ) {
                        Ok(p) => p,
                        Err(e) => panic!("error parsing profile: {}", e),
                    };
                    print!("profile: {:?}", profile);
                }
                "NewMessageFromPeer" => {
                    let to = &event.data["ProfileOnion"];
                    let conversation_id = event.data["ConversationID"].parse::<i32>().unwrap();
                    let message: Message =
                        serde_json::from_str(&event.data["Data"]).expect("Error parsing message");

                    let response = Message { o: 1, d: message.d };
                    let response_json =
                        serde_json::to_string(&response).expect("Error parsing json response");
                    cwtch.send_message(&to, conversation_id, &response_json);
                }
                _ => eprintln!("unhandled event!"),
            };
        }
    });

    event_loop_handle.join().expect("Error running event loop");
}
