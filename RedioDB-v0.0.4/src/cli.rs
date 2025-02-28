// src/cli.rs

use clap::{Parser, Subcommand};
use std::env;

// Import gRPC client types from our generated code.
use rediodb::server::rediodb_server::rediodb_client::RediodbClient;
use rediodb::server::rediodb_server::{
    // Basic Key-Value operations
    SetRequest, KeyRequest, ExpireRequest,
    // Extended Atomic Operations
    IncrRequest, DecrRequest, AppendRequest,
    // Key Pattern Matching
    PatternRequest,
    // Transaction support
    MultiRequest, ExecRequest,
    // Data Structures: Lists
    ListPushRequest, ListPopRequest,
    // Data Structures: Sets
    SetAddRequest, SetMembersRequest,
    // Data Structures: Hashes
    HashSetRequest, HashGetRequest,
    // Pub/Sub
    PublishRequest, SubscribeRequest,
};
// Fix: Import PubSub as an external crate module rather than from crate::rediodb.
// use rediodb::pubsub::PubSub;

// For the interactive shell, import the default history type.
use rustyline::history::DefaultHistory;
use rustyline::error::ReadlineError;
use rustyline::Editor;

#[derive(Parser)]
#[command(
    name = "rediodb-cli", 
    about = "REDIODB CLI similar to redis-cli", 
    version = "0.1"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set a key-value pair with an optional TTL (in seconds)
    Set {
        key: String,
        value: String,
        #[arg(default_value_t = 0)]
        ttl: i32,
    },
    /// Get the value for a key
    Get {
        key: String,
    },
    /// Set expiration for a key (TTL in seconds)
    Expire {
        key: String,
        ttl: i32,
    },
    /// Atomically increment the numeric value of a key by an amount (default is 1)
    Incr {
        key: String,
        #[arg(default_value_t = 1)]
        amount: i32,
    },
    /// Atomically decrement the numeric value of a key by an amount (default is 1)
    Decr {
        key: String,
        #[arg(default_value_t = 1)]
        amount: i32,
    },
    /// Append a string to the current value of a key
    Append {
        key: String,
        value: String,
    },
    /// List all keys matching a pattern (e.g. "*" for all keys)
    Keys {
        pattern: String,
    },
    /// Start a transaction block (queue multiple commands)
    Multi {
        commands: Vec<String>,
    },
    /// Execute queued transactional commands
    Exec,
    /// List Push: add an element to a list
    LPush {
        key: String,
        value: String,
    },
    /// List Pop: remove and return an element from a list
    LPop {
        key: String,
    },
    /// Set Add: add a member to a set
    SAdd {
        key: String,
        member: String,
    },
    /// Set Members: retrieve all members of a set
    SMembers {
        key: String,
    },
    /// Hash Set: set a field in a hash
    HSet {
        key: String,
        field: String,
        value: String,
    },
    /// Hash Get: get a field from a hash
    HGet {
        key: String,
        field: String,
    },
    /// Publish a message to a channel
    Publish {
        channel: String,
        message: String,
    },
    /// Subscribe to messages on channels (supports multiple channels)
    Subscribe {
        channels: Vec<String>,
    },
    /// Start an interactive shell
    Interactive,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Retrieve the gRPC server address from an environment variable or default.
    let address = env::var("REDIO_ADDRESS")
        .unwrap_or_else(|_| "http://127.0.0.1:50051".to_string());

    match cli.command {
        Commands::Interactive => {
            run_interactive(&address).await?;
        }
        other => {
            execute_command(other, &address).await?;
        }
    }

    Ok(())
}

