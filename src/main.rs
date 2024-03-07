use axum::http::Response;
use axum::response::ErrorResponse;
use axum::routing::{get, post};
use axum::{Form, Router};
use maud::{html, Markup};
use serde::Deserialize;
use tokio::process::Command;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let app = Router::new()
        .route("/", get(index))
        .route("/run", post(run));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap()
}

#[derive(Debug, Deserialize)]
struct RunCommand {
    command: String,
}

async fn index() -> Markup {
    let body = html! {
        (maud::DOCTYPE)
        html {
            head {
                title { "ShWeb" };
                script src="https://unpkg.com/htmx.org@1.9.8"
                  integrity="sha384-rgjA7mptc2ETQqXoYC3/zJvkU7K/aP44Y+z7xQuJiVnB/422P/Ak+F/AqFR7E4Wr"
                  crossorigin="anonymous" {}
            }
            body {
                div id="outputs" {}
                form
                    hx-post="/run"
                    hx-swap="beforeend"
                    hx-target=" #outputs"
                    {
                        input type="text" name="command";
                        input type="submit" value="Enter";
                    }
            }
        }
    };

    body
}

async fn run(Form(RunCommand { command }): Form<RunCommand>) -> Result<Markup, ErrorResponse> {
    let output = Command::new(command.clone()).output().await;

    let res = match output {
        Ok(out) => match String::from_utf8(out.stdout) {
            Ok(out) => out,
            Err(err) => err.to_string(),
        },
        Err(err) => err.to_string(),
    };

    let body = html! {
      div
      {
          (&command)
          br;
          (res)
      }
    };

    Ok(body)
}
