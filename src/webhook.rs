use std::thread;

pub fn send_victory_message(url: String, boss_name: String, level: u32) {
    if url.is_empty() { return; }

    thread::spawn(move || {
        let payload = serde_json::json!({
            "username": "Terminal Quest",
            "avatar_url": "https://i.imgur.com/G2x2V32.png",
            "content": format!("⚔️ **A Boss has Fallen!**\nHunter (Level {}) just defeated **{}**!", level, boss_name)
        });

        let _ = ureq::post(&url).send_json(payload);
    });
}