async fn execute_command(cmd: Commands, address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RediodbClient::connect(address.to_string()).await?;
    match cmd {
        Commands::Set { key, value, ttl } => {
            let req = tonic::Request::new(SetRequest { key, value, ttl });
            let resp = client.set(req).await?;
            println!("Set Response: {:?}", resp.into_inner());
        }
        Commands::Get { key } => {
            let req = tonic::Request::new(KeyRequest { key });
            let resp = client.get(req).await?;
            println!("Get Response: {:?}", resp.into_inner());
        }
        Commands::Expire { key, ttl } => {
            let req = tonic::Request::new(ExpireRequest { key, ttl });
            let resp = client.expire(req).await?;
            println!("Expire Response: {:?}", resp.into_inner());
        }
        Commands::Incr { key, amount } => {
            let req = tonic::Request::new(IncrRequest { key, amount });
            let resp = client.incr(req).await?;
            println!("Incr Response: {:?}", resp.into_inner());
        }
        Commands::Decr { key, amount } => {
            let req = tonic::Request::new(DecrRequest { key, amount });
            let resp = client.decr(req).await?;
            println!("Decr Response: {:?}", resp.into_inner());
        }
        Commands::Append { key, value } => {
            let req = tonic::Request::new(AppendRequest { key, value });
            let resp = client.append(req).await?;
            println!("Append Response: {:?}", resp.into_inner());
        }
        Commands::Keys { pattern } => {
            let req = tonic::Request::new(PatternRequest { pattern });
            let resp = client.keys(req).await?;
            println!("Keys Response: {:?}", resp.into_inner());
        }
        Commands::Multi { commands } => {
            let req = tonic::Request::new(MultiRequest { commands });
            let resp = client.multi(req).await?;
            println!("Multi Response: {:?}", resp.into_inner());
        }
        Commands::Exec => {
            let req = tonic::Request::new(ExecRequest {});
            let resp = client.exec(req).await?;
            println!("Exec Response: {:?}", resp.into_inner());
        }
        Commands::LPush { key, value } => {
            let req = tonic::Request::new(ListPushRequest { key, value });
            let resp = client.l_push(req).await?;
            println!("LPush Response: {:?}", resp.into_inner());
        }
        Commands::LPop { key } => {
            let req = tonic::Request::new(ListPopRequest { key });
            let resp = client.l_pop(req).await?;
            println!("LPop Response: {:?}", resp.into_inner());
        }
        Commands::SAdd { key, member } => {
            let req = tonic::Request::new(SetAddRequest { key, member });
            let resp = client.s_add(req).await?;
            println!("SAdd Response: {:?}", resp.into_inner());
        }
        Commands::SMembers { key } => {
            let req = tonic::Request::new(SetMembersRequest { key });
            let resp = client.s_members(req).await?;
            println!("SMembers Response: {:?}", resp.into_inner());
        }
        Commands::HSet { key, field, value } => {
            let req = tonic::Request::new(HashSetRequest { key, field, value });
            let resp = client.h_set(req).await?;
            println!("HSet Response: {:?}", resp.into_inner());
        }
        Commands::HGet { key, field } => {
            let req = tonic::Request::new(HashGetRequest { key, field });
            let resp = client.h_get(req).await?;
            println!("HGet Response: {:?}", resp.into_inner());
        }
        Commands::Publish { channel, message } => {
            let req = tonic::Request::new(PublishRequest { channel, message });
            let resp = client.publish(req).await?;
            println!("Publish Response: {:?}", resp.into_inner());
        }
        Commands::Subscribe { channels } => {
            let req = tonic::Request::new(SubscribeRequest {
                channels,
                pattern: "".into(),
            });
            let mut stream = client.subscribe(req).await?.into_inner();
            println!("Subscribed. Listening for messages (Ctrl+C to exit)...");
            while let Some(msg) = stream.message().await? {
                println!("Received message on channel '{}': {}", msg.channel, msg.message);
            }
        }
        _ => {}
    }
    Ok(())
}

async fn run_interactive(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting RedioDB interactive shell. Type 'exit' or 'quit' to leave.");
    // Fix: Supply both generic parameters for Editor
    let mut rl = Editor::<(), DefaultHistory>::new()?;
    
    loop {
        let line = rl.readline("rediodb> ");
        match line {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                if line.eq_ignore_ascii_case("exit") || line.eq_ignore_ascii_case("quit") {
                    break;
                }
                // Split the input and prepend a dummy program name so that clap can parse it.
                let args: Vec<&str> = line.split_whitespace().collect();
                let mut clap_args = vec!["rediodb-cli"];
                clap_args.extend(args);
                match Cli::try_parse_from(clap_args) {
                    Ok(cli) => {
                        if let Err(e) = execute_command(cli.command, address).await {
                            eprintln!("Error: {}", e);
                        }
                    }
                    Err(e) => eprintln!("Parse error: {}", e),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
