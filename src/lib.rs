//!
//! # Contents
//!   * Getting Started
//!   * Usage
//!   * Examples
//! # Importing `passport-strategies`
//!
//! Passport strategy for authenticating with Facebook, Google, Microsoft and Github using the OAuth 2.0 API. This library is a thin wrapper of [`oauth2`](https://crates.io/crates/oauth2) that simplifies the auth flow.
//! This module lets you authenticate with the above mentioned providers in your applications. By plugging into passport-strategies, (Microsoft, Google, Github and Facebook) authentication can be easily and unobtrusively integrated into any rust application or rust framework.
//! ```toml
//! passport-strategies = { version = "0.1.4" }
//! ```
//! # Usage
//! Create an Application
//! Before using passport-strategies, you must register an application with the respective provider. If you have not already done so, a new application can be created at [`Facebook`](https://developers.facebook.com), [`Google`](https://console.cloud.google.com), [`Github`](https://github.com/settings/developers), [`Microsoft`](), [`Discord`](https://discord.com/developers). Your application will be issued an app ID and app secret, which need to be provided to the strategy. You will also need to configure a redirect URI which matches the route in your application.

//! #Configure Strategy
//! The `passport-strategy` authenticates users using the desired provider account and OAuth 2.0 tokens. The `app ID(or in some cases client id)`, `redirect url` and `client secret` obtained when creating an application are supplied as requirements when creating the strategy. You do not need to provide the authorization url and token url.Unlike [`passportjs`](https://www.passportjs.org/), the strategy does not require a verify callback, which receives the access token and optional refresh token, as well as profile which contains the authenticated user's provider profile. Instead, the profile containing the access token and optional refresh token is returned to complete authentication.
//!
//! # Example (Microsoft)
//!
//! ```rust,no_run
//!  use passport_strategies::strategies::MicrosoftStrategy;
//!  use passport_strategies::basic_client::PassPortBasicClient;
//!  
//!  let mut passport = PassPortBasicClient::default();
//!   passport.using(
//!            "microsoft", // Whether it's all caps or not or just an abbreviation or any other word, it's still acceptable provided that you should use it in the passport.authenticate() function.
//!            MicrosoftStrategy::new(
//!                "<client_id>",
//!                "<client_secret>",
//!                vec!["<scope>"],
//!                "<redirect_url>",
//!                ///New field for purpose of error handling incase of user canceling the authorization or csrf token and code challenge mismatch.
//!                 "<failure_redirect>"
//!            ),
//!        );
//! ```
//!
//! # Example(Discord) with (actix-web)
//! ```rust,no_run
//! use std::sync::Arc;
//!
//! use actix_web::{
//!        http,
//!        middleware::Logger,
//!        web::{self, Data},
//!        App, HttpResponse, HttpServer,
//!    };
//!
//!    use passport_strategies::{
//!        basic_client::{PassPortBasicClient, StateCode},
//!        strategies::DiscordStrategy,
//!    };
//!
//!    use tokio::sync::RwLock;
//!
//!    pub async fn discord(passport: Data<Arc<RwLock<PassPortBasicClient>>>) -> HttpResponse {
//!        let mut auth = passport.write().await;
//!        auth.authenticate("discord");
//!        let url = auth.generate_redirect_url();
//!        HttpResponse::SeeOther()
//!            .append_header((http::header::LOCATION, url))
//!            .finish()
//!    }
//!
//!    pub async fn authenticate_discord(
//!        auth: Data<Arc<RwLock<PassPortBasicClient>>>,
//!        authstate: web::Query<StateCode>,
//!    ) -> HttpResponse {
//!        let mut auth = auth.write().await;
//!        /// The `response` is an enum. It can either be a failure_redirect or profile 
//!        match auth.get_profile(authstate.0).await {
//!            /// The profile is a json value containing the user profile, access_token and refresh_token.
//!            Ok(response) => {
//!                   match response {
//!                        /// At this point you can proceed to save the profile info in the database or use the access token or refresh token to request for more user info or some other relevant info.
//!                        PassportResponse::Profile(profile) => HttpResponse::Ok().json(profile),
//!                        /// If the user canceled the authorization process, a redirect to i.e login page would be very convinient rather 
//!                        /// than displaying some `Internal server error` just to say. It may not be exactly that kind of error, but can be inclusive of others.
//!                        PassportResponse::FailureRedirect(failure) => HttpResponse::SeeOther()
//!                        .append_header((http::header::LOCATION, failure.to_string()))
//!                        .finish()
//!                    },
//!            Err(error) => HttpResponse::BadRequest().body(error.to_string()),
//!        }
//!    }
//!    pub async fn signup_get() -> HttpResponse {
//!        let html = r#"<!DOCTYPE html>
//!        <html lang="en">
//!        <head>
//!            <meta charset="UTF-8" />
//!            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
//!            <title>Auth Demo</title>
//!            />
//!        </head>
//!            <a href="/discord">discord</a>
//!        </body>
//!        </html>
//!        "#;
//!        HttpResponse::Ok().body(html)
//!    }
//!
//!    #[tokio::main]
//!    async fn main() -> std::io::Result<()> {
//!        std::env::set_var("RUST_LOG", "debug");
//!        pretty_env_logger::init();
//!
//!        let mut passport = PassPortBasicClient::default();
//!        passport.using(
//!            "microsoft",
//!            MicrosoftStrategy::new(
//!                "<client_id>",
//!                "<client_secret>",
//!                vec!["<scope>"],
//!                "<redirect_url>",
//!                 //New field for purpose of error handling incase of user canceling the authorization or csrf token and code challenge mismatch.
//!                 "<failure_redirect>"
//!            ),
//!        );
//!        
//!        let passport_clone = Arc::new(RwLock::new(passport));
//!        HttpServer::new(move || {
//!            App::new()
//!                .wrap(Logger::default())
//!                .app_data(Data::new(passport_clone.clone()))
//!                .route("/signup", web::get().to(signup_get))
//!                .route("/discord", web::get().to(msft))
//!                .route("/<discord_redirect_endpoint>", web::get().to(authenticate_msft))
//!        })
//!        .bind("<socket_address>")? // For me, i chose port 4000 when creating Auth applications. So, it would be 127.0.0.1:4000
//!        .run()
//!        .await?;
//!        Ok(())
//!   }
//! ```


// # Basic Client
/// Contains the `PassPortBasicClient` that holds the strategies.
pub mod basic_client;

// # Strategies
/// Contains all the basic strategies  `DiscordStrategy`, `GoogleStrategy`, `MicrosoftStrategy`, `GithubStrategy` and `FacebookStrategy`.
///  Other strategies will be added later.
pub mod strategies;
