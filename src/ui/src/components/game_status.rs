use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct GameStatusProps {
    pub is_playing: bool,
    pub is_paused: bool,
    pub is_game_over: bool,
}

#[function_component(GameStatus)]
pub fn gameStatus(props: &GameStatusProps) -> Html {
    let title: String;
    let subtitle: String;

    if props.is_playing {
        title = "Playing".to_string();
        subtitle = "Use the arrow keys (or 'WASD') to move the snake.".to_string();
    } else if props.is_paused {
        title = "Paused".to_string();
        subtitle = "Press 'space' to resume...".to_string();
    } else if props.is_game_over {
        title = "Game Over!".to_string();
        subtitle = "Press 'space' to play again!".to_string();
    } else {
        panic!("Unknown game state")
    }

    html!(
        <div id={"game-status-wrapper"}>
            <div id={"game-status-title"}>{ title }</div>
            <div id={"game-status-subtitle"}>{ subtitle }</div>
        </div>
    )
}
