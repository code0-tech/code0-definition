use crate::analyser::core::Analyser;

mod auth;

pub async fn push(
    token: String,
    url: String,
    version_option: Option<String>,
    path: Option<String>,
) {
    let dir_path = path.unwrap_or_else(|| "./definitions".to_string());

    let version = match version_option {
        None => String::from("0.0.0"),
        Some(v) => v,
    };

    let mut analyzer = Analyser::new(dir_path.as_str());
    analyzer.report(false, true);
    todo!("Implement Sagittarius Module Service Client Endpoint!")
}
