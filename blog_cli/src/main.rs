mod command;
mod security;

use clap::Parser;
use command::{Cli, Commands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut cli = Cli::parse();
    if !cli.server.starts_with("http://") {
        cli.server = format!("http://{}", cli.server);
    }
    let mut serv = format!("{}:{}", cli.server, cli.port);

    let mut transport = blog_client::Transport::Http(serv.clone());
    if cli.grpc {
        if cli.port == "8081" {
            serv = format!("{}:{}", cli.server, "50051");
        }
        transport = blog_client::Transport::Grpc(serv)
    }

    let mut blog = blog_client::BlogClient::new(transport).await?;
    if security::has_token() {
        blog.set_token(security::read_token()?);
    }

    let new_token: Option<String> = match &cli.command {
        Commands::Register(args) => {
            let response = blog
                .register(&args.username, &args.email, &args.password)
                .await?;
            println!("successfully");
            println!("successfully");
            Some(response.token)
        }

        Commands::Login(args) => {
            let response = blog.login(&args.username, &args.password).await?;
            Some(response.token)
        }
        Commands::Create(args) => {
            let response = blog.create_post(&args.title, &args.content).await?;

            if let Some(post) = response.post {
                print_post(post);
            };

            None
        }
        Commands::Get(args) => {
            let response = blog.get_post(args.id).await?;
            if let Some(post) = response.post {
                print_post(post);
            };
            None
        }
        Commands::Update(args) => {
            let response = blog
                .update_post(args.id, args.title.clone(), args.content.clone())
                .await?;

            if let Some(post) = response.post {
                print_post(post);
            };

            None
        }
        Commands::Delete(args) => {
            let response = blog.delete_post(args.id).await?;

            if response.success {
                println!("Post with id {} was deleted", args.id);
            }

            None
        }
        Commands::List(args) => {
            let response = blog.list_posts(args.limit, args.offset).await?;

            for post in response.post {
                print_post(post);
            }

            None
        }
    };

    if let Some(new_token) = new_token {
        security::save_token(&new_token)?;
    }

    Ok(())
}

fn print_post(post: blog_client::blog_grpc::Post) {
    println!(
        "id: {}, \ntitle: {}, \ncontent: {}",
        post.id, post.title, post.content
    );
}
