use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::StandardFramework;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{commands, group};

// Simple struct to hold game state
struct GameState {
    is_game_running: bool, 
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            is_game_running: false,
        }
    }
}

// Simple event handler
struct Handler;

#[async_trait]
impl EventHandler for Handler {}

// Simple command group for game-related commands 
#[group]
#[commands(start, join, vote)]
struct Game;

#[tokio::main]
async fn main() {
    let token = "bot_token"; // write direnv code here 
    
    // Create new client 
    let mut client = Client::builder(token).event_handler(Handler).framework(StandardFramework::new().configure(|c| c.prefix("!")).group(&GAME_GROUP),).await.expect("Error creating client");
    
    // Set up and start the bot 
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

// Implement the start command 
#[command]
async fn start(ctx: &Context, msg: &Message) {
    let mut data = ctx.data.write().await;
    let game_state = data.get_mut::<GameState>().expect("Failed to get game state");

    if game_state.is_game_running {
        msg.reply(ctx, "You joined the game!").await.expect("Failed to reply");
        // Add logic to handle player joining
    }
    else {
        msg.reply(ctx, "No game is currently running!").await.expect("Failed to reply");
    }
}

// Implement vote command
#[command]
async fn vote(ctx: &Context, msg: &Message) {
    let mut data = ctx.data.write().await;
    let game_state = data.get_mut::<GameState>().expect("Failed to reply");

    if game_state.is_game_running {
        msg.reply(ctx, "Vote registered!").await.expect("Failed to reply")
        // Add logic to handle player voting
    }
    else {
        msg.reply(ctx, "No game is currently running!").await.expect("Failed to reply");
    }
}
