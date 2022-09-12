use yew::{function_component, html, Properties};

use game::game_status::GameStatus as LogicalGameStatus;

#[derive(Properties, PartialEq)]
pub struct GameStatusProps {
    pub status: LogicalGameStatus,
}

#[function_component(GameStatus)]
pub fn gameStatus(props: &GameStatusProps) -> Html {
    let (title, subtitle) = match props.status {
        LogicalGameStatus::Playing => (
            "Playing",
            "Use the arrow keys (or 'WASD') to move the snake.",
        ),
        LogicalGameStatus::Paused => (
            "Paused",
            "Click on the game area or press 'space' to resume",
        ),
        LogicalGameStatus::GameOver => ("Game Over!", "Refresh your browser to play again."),
    };

    html!(
        <div id={"game-status-wrapper"}>
            <div id={"game-status-title"}>{ title }</div>
            <div id={"game-status-subtitle"}>{ subtitle }</div>
        </div>
    )
}
