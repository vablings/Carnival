use axum::response::Html;
use super::components::base;

pub async fn index() -> Html<String> {
    Html(base().await.replace(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            r###"<div hx-get="http://localhost:3000/components/hero" hx-trigger="load" hx-target="#app""></div>"###,
        )
    )
}

pub async fn login_route() -> Html<String> {
    Html(base().await.replace(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            r###"<div hx-get="http://localhost:3000/components/login" hx-trigger="load" hx-target="#app""></div>"###
        )
    )
}

pub async fn register_route() -> Html<String> {
    Html(base().await.replace(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            r###"<div hx-get="http://localhost:3000/components/registration" hx-trigger="load" hx-target="#app""></div>"###
        )
    )
}

/// NOT IMPLEMENTED
pub async fn leaderboard_route() -> Html<String> {
    Html(base().await.replace(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            r###"<div hx-get="http://localhost:3000/components/leaderboard" hx-trigger="load" hx-target="#app""></div>"###
        )
    )
}

/// NOT IMPLEMENTED
pub async fn queue_route() -> Html<String> {
    Html(base().await.replace(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            r###"<div hx-get="http://localhost:3000/components/queue_table" hx-trigger="load" hx-target="#app""></div>"###
        )
    )
}

/// NOT IMPLEMENTED
pub async fn profile_route() -> Html<String> {
    Html(base().await.replace(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            r###"<div hx-get="http://localhost:3000/components/profile" hx-trigger="load" hx-target="#app""></div>"###
        )
    )
}