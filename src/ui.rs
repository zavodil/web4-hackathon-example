use crate::*;

use web4::*;
use utils::*;

#[near_bindgen]
impl Contract {
    #[allow(unused_variables)]
    // function to render the frontend using Web4 Protocol
    pub fn web4_get(&self, request: Web4Request) -> Web4Response {
        let path = request.path;

        if path == "/robots.txt" {
            return Web4Response::plain_response("User-agent: *\nDisallow:".to_string());
        }

        // submit page
        if path == "/register" {
            let (login_html, show_submit_form) = if let Some(user_account_id) = request.account_id {
                (
                    "".to_string(),
                    "block"
                )
            } else {
                (
                    format!(r#"<h3>Sign in with NEAR account to submit new app.</h3><div><a href="/web4/login?web4_contract_id={}" class="btn btn-primary">Sing in</a></div>"#, env::current_account_id()),
                    "none"
                )
            };

            return Web4Response::html_response(
                include_str!("../res/submit.html")
                    .replace("%LOGIN_BLOCK%", &login_html)
                    .replace("%SHOW_SUBMIT_FORM%", show_submit_form)
                    .replace("%CONTRACT_ID%", &env::current_account_id().to_string())
            );
        }

        // homepage
        let mut app_html = "".to_string();
        for (account_id, application_data) in self.get_applications(None, None) {
            if application_data.hidden == Some(false) {
                let mut youtube_url_text = "".to_string();

                if let Some(youtube_url) = application_data.youtube_url {
                    if !youtube_url.is_empty() {
                        youtube_url_text = format!(r#"<a href="{}">Youtube</a>&nbsp;|| "#, youtube_url);
                    }
                };

                let data = format!(r#"{}<a href="{}">Github</a>, contract: {}"#, youtube_url_text, application_data.github_url, application_data.contract_id);

                let winner_text = if let Some(reward) = application_data.reward {
                    format!(" [Prize: {} NEAR]", format_ynear(reward))
                } else {
                    "".to_string()
                };

                app_html = format!(r#"{}<tr><td>{}</td><td>{}</td><td>{}{}</td><td>{}</td></tr>"#, &app_html,
                                   application_data.description,
                                   data,
                                   account_id,
                                   winner_text,
                                   application_data.contact_data
                );
            }
        }

        Web4Response::html_response(
            include_str!("../res/example.html")
                .replace("%REWARD_POOL%", &format_ynear(self.prize_pool))
                .replace("%DEADLINE_SCRIPT%", &format_timestamp_script(self.deadline))
                .replace("%APPLICATIONS%", &app_html)
        )
    }
}

// helper to transfer timestamp in nanoseconds to human readable format using JS
fn format_timestamp_script(value: Option<Timestamp>) -> String {
    let date = if let Some(value) = value {
        format!("(new Date({}).toLocaleString(\"en-US\"))", value)
    } else {
        r#""Not set""#.to_string()
    };
    format!("document.getElementById(\"deadline\").innerText={};", date)
}